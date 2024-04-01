use anyhow::Context;
use chrono::{DateTime, Local};
use paho_mqtt::{self as mqtt};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::header::Header;

#[derive(Deserialize, Serialize, Clone, Debug, FromRow)]
pub struct RecMessage {
    pub id: String,
    pub topic: String,
    pub qos: i32,
    pub retain: bool,
    pub mine: bool,
    pub timestamp: DateTime<Local>,
    #[sqlx(flatten)]
    pub content: MessageContent,
    pub sender: Option<String>,
    pub receiver: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SendMessage {
    pub topic: String,
    pub qos: i32,
    pub retain: Option<bool>,
    pub receiver: Option<String>,
    pub ephemeral: bool,
    #[serde(rename = "type")]
    pub message_type: String,
    pub raw: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, sqlx::FromRow)]
pub struct MessageContent {
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub message_type: String,
    pub raw: String,
    pub html: Option<String>,
}

impl SendMessage {
    pub fn to_mqtt(&self) -> anyhow::Result<mqtt::Message> {
        let mut props = mqtt::properties::Properties::new();

        props.push_string_pair(
            mqtt::PropertyCode::UserProperty,
            "sender",
            &crate::config::TYME_CONFIG.lock().clone().get_clint_name(),
        )?;

        if let Some(receiver) = &self.receiver {
            props.push_string_pair(
                mqtt::PropertyCode::UserProperty,
                "receiver",
                receiver.as_str(),
            )?;
        }

        props.push_string(
            mqtt::PropertyCode::ContentType,
            self.message_type.clone().as_str(),
        )?;

        let msg = mqtt::MessageBuilder::new()
            .topic(self.topic.clone())
            .payload(self.raw.clone())
            .properties(props)
            .qos(self.qos)
            .retained(self.retain.unwrap_or(false))
            .finalize();

        Ok(msg)
    }
}

impl RecMessage {
    pub fn to_html(&mut self) -> anyhow::Result<()> {
        let msg_type = self.content.message_type.clone();
        let msg_type: mime::Mime = msg_type.parse().context("Unable to parse mime type")?;

        if msg_type.essence_str().eq("text/markdown") {
            let html: String =
                markdown::to_html_with_options(&self.content.raw, &markdown::Options::gfm())
                    .unwrap();
            self.content.html = Some(html);
        } else if msg_type.essence_str().eq("application/json") {
            let html: String = markdown::to_html_with_options(
                &format!("```json \n{}\n```", &self.content.raw),
                &markdown::Options::gfm(),
            )
            .unwrap();
            self.content.html = Some(html);
        } else {
            Err(anyhow::anyhow!("Unsupported message type"))?;
        };
        Ok(())
    }

    pub async fn get_header(&self) -> anyhow::Result<Option<Header>> {
        Ok(Header::get_all_header()
            .await?
            .into_iter()
            .find(|pattern| pattern.mqtt_topic_matches(self.topic.as_ref())))
    }
}

impl TryFrom<&mqtt::Message> for RecMessage {
    type Error = anyhow::Error;

    fn try_from(msg: &mqtt::Message) -> Result<Self, Self::Error> {
        let topic = msg.topic().to_string();

        let qos = msg.qos();

        let retain = msg.retained();

        let sender = msg.properties().find_user_property("sender");

        let mine = sender
            .clone()
            .context("Unable to find publish property")?
            .eq(&crate::config::TYME_CONFIG.lock().clone().get_clint_name());

        let receiver = msg.properties().find_user_property("receiver");

        let message_type = msg
            .properties()
            .get_string(mqtt::PropertyCode::ContentType)
            .context("Unable to find content type property")?;

        let content = MessageContent {
            message_type,
            raw: msg.payload_str().to_string(),
            html: None,
        };

        Ok(RecMessage {
            id: nanoid::nanoid!(),
            topic,
            qos,
            retain,
            mine,
            timestamp: Local::now().into(),
            content,
            sender,
            receiver,
        })
    }
}
