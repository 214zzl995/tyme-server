use std::sync::Arc;
use std::time::Duration;

use futures::executor::block_on;
use mqtt::ConnectOptionsBuilder;
use paho_mqtt::AsyncClient;
use paho_mqtt::{self as mqtt};
use parking_lot::Mutex;

use crate::config::SYSCONIFG;
use crate::message::Message;

const QOS: &[i32] = &[1, 1];

lazy_static! {
    pub static ref CLINT: Arc<Mutex<AsyncClient>> =
        Arc::new(Mutex::new(get_clint().expect("Clint Error")));
}

fn get_clint() -> anyhow::Result<AsyncClient> {
    let protocol = if SYSCONIFG.mqtt_config.ssl.enable {
        "mqtts://"
    } else {
        "mqtt://"
    };

    let host = format!(
        "{}{}:{}",
        protocol, SYSCONIFG.mqtt_config.broker, SYSCONIFG.mqtt_config.port
    );

    println!("Connecting to the MQTT server at '{}'...", host);

    let trust_store = if let Some(trust_store) = &SYSCONIFG.mqtt_config.ssl.trust_store {
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
        .client_id(SYSCONIFG.mqtt_config.client_id.clone())
        .finalize();

    let ssl_opts = mqtt::SslOptionsBuilder::new()
        .trust_store(trust_store)?
        .finalize();

    let cli = mqtt::AsyncClient::new(create_opts)?;

    if let Err(err) = block_on(async {
        let mut conn_opts = ConnectOptionsBuilder::with_mqtt_version(SYSCONIFG.mqtt_config.version);
        let conn_opts = conn_opts
            .ssl_options(ssl_opts)
            .clean_start(false)
            .properties(mqtt::properties![mqtt::PropertyCode::SessionExpiryInterval => 3600]);

        if let Some(keep_alive_interval) = SYSCONIFG.mqtt_config.keep_alive_interval {
            conn_opts.keep_alive_interval(Duration::from_secs(keep_alive_interval));
        }

        if SYSCONIFG.mqtt_config.auth.enable {
            if let (Some(user_name), Some(password)) = (
                &SYSCONIFG.mqtt_config.auth.user_name,
                &SYSCONIFG.mqtt_config.auth.password,
            ) {
                conn_opts.user_name(user_name).password(password);
            }
        }

        if let Some(lwt) = &SYSCONIFG.mqtt_config.lwt {
            conn_opts.will_message(lwt.to_mqtt()?);
        };

        let conn_opts = conn_opts.finalize();

        let rsp = cli.connect(conn_opts).await?;

        if let Some(conn_rsp) = rsp.connect_response() {
            println!(
                "Connected to: '{}' with MQTT version {}",
                conn_rsp.server_uri, conn_rsp.mqtt_version
            );

            if conn_rsp.session_present {
                println!("Client session already present on broker.");
            } else {
                // Register subscriptions on the server, using Subscription ID's.
                println!(
                    r#"Subscribing to topics [{}]..."#,
                    SYSCONIFG.mqtt_config.topics.join(", ")
                );
                let sub_opts = vec![
                    mqtt::SubscribeOptions::with_retain_as_published();
                    SYSCONIFG.mqtt_config.topics.len()
                ];
                cli.subscribe_many_with_options(
                    &SYSCONIFG.mqtt_config.topics,
                    QOS,
                    &sub_opts,
                    None,
                )
                .await?;
            }
        }

        Ok::<(), anyhow::Error>(())
    }) {
        return Err(anyhow::anyhow!("Error connecting: {:?}", err));
    }

    Ok(cli)
}

pub fn diable_connect() {
    let clint = CLINT.lock();
    clint.stop_consuming();
    if clint.is_connected() {
        clint.disconnect(None);
    }
}

pub async fn publish(msg: Message) -> anyhow::Result<()> {
    let clint = CLINT.lock().clone();
    let msg = msg.to_mqtt()?;
    clint.publish(msg).await?;
    Ok(())
}
