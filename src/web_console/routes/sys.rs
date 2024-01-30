use axum::{response::IntoResponse, Json};
use serde_json::json;

use crate::config::{SysConfig, SYSCONIFG};

#[allow(clippy::unused_async)]
pub async fn get_config() -> impl IntoResponse {
    let config = SYSCONIFG.clone();
    Json(config)
}

#[allow(clippy::unused_async)]
pub async fn update_config(Json(config): Json<SysConfig>) -> impl IntoResponse {

    match config.update().await {
        Ok(_) => Json(json!({"result": "ok"})),
        Err(err) => Json(json!({"result": "error","message" : format!("Update Config{}",err)})),
    }
}
