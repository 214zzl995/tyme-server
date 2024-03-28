use std::sync::Arc;
use std::time::Duration;

use futures::executor::block_on;
use log::{error, info};
use mqtt::ConnectOptionsBuilder;
use paho_mqtt::AsyncClient;
use paho_mqtt::{self as mqtt};
use parking_lot::Mutex;

use crate::header::Header;
use crate::message::SendMessage;
use crate::{task_manger, tyme_config};

const QOS: &[i32] = &[1, 1];

lazy_static! {
    pub static ref CLINT: Arc<Mutex<AsyncClient>> = Arc::new(Mutex::new({
        match get_clint() {
            Ok(clint) => clint,
            Err(err) => {
                error!("Error creating the client: {}", err);
                std::process::exit(1);
            }
        }
    }));
}

fn get_clint() -> anyhow::Result<AsyncClient> {
    let config = tyme_config.lock().clone();
    let protocol = if config.mqtt_config.ssl.enable {
        "mqtts://"
    } else {
        "mqtt://"
    };

    let host = format!(
        "{}{}:{}",
        protocol, config.mqtt_config.broker, config.mqtt_config.port
    );

    info!("Connecting to the MQTT server at '{}'...", host);

    let trust_store = if let Some(trust_store) = &config.mqtt_config.ssl.trust_store {
        if !trust_store.exists() {
            return Err(anyhow::anyhow!(
                "The trust store file does not exist: {:?}",
                trust_store
            ));
        }
        trust_store
    } else {
        return Err(anyhow::anyhow!("The trust store connfig is none"));
    };
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(config.get_clint_name())
        .finalize();

    let ssl_opts = mqtt::SslOptionsBuilder::new()
        .trust_store(trust_store)?
        .finalize();

    let cli = mqtt::AsyncClient::new(create_opts)?;

    block_on(async {
        let mut conn_opts = ConnectOptionsBuilder::with_mqtt_version(config.mqtt_config.version);
        let conn_opts = conn_opts
            .ssl_options(ssl_opts)
            .clean_start(true)
            .properties(mqtt::properties![mqtt::PropertyCode::SessionExpiryInterval => 3600]);

        if let Some(keep_alive_interval) = config.mqtt_config.keep_alive_interval {
            conn_opts.keep_alive_interval(Duration::from_secs(keep_alive_interval));
        }

        if config.mqtt_config.auth.enable {
            if let (Some(user_name), Some(password)) = (
                &config.mqtt_config.auth.username,
                &config.mqtt_config.auth.password,
            ) {
                conn_opts.user_name(user_name).password(password);
            }
        }

        let lwt_msg = SendMessage {
            topic: "system/lwt".to_string(),
            qos: 1,
            retain: Some(true),
            receiver: None,
            ephemeral: true,
            message_type: String::from("text/markdown; charset=UTF-8"),
            raw: String::new(),
        };

        conn_opts.will_message(lwt_msg.to_mqtt()?);

        let conn_opts = conn_opts.finalize();

        let rsp = cli.connect(conn_opts).await?;

        if let Some(conn_rsp) = rsp.connect_response() {
            info!(
                "Connected to: '{}' with MQTT version {}",
                conn_rsp.server_uri, conn_rsp.mqtt_version
            );

            if conn_rsp.session_present {
                info!("Client session already present on broker.");
                // Will not resubscribe when kicked out by broker
            } else {
                // Register subscriptions on the server, using Subscription ID's.
                let topics = crate::headers.lock().clone();
                info!(
                    r#"Subscribing to topics [{}]..."#,
                    topics
                        .clone()
                        .into_iter()
                        .map(|x| format!("{{topic:{:?},qos:{}}}", x.topic, x.qos))
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                let sub_opts =
                    vec![mqtt::SubscribeOptions::with_retain_as_published(); topics.len()];

                let qos = topics
                    .clone()
                    .into_iter()
                    .map(|x| x.qos)
                    .collect::<Vec<i32>>();

                let topics = topics.into_iter().map(|x| x.topic).collect::<Vec<String>>();

                cli.subscribe_many_with_options(&topics, &qos, &sub_opts, None)
                    .await?;
            }
        }
        task_manger.lock().start().await?;
        Ok::<(), anyhow::Error>(())
    })?;

    Ok(cli)
}

pub fn diable_connect() {
    let clint = CLINT.lock();
    clint.stop_consuming();
    clint.stop_stream();

    if clint.is_connected() {
        clint.disconnect(None);
    }
}

pub async fn publish(msg: SendMessage) -> anyhow::Result<()> {
    let clint = CLINT.lock().clone();
    let msg = msg.to_mqtt()?;
    clint.publish(msg).await?;
    Ok(())
}

/// Subscribe to a topic Temporary not stored
pub async fn subscribe_topic(topics: Vec<Header>) -> anyhow::Result<()> {
    {
        let mut loc_topics = crate::headers.lock().clone();
        loc_topics.extend(topics.clone());
    };

    let sub_opts = vec![mqtt::SubscribeOptions::with_retain_as_published(); topics.len()];

    let topics = topics
        .clone()
        .into_iter()
        .map(|x| x.topic)
        .collect::<Vec<String>>();

    let clint = CLINT.lock().clone();
    clint
        .subscribe_many_with_options(&topics, QOS, &sub_opts, None)
        .await?;

    Ok(())
}
