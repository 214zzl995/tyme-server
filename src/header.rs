use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref HEADERS: Mutex<Vec<Header>> = {
        match Header::get_all_header() {
            Ok(headers) => Mutex::new(headers),
            Err(err) => {
                log::error!("Error creating the header: {}", err);
                std::process::exit(1);
            }
        }
    };
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct Header {
    pub id: Option<String>,
    pub topic: String,
    pub qos: i32,
}

impl Header {
    pub fn mqtt_topic_matches(&self, topic: &str) -> bool {
        let pattern = self.topic.as_str();
        let mut pattern_parts = pattern.split('/').peekable();
        let mut topic_parts = topic.split('/').peekable();

        while pattern_parts.peek().is_some() || topic_parts.peek().is_some() {
            match (pattern_parts.next(), topic_parts.next()) {
                (Some("#"), _) => {
                    return true;
                }
                (Some("+"), None) | (None, Some(_)) => {
                    return false;
                }
                (Some("+"), Some(_)) => {}
                (Some(pattern), Some(topic)) => {
                    if pattern != topic {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }
        }

        true
    }

    pub fn check(&self) -> anyhow::Result<()> {
        if self.topic.is_empty() {
            anyhow::bail!("topic is empty");
        }

        if self.qos < 0 || self.qos > 2 {
            anyhow::bail!("qos is invalid");
        }

        if self.mqtt_topic_matches("system/#") {
            anyhow::bail!("system/# is a reserved topic");
        }

        Ok(())
    }
}
