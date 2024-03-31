use std::time::Duration;

use anyhow::Context;
use futures::StreamExt;
use log::{error, info};
use mqtt::AsyncReceiver;
use tokio::sync::{broadcast, mpsc::UnboundedReceiver};

use paho_mqtt::{self as mqtt, AsyncClient};

use crate::{
    config::TymeConfig,
    header::Header,
    message::{RecMessage, SendMessage},
    tyme_config,
};

pub async fn run_mqtt_clint(
    mut send_msg_rx: UnboundedReceiver<SendMessage>,
    rec_msg_tx: broadcast::Sender<(Header, RecMessage)>,
) -> anyhow::Result<()> {
    let config = tyme_config.lock().clone();
    let mut clint = get_mqtt_clint(&config)?;

    let sub_clint = clint.clone();
    let strm = clint.get_stream(None);
    tokio::spawn(subscribe(strm, sub_clint, rec_msg_tx));

    let conn_opts = get_conn_option(&config)?;
    clint.connect(conn_opts).await?;

    while let Some(send_msg) = send_msg_rx.recv().await {
        let msg = send_msg.to_mqtt()?;
        clint.publish(msg).await?;
    }

    Ok(())
}

fn get_mqtt_clint(config: &TymeConfig) -> anyhow::Result<AsyncClient> {
    let host = if config.mqtt_config.ssl.enable {
        format!(
            "mqtts://{}:{}",
            config.mqtt_config.broker, config.mqtt_config.port
        )
    } else {
        format!(
            "mqtt://{}:{}",
            config.mqtt_config.broker, config.mqtt_config.port
        )
    };

    log::info!("Connecting to the MQTT server at '{}'...", host);

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(config.get_clint_name())
        .finalize();

    let clint = mqtt::AsyncClient::new(create_opts)?;

    Ok(clint)
}

fn get_conn_option(config: &TymeConfig) -> anyhow::Result<mqtt::ConnectOptions> {
    let mut conn_opts = mqtt::ConnectOptionsBuilder::new();

    conn_opts.keep_alive_interval(Duration::from_secs(
        config.mqtt_config.keep_alive_interval.unwrap_or(60),
    ));

    if config.mqtt_config.ssl.enable {
        let ssl_opts = get_ssl_options(config)?;
        conn_opts.ssl_options(ssl_opts);
    }

    if config.mqtt_config.auth.enable {
        let username = config
            .mqtt_config
            .auth
            .username
            .clone()
            .context("The username config is none")?;
        let password = config
            .mqtt_config
            .auth
            .password
            .clone()
            .context("The password config is none")?;

        conn_opts.user_name(username).password(password);
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

    conn_opts
        .clean_start(true)
        .properties(mqtt::properties![mqtt::PropertyCode::SessionExpiryInterval => 3600]);

    conn_opts.ssl_options(get_ssl_options(config)?);

    Ok(conn_opts.finalize())
}

fn get_ssl_options(config: &TymeConfig) -> anyhow::Result<mqtt::SslOptions> {
    let mut ssl_opts = mqtt::SslOptionsBuilder::new();

    let trust_store = config
        .mqtt_config
        .ssl
        .trust_store
        .clone()
        .context("The trust store config is none")?;

    let trust_store = crate::start_param
        .word_dir
        .clone()
        .join("ssl")
        .join(trust_store);

    if !trust_store.exists() {
        return Err(anyhow::anyhow!(
            "The trust store file does not exist: {:?}",
            trust_store
        ));
    };

    ssl_opts.trust_store(trust_store)?.finalize();

    Ok(ssl_opts.finalize())
}

async fn subscribe(
    mut strm: AsyncReceiver<Option<mqtt::Message>>,
    clint: mqtt::AsyncClient,
    rec_msg_tx: broadcast::Sender<(Header, RecMessage)>,
) {
    while let Some(msg_opt) = strm.next().await {
        if let Some(msg) = msg_opt {
            info!(
                "{} <<< [{:02}] ({}) {:?} : {:?}",
                if msg.retained() { "(R)" } else { "" },
                msg.qos(),
                msg.topic(),
                msg.payload_str(),
                msg.properties()
            );

            for prop in msg.properties().clone().user_iter() {
                info!("Property: {:?}", prop);
            }
            let ephemeral =
                msg.properties().find_user_property("ephemeral") == Some("true".to_string());

            match RecMessage::try_from(&msg) {
                Ok(mut rec_msg) => {
                    if let Err(err) = rec_msg.to_html() {
                        error!("Error converting message to html: {}", err);
                    } else {
                        match rec_msg.get_header() {
                            Some(header) => {
                                if let Err(err) = rec_msg_tx.send((header.clone(), rec_msg.clone()))
                                {
                                    error!("Error sending message: {}", err);
                                };
                                tokio::spawn(async move {
                                    if !ephemeral {
                                        if let Err(err) = rec_msg.insert(&header.id).await {
                                            error!("Error inserting message: {}", err);
                                        };
                                    }
                                });
                            }
                            None => {
                                error!("No header found for message: {:?}", rec_msg);
                            }
                        };
                    };
                }
                Err(err) => {
                    error!("Error converting message: {}", err);
                }
            }
        } else {
            info!("Lost connection. Attempting reconnect.");
            while let Err(err) = clint.reconnect().await {
                error!("Error reconnecting: {}", err);
                tokio::time::sleep(Duration::from_millis(1000)).await;
            }
            info!("Reconnected.");
        }
    }
}
