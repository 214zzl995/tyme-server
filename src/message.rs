use std::time::SystemTime;

use bincode::{Decode, Encode};
use paho_mqtt::{self as mqtt};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Decode, Encode)]
pub struct Message {
    pub id: Option<String>,
    pub topic: String,
    pub qos: i32,
    pub mine: Option<bool>,
    pub timestamp: Option<u128>,
    pub content: MessageContent,
}

#[derive(Deserialize, Serialize, Clone, Decode, Encode)]
pub struct MessageContent {
    #[serde(rename = "type")]
    pub message_type: MessageType,
    pub raw: String,
    pub html: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Decode, Encode)]
pub enum MessageType {
    MarkDown,
    Json,
    Raw,
}

impl Message {
    pub fn to_mqtt(&self) -> anyhow::Result<mqtt::Message> {
        let payload = serde_json::to_string(&self.content)?;
        Ok(mqtt::Message::new(self.topic.clone(), payload, self.qos))
    }

    pub fn from_mqtt(msg: mqtt::Message) -> anyhow::Result<Self> {
        let content = serde_json::from_str::<MessageContent>(&msg.payload_str())?;

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("get millis error");

        Ok(Message {
            id: Some(nanoid::nanoid!()),
            topic: msg.topic().to_owned(),
            qos: msg.qos(),
            timestamp: Some(now.as_millis()),
            mine: Some(false),
            content,
        })
    }

    pub fn to_html(&mut self) {
        if self.content.message_type.eq(&MessageType::MarkDown) {
            let html: String =
                markdown::to_html_with_options(&self.content.raw, &markdown::Options::gfm())
                    .unwrap();
            self.content.html = Some(html);
        } else if self.content.message_type.eq(&MessageType::Json) {
            let html: String = markdown::to_html_with_options(
                &format!("```json \n{}\n```", &self.content.raw),
                &markdown::Options::gfm(),
            )
            .unwrap();
            self.content.html = Some(html);
        } else {
            self.content.html = Some(self.content.raw.clone());
        }
    }
}
