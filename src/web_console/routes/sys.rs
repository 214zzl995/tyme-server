use axum::{response::IntoResponse, Json};
use serde_json::json;

use crate::config::{TymeConfig, TYME_CONFIG};

#[allow(clippy::unused_async)]
pub async fn get_config() -> impl IntoResponse {
    let config = TYME_CONFIG.clone();
    Json(config)
}

#[allow(clippy::unused_async)]
pub async fn update_config(Json(config): Json<TymeConfig>) -> impl IntoResponse {

    match config.update().await {
        Ok(_) => Json(json!({"result": "ok"})),
        Err(err) => Json(json!({"result": "error","message" : format!("Update Config{}",err)})),
    }
}
