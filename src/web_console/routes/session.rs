// print out session

use axum::{response::IntoResponse, Json};
use log::debug;
use serde_json::json;
use tower_sessions::Session;

/// output entire session object
#[allow(clippy::unused_async)]
pub async fn handler(session: Session) -> impl IntoResponse {
    debug!("Seeking session info");
    Json(json!({ "session": format!("{:?}", session) }))
}

/// output session data in json
#[allow(clippy::unused_async)]
pub async fn session(session: Session) -> impl IntoResponse {
    debug!("Seeking session data");
    let user_id = session
        .get_value("user_id")
        .unwrap_or_else(|| serde_json::Value::String("".to_string()));
    debug!("user_id: {}", user_id);

    Json(json!({ "user_id": user_id }))
}

pub async fn guide() -> impl IntoResponse {
    Json(json!({ "guide": true }))
}
