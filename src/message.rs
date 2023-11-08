use paho_mqtt::{self as mqtt};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Message {
    pub topic: String,
    pub qos: i32,
    pub mine: Option<bool>,
    pub content: MessageContent,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MessageContent {
    #[serde(rename = "type")]
    pub message_type: MessageType,
    pub raw: String,
    pub html: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum MessageType {
    MarkDown,
    Json,
    Raw
}

impl Message {
    pub fn to_mqtt(&self) -> anyhow::Result<mqtt::Message> {
        let payload = serde_json::to_string(&self.content)?;
        Ok(mqtt::Message::new(self.topic.clone(), payload, self.qos))
    }
}


