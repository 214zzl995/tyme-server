use paho_mqtt::{self as mqtt};
use std::time::Duration;

use futures::StreamExt;

use crate::{clint::CLINT, message::Message};

pub async fn subscribe() {
    let mut clint = CLINT.lock().clone();
    let mut strm = clint.get_stream(25);

    while let Some(msg_opt) = strm.next().await {
        if let Some(msg) = msg_opt {
            if msg.retained() {
                print!("(R) ");
            }
            println!(
                "<<< [{:02}] ({}) {:?} : {:?}",
                msg.qos(),
                msg.topic(),
                msg.payload_str(),
                msg.properties()
            );

            if let Ok(mut msg) = Message::try_from(msg) {
                msg.to_html();
                tokio::spawn(async move {
                    crate::web_console::ws_send_all(&msg).await;
                    crate::r_db::insert_msg(&msg).unwrap();
                });
            } else {
                eprintln!("Error converting message");
            }
        } else {
            println!("Lost connection. Attempting reconnect.");
            while let Err(err) = clint.reconnect().await {
                println!("Error reconnecting: {}", err);
                tokio::time::sleep(Duration::from_millis(1000)).await;
            }
            println!("Reconnected.");
        }
    }
}

// Create a set of poperties with a single Subscription ID
fn _sub_id(id: i32) -> mqtt::Properties {
    mqtt::properties![
        mqtt::PropertyCode::SubscriptionIdentifier => id
    ]
}
