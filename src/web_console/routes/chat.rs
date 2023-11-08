use axum::{response::IntoResponse, Json};
use serde_json::json;

use crate::message::{Message, MessageContent, MessageType};

#[allow(clippy::unused_async)]
pub async fn send(Json(msg): Json<Message>) -> impl IntoResponse {
    match crate::clint::publish(msg).await {
        Ok(_) => Json(json!({"result": "ok", "message": "Push success"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

#[allow(clippy::unused_async)]
pub async fn get_all_toppic() -> impl IntoResponse {
    Json(json!({"result": "ok", "topics": crate::clint::TOPICS.clone()}))
}

#[allow(clippy::unused_async)]
pub async fn subscribe_topic(Json(topics): Json<Vec<String>>) -> impl IntoResponse {
    match crate::clint::subscribe_topic(topics).await {
        Ok(_) => Json(json!({"result": "ok", "message": "Push success"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

#[allow(clippy::unused_async)]
pub async fn get_chat_msg() -> impl IntoResponse {
    let mut msgs: Vec<Message> = vec![
        Message {
            topic: "test".to_string(),
            qos: 0,
            mine: Some(true),
            content: MessageContent {
                message_type: MessageType::MarkDown,
                raw: "## Hello".to_string(),
                html: None,
            },
        },
        Message {
            topic: "test".to_string(),
            qos: 0,
            mine: Some(false),
            content: MessageContent {
                message_type: MessageType::Json,
                raw: r#"{"name": "hello"}"#.to_string(),
                html: None,
            },
        },
        Message {
            topic: "test".to_string(),
            qos: 0,
            mine: Some(false),
            content: MessageContent {
                message_type: MessageType::Raw,
                raw: "Hello".to_string(),
                html: None,
            },
        },
    ];

    for _ in 0..100 {
        msgs.push(Message {
            topic: "test".to_string(),
            qos: 0,
            mine: Some(false),
            content: MessageContent {
                message_type: MessageType::MarkDown,
                raw: format!(
                    r#"2. rust检查不到vcpkg
 ```toml
 #解决方案1
 #设置环境变量 这种方式只配置OPENSSL环境所在位置
 OPENSSL_DIR="C:\\Users\\Leri\\Path\\vcpkg\\packages\\openssl_x86-windows"  
 ``` "#),
                html: None,
            },
        })
    }

    let msgs: Vec<Message> = msgs
        .into_iter()
        .map(|mut msg| {
            if msg.content.message_type.eq(&MessageType::MarkDown) {
                let html: String = markdown::to_html(&msg.content.raw);
                msg.content.html = Some(html);
            } else if msg.content.message_type.eq(&MessageType::Json) {
                let html: String =
                    markdown::to_html(&format!("```json \n{}\n```", &msg.content.raw));
                msg.content.html = Some(html);
            } else {
                msg.content.html = Some(msg.content.raw.clone());
            }
            msg
        })
        .collect();

    Json(json!({"result": "ok", "data": msgs}))
}
