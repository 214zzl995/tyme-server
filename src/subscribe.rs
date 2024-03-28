use log::{error, info};
use paho_mqtt::{self as mqtt};
use std::time::Duration;

use futures::StreamExt;

use crate::{clint::CLINT, message::RecMessage};

pub async fn subscribe() {
    let mut clint = CLINT.lock().clone();
    let mut strm = clint.get_stream(25);

    while let Some(msg_opt) = strm.next().await {
        if let Some(msg) = msg_opt {
            if msg.retained() {
                print!("(R) ");
            }
            info!(
                "<<< [{:02}] ({}) {:?} : {:?}",
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
                                tokio::spawn(async move {
                                    crate::web_console::ws_send_all(&header, &rec_msg).await;
                                    if !ephemeral {
                                        if let Err(err) = rec_msg.insert(&header).await {
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

// Create a set of poperties with a single Subscription ID
fn _sub_id(id: i32) -> mqtt::Properties {
    mqtt::properties![
        mqtt::PropertyCode::SubscriptionIdentifier => id
    ]
}
