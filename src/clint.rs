use std::sync::Arc;
use std::time::Duration;

use futures::executor::block_on;
use log::info;
use mqtt::ConnectOptionsBuilder;
use paho_mqtt::AsyncClient;
use paho_mqtt::{self as mqtt};
use parking_lot::Mutex;

use crate::config::SYSCONIFG;
use crate::message::{Message, MessageContent, Topic};
use crate::task_manger;

const QOS: &[i32] = &[1, 1];

lazy_static! {
    pub static ref CLINT: Arc<Mutex<AsyncClient>> =
        Arc::new(Mutex::new(get_clint().expect("Clint Error")));
    pub static ref TOPICS: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(
        SYSCONIFG.lock().clone().mqtt_config.topics.clone()
    ));
}

fn get_clint() -> anyhow::Result<AsyncClient> {
    let config = SYSCONIFG.lock().clone();
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
                &config.mqtt_config.auth.user_name,
                &config.mqtt_config.auth.password,
            ) {
                conn_opts.user_name(user_name).password(password);
            }
        }

        if let Some(lwt) = &config.mqtt_config.lwt {
            let topic = Topic::try_from("system/lwt")?;
            let lwt_msg = Message {
                id: None,
                topic,
                qos: mqtt::QOS_1,
                timestamp: None,
                mine: None,
                content: MessageContent {
                    message_type: String::from("text/markdown; charset=UTF-8"),
                    raw: lwt.clone(),
                    html: None,
                },
                sender: Some(config.get_clint_name()),
                receiver: None,
                retain: Some(false),
            };
            conn_opts.will_message(lwt_msg.to_mqtt()?);
        };

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
                let mut topics = config.mqtt_config.topics.clone();
                topics.push("system/#".to_string());
                info!(r#"Subscribing to topics [{}]..."#, topics.join(", "));
                let sub_opts =
                    vec![mqtt::SubscribeOptions::with_retain_as_published(); topics.len()];

                let qos = vec![mqtt::QOS_1; config.mqtt_config.topics.len()];
                cli.subscribe_many_with_options(&topics, &qos, &sub_opts, None)
                    .await?;
            }
        }

        Ok::<(), anyhow::Error>(())
    })?;


    task_manger.lock().start();
    
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

pub async fn publish(msg: Message) -> anyhow::Result<()> {
    let clint = CLINT.lock().clone();
    let msg = msg.to_mqtt()?;
    clint.publish(msg).await?;
    Ok(())
}

/// Subscribe to a topic Temporary not stored
pub async fn subscribe_topic(topics: Vec<String>) -> anyhow::Result<()> {
    {
        let mut loc_topics = TOPICS.lock();
        loc_topics.extend(topics.clone());
    };

    let sub_opts = vec![mqtt::SubscribeOptions::with_retain_as_published(); topics.len()];

    let clint = CLINT.lock().clone();
    clint
        .subscribe_many_with_options(&topics, QOS, &sub_opts, None)
        .await?;

    Ok(())
}

#[test]
fn scrcpy() {
    let lua = mlua::Lua::new();

    let globals = lua.globals();

    let package_path = globals
        .get::<_, mlua::Table>("package")
        .unwrap()
        .get::<_, String>("path")
        .unwrap();

    let package_path = format!("{};./scrcpy/?.lua", package_path);

    globals
        .get::<_, mlua::Table>("package")
        .unwrap()
        .set("path", package_path)
        .unwrap();

    // 加载一个 Lua 脚本文件
    let script_path = "scrcpy/os.lua";
    let script_content = std::fs::read_to_string(script_path).expect("Failed to read script file");

    let script = lua.load(script_content);

    // 执行脚本
    // let _ = script.exec().unwrap();
    // let _ = script.call::<_, mlua::Value>(()).unwrap();
    // let _ = script.eval::<mlua::Value>().unwrap();

    let os_info = script.eval::<mlua::Value>().unwrap();

    let os_info_json = serde_json::to_string_pretty(&os_info).unwrap();

    println!("{}", os_info_json);
}
