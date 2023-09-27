use futures::{executor::block_on, stream::StreamExt};
use paho_mqtt::{self as mqtt, MQTT_VERSION_5};
use std::{env, process, time::Duration};

use crate::ssl_publish;

// The topics to which we subscribe.
const TOPICS: &[&str] = &["test"];
const QOS: &[i32] = &[1, 1];

pub fn subscribe() -> mqtt::Result<()> {

    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "mqtts://k37bbe35.ala.cn-hangzhou.emqxsl.cn:8883".to_string());

    println!("Connecting to the MQTT server at '{}'...", host);

    const TRUST_STORE: &str = "emqxsl-ca.crt";

    let mut trust_store = env::current_dir()?;
    trust_store.push("ssl");
    trust_store.push(TRUST_STORE);

    if !trust_store.exists() {
        println!("The trust store file does not exist: {:?}", trust_store);
        println!("  Get a copy from \"paho.mqtt.c/test/ssl/{}\"", TRUST_STORE);
        process::exit(1);
    }

    // Create the client. Use an ID for a persistent session.
    // A real system should try harder to use a unique ID.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id("rust_async_sub_v5")
        .finalize();

    // Create the client connection
    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    let ssl_opts = mqtt::SslOptionsBuilder::new()
        .trust_store(trust_store)?
        .finalize();

    if let Err(err) = block_on(async {
        // Get message stream before connecting.
        let mut strm = cli.get_stream(25);

        // Define the set of options for the connection
        let lwt = mqtt::Message::new(
            "LWT",
            "[LWT] Async subscriber v5 lost connection",
            mqtt::QOS_1,
        );

        // Connect with MQTT v5 and a persistent server session (no clean start).
        // For a persistent v5 session, we must set the Session Expiry Interval
        // on the server. Here we set that requests will persist for an hour
        // (3600sec) if the service disconnects or restarts.
        let conn_opts = mqtt::ConnectOptionsBuilder::with_mqtt_version(MQTT_VERSION_5)
            .ssl_options(ssl_opts)
            .clean_start(false)
            .user_name("leri")
            .password("R7ddsQxAGchQPQB")
            .properties(mqtt::properties![mqtt::PropertyCode::SessionExpiryInterval => 3600])
            .will_message(lwt)
            .finalize();

        // Make the connection to the broker
        cli.connect(conn_opts).await?;

        println!("Subscribing to topics: {:?}", TOPICS);
        let sub_opts = vec![mqtt::SubscribeOptions::with_retain_as_published(); TOPICS.len()];
        cli.subscribe_many_with_options(TOPICS, QOS, &sub_opts, None)
            .await?;

        // Just loop on incoming messages.
        println!("Waiting for messages...");

        // Note that we're not providing a way to cleanly shut down and
        // disconnect. Therefore, when you kill this app (with a ^C or
        // whatever) the server will get an unexpected drop and then
        // should emit the LWT message.

        while let Some(msg_opt) = strm.next().await {
            if let Some(msg) = msg_opt {
                if msg.retained() {
                    print!("(R) ");
                }
                println!("{}", msg);
                //回复消息
                ssl_publish::publish().await?;
            } else {
                // A "None" means we were disconnected. Try to reconnect...
                println!("Lost connection. Attempting reconnect.");
                while let Err(err) = cli.reconnect().await {
                    println!("Error reconnecting: {}", err);
                    // For tokio use: tokio::time::delay_for()
                    async_std::task::sleep(Duration::from_millis(1000)).await;
                }
            }
        }

        // Explicit return type for the async block
        Ok::<(), mqtt::Error>(())
    }) {
        eprintln!("{}", err);
    }
    Ok(())
}
