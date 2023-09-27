use paho_mqtt as mqtt;
use parking_lot::Mutex;
use std::{env, process};

lazy_static! {
    pub static ref KEY: Mutex<bool> = Mutex::new(false);
    pub static ref TRUST: Mutex<bool> = Mutex::new(false);
}

pub async fn publish() -> mqtt::Result<()> {
    const TRUST_STORE: &str = "emqxsl-ca.crt";

    let mut trust_store = env::current_dir()?;
    trust_store.push("ssl");
    trust_store.push(TRUST_STORE);

    if !trust_store.exists() {
        println!("The trust store file does not exist: {:?}", trust_store);
        println!("  Get a copy from \"paho.mqtt.c/test/ssl/{}\"", TRUST_STORE);
        process::exit(1);
    }

    // Let the user override the host, but note the "ssl://" protocol.
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "mqtts://k37bbe35.ala.cn-hangzhou.emqxsl.cn:8883".to_string());

    println!("Connecting to host: '{}'", host);

    // Run the client in an async block

    // Create a client & define connect options
    let cli = mqtt::CreateOptionsBuilder::new()
        .server_uri(&host)
        .client_id("ssl_publish_rs")
        .max_buffered_messages(100)
        .create_client()?;

    let ssl_opts = mqtt::SslOptionsBuilder::new()
        .trust_store(trust_store)?
        .finalize();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .ssl_options(ssl_opts)
        .user_name("leri")
        .password("R7ddsQxAGchQPQB")
        .finalize();

    cli.connect(conn_opts).await?;

    let msg = mqtt::MessageBuilder::new()
        .topic("hello")
        .payload("Hello secure world!")
        .qos(1)
        .finalize();

    cli.publish(msg).await?;
    cli.disconnect(None).await?;

    Ok(())
}
