use std::time::SystemTime;

use anyhow::{Context, Ok};
use paho_mqtt::{self as mqtt};
use serde::{Deserialize, Serialize};

use crate::config::Header;

// Message 是否可以做两个 当前的Header 完全可以在用完直接扔掉 没有必要 反而增加了代码的冗余 年前改掉
// 一个是接收过程中的Message 需要实现 mqtt消息转换为Message
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RecMessage {
    pub id: String,
    pub topic: String,
    pub qos: i32,
    pub retain: bool,
    pub mine: bool,
    pub timestamp: u128,
    pub content: MessageContent,
    pub sender: Option<String>,
    pub receiver: Option<String>,
}
// 一个是发送流程中的 Message 需要实现 Message转换为mqtt消息
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PushMessage {
    pub topic: String,
    pub qos: i32,
    pub retain: Option<bool>,
    pub content: MessageContent,
    pub sender: Option<String>,
    pub receiver: Option<String>,
    pub ephemeral: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Message {
    pub id: Option<String>,
    pub topic: Topic,
    pub retain: Option<bool>,
    pub mine: Option<bool>,
    pub timestamp: Option<u128>,
    pub content: MessageContent,
    pub sender: Option<String>,
    pub receiver: Option<String>,
    pub ephemeral: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Topic {
    pub topic: String,
    pub header: Header,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MessageContent {
    #[serde(rename = "type")]
    pub message_type: String,
    pub raw: String,
    pub html: Option<String>,
}

impl Message {
    pub fn to_mqtt(&self) -> anyhow::Result<mqtt::Message> {
        let mut props = mqtt::properties::Properties::new();

        props.push_string_pair(
            mqtt::PropertyCode::UserProperty,
            "sender",
            &crate::config::SYSCONIFG.lock().clone().get_clint_name(),
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
            self.content.message_type.clone().as_str(),
        )?;

        let msg = mqtt::MessageBuilder::new()
            .topic(self.topic.topic.clone())
            .payload(self.content.raw.clone())
            .properties(props)
            .qos(self.topic.header.qos)
            .retained(self.retain.unwrap_or(false))
            .finalize();

        Ok(msg)
    }

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
}

impl TryFrom<mqtt::Message> for Message {
    type Error = anyhow::Error;

    fn try_from(msg: mqtt::Message) -> Result<Self, Self::Error> {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

        let topic = Topic::try_from(msg.topic())?;

        let sender = msg.properties().find_user_property("sender");

        let mine = sender
            .clone()
            .context("Unable to find publish property")?
            .eq(&crate::config::SYSCONIFG.lock().clone().get_clint_name());

        let receiver = msg.properties().find_user_property("receiver");

        let ephemeral =
            msg.properties().find_user_property("ephemeral") == Some("true".to_string());

        let message_type = msg
            .properties()
            .get_string(mqtt::PropertyCode::ContentType)
            .context("Unable to find content type property")?;

        let content = MessageContent {
            message_type,
            raw: msg.payload_str().to_string(),
            html: None,
        };

        Ok(Message {
            id: Some(nanoid::nanoid!()),
            topic,
            timestamp: Some(now.as_millis()),
            mine: Some(mine),
            content,
            retain: Some(msg.retained()),
            sender,
            receiver,
            ephemeral,
        })
    }
}

impl TryFrom<&str> for Topic {
    type Error = anyhow::Error;
    fn try_from(topic_str: &str) -> Result<Self, Self::Error> {
        let header = get_pattern(&topic_str).context("Unable to find matching topic")?;

        Ok(Topic {
            topic: topic_str.to_string(),
            header,
        })
    }
}

impl TryFrom<String> for Topic {
    type Error = anyhow::Error;
    fn try_from(topic_str: String) -> Result<Self, Self::Error> {
        let header = get_pattern(&topic_str).context("Unable to find matching topic")?;

        Ok(Topic {
            topic: topic_str.to_string(),
            header,
        })
    }
}

fn get_pattern<T: AsRef<str>>(topic: &T) -> Option<Header> {
    crate::sys_config
        .lock()
        .mqtt_config
        .get_topics_with_sys()
        .clone()
        .into_iter()
        .find(|pattern| mqtt_topic_matches(pattern, topic.as_ref()))
}

pub fn mqtt_topic_matches(pattern: &Header, topic: &str) -> bool {
    if let Some(pattern) = &pattern.topic {
        let pattern = pattern.as_str();
        let mut pattern_parts = pattern.split('/').peekable();
        let mut topic_parts = topic.split('/').peekable();

        while pattern_parts.peek().is_some() || topic_parts.peek().is_some() {
            match (pattern_parts.next(), topic_parts.next()) {
                (Some("#"), _) => {
                    // # 匹配该级别及其所有子级
                    return true;
                }
                (Some("+"), None) | (None, Some(_)) => {
                    // + 需要匹配一个级别，如果没有额外的级别，则不匹配
                    return false;
                }
                (Some("+"), Some(_)) => {
                    // + 匹配任何单个级别
                }
                (Some(pattern), Some(topic)) => {
                    // 如果两者不相等，则不匹配
                    if pattern != topic {
                        return false;
                    }
                }
                _ => {
                    // 其他情况，不匹配
                    return false;
                }
            }
        }

        true
    } else {
        false
    }
}

#[test]
fn mime_test() {
    let msg_type = "text/markdown; charset=UTF-8";
    let msg_tyme: mime::Mime = msg_type.parse().unwrap();

    println!("{:?}", msg_tyme.essence_str());
}
