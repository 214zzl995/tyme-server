use std::process;
use std::sync::Arc;

use futures::executor::block_on;
use mqtt::ConnectOptionsBuilder;
use paho_mqtt::AsyncClient;
use paho_mqtt::{self as mqtt};
use parking_lot::Mutex;

use crate::config::SYSCONIFG;

const QOS: &[i32] = &[1, 1];

lazy_static! {
    pub static ref CLINT: Arc<Mutex<AsyncClient>> = Arc::new(Mutex::new(get_clint().unwrap()));
}

fn get_clint() -> anyhow::Result<AsyncClient> {
    let protocol = if SYSCONIFG.ssl.enable {
        "mqtts://"
    } else {
        "mqtt://"
    };

    let host = format!("{}{}:{}", protocol, SYSCONIFG.broker, SYSCONIFG.port);

    println!("Connecting to the MQTT server at '{}'...", host);

    let trust_store = if let Some(trust_store) = &SYSCONIFG.ssl.trust_store {
        if !trust_store.exists() {
            println!("The trust store file does not exist: {:?}", trust_store);
            process::exit(1);
        }
        trust_store
    } else {
        println!("trust store is not configured in the configuration file");
        process::exit(1);
    };
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id("rust_async_sub_v5")
        .finalize();

    let ssl_opts = mqtt::SslOptionsBuilder::new()
        .trust_store(trust_store)?
        .finalize();

    let cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    if let Err(err) = block_on(async {
        let mut conn_opts = ConnectOptionsBuilder::with_mqtt_version(SYSCONIFG.version);
        let conn_opts = conn_opts
            .ssl_options(ssl_opts)
            .clean_start(false)
            .properties(mqtt::properties![mqtt::PropertyCode::SessionExpiryInterval => 3600]);

        if SYSCONIFG.auth.enable {
            if let (Some(user_name), Some(password)) =
                (&SYSCONIFG.auth.user_name, &SYSCONIFG.auth.password)
            {
                conn_opts.user_name(user_name).password(password);
            }
        }

        if let Some(lwt) = &SYSCONIFG.lwt {
            conn_opts.will_message(lwt.to_mqtt()?);
        };

        let conn_opts = conn_opts.finalize();

        cli.connect(conn_opts).await?;

        let sub_opts = vec![mqtt::SubscribeOptions::with_retain_as_published(); SYSCONIFG.topics.len()];
        cli.subscribe_many_with_options(&SYSCONIFG.topics, QOS, &sub_opts, None)
            .await?;

        Ok::<(), anyhow::Error>(())
    }) {
        eprintln!("{}", err);
    }

    Ok(cli)
}
