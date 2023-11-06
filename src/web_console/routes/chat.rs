use axum::{response::IntoResponse, Json};
use serde_json::json;

use crate::message::Message;

#[allow(clippy::unused_async)]
pub async fn handler(Json(msg): Json<Message>) -> impl IntoResponse {
    match crate::clint::publish(msg).await {
        Ok(_) => Json(json!({"result": "ok", "message": "Push success"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}
