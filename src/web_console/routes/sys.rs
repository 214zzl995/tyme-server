use axum::{response::IntoResponse, Json};

use crate::config::SYSCONIFG;

#[allow(clippy::unused_async)]
pub async fn get_config() -> impl IntoResponse {
    let config = SYSCONIFG.clone();
    Json(config)
}
