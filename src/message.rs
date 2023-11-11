use std::time::SystemTime;

use anyhow::Context;
use paho_mqtt::{self as mqtt};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Message {
    pub id: Option<String>,
    pub topic: Topic,
    pub retain: Option<bool>,
    pub qos: i32,
    pub mine: Option<bool>,
    pub timestamp: Option<u128>,
    pub content: MessageContent,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Topic {
    pub topic: String,
    pub header: Option<String>,
    pub publish: Option<String>,
    pub title: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MessageContent {
    #[serde(rename = "type")]
    pub message_type: MessageType,
    pub raw: String,
    pub html: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum MessageType {
    MarkDown,
    Json,
    Raw,
}

impl Message {
    pub fn to_mqtt(&self) -> anyhow::Result<mqtt::Message> {
        let payload = serde_json::to_string(&self.content)?;
        Ok(mqtt::Message::new(
            self.topic.topic.clone(),
            payload,
            self.qos,
        ))
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

impl TryFrom<mqtt::Message> for Message {
    type Error = anyhow::Error;

    fn try_from(msg: mqtt::Message) -> Result<Self, Self::Error> {
        let content = serde_json::from_str::<MessageContent>(&msg.payload_str()).unwrap();

        let topic = msg.topic().to_owned();
        let topic_node: Vec<&str> = topic.split('/').collect();

        if topic_node.len() < 3 {
            return Err(anyhow::anyhow!("topic error"));
        }

        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

        let topic = Topic::try_from(msg.topic())?;
        let mine = topic
            .clone()
            .publish
            .unwrap()
            .eq(&crate::config::SYSCONIFG.lock().clone().get_clint_name());

        Ok(Message {
            id: Some(nanoid::nanoid!()),
            topic,
            qos: msg.qos(),
            timestamp: Some(now.as_millis()),
            mine: Some(mine),
            content,
            retain: Some(msg.retained()),
        })
    }
}

impl TryFrom<&str> for Topic {
    type Error = anyhow::Error;
    fn try_from(topic_str: &str) -> Result<Self, Self::Error> {
        let topic_node: Vec<&str> = topic_str.split('/').collect();

        if topic_node.len() < 3 {
            panic!("topic error");
        }

        let header = get_pattern(&topic_str).context("Unable to find matching topic")?;
        let publish = topic_node[1].to_string();
        let title = topic_node[2].to_string();

        Ok(Topic {
            topic: topic_str.to_string(),
            publish: Some(publish),
            header: Some(header),
            title: Some(title),
        })
    }
}

impl TryFrom<String> for Topic {
    type Error = anyhow::Error;
    fn try_from(topic_str: String) -> Result<Self, Self::Error> {
        let topic_node: Vec<&str> = topic_str.split('/').collect();

        if topic_node.len() < 3 {
            panic!("topic error");
        }

        let header = get_pattern(&topic_str).context("Unable to find matching topic")?;

        let publish = topic_node[1].to_string();
        let title = topic_node[2].to_string();

        Ok(Topic {
            topic: topic_str.to_string(),
            publish: Some(publish),
            header: Some(header),
            title: Some(title),
        })
    }
}

fn get_pattern<T: AsRef<str>>(topic: &T) -> Option<String> {
    for pattern in crate::clint::TOPICS.lock().clone() {
        if mqtt_topic_matches(&pattern, topic.as_ref()) {
            return Some(pattern);
        }
    }
    None
}

fn mqtt_topic_matches(pattern: &str, topic: &str) -> bool {
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
}
