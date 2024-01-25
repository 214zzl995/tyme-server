use std::env;

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
    let current_dir = env::current_dir().unwrap();
    let conf = current_dir.join("../../../SysConfig.toml");

    let config_str = toml_edit::ser::to_string_pretty(&config).unwrap();
    {
        let mut loc_config = SYSCONIFG.lock();
        *loc_config = config.clone();
    }

    match tokio::fs::write(&conf, config_str).await {
        Ok(_) => Json(json!({"result": "ok"})),
        Err(err) => Json(json!({"result": "error","message" : format!("Update Config{}",err)})),
    }
}
