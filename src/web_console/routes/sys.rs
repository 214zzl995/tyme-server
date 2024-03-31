use axum::{extract::State, response::IntoResponse, Json};
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
        Err(err) => {
            log::error!("Config Update: {}", err);
            Json(json!({"result": "error","message" : format!("{}",err)}))
        }
    }
}

#[allow(clippy::unused_async)]
pub async fn guide_finish(
    State(mqtt_state): State<tokio::sync::mpsc::Sender<()>>,
) -> impl IntoResponse {
    if let Err(err) = mqtt_state.send(()).await {
        log::error!("Guide Final: {}", err);
        Json(json!({"result": "error","message" : format!("{}",err)}))
    } else {
        Json(json!({"result": "ok"}))
    }
}

