use std::sync::Arc;

use axum::{
    extract::State,
    http::{self, Request, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use log::info;
use serde::{Deserialize, Serialize};

use crate::web_console::store::Store;


#[allow(clippy::missing_errors_doc)]
pub async fn auth<B: Send + Sync>(
    State(store): State<Arc<Store>>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, (StatusCode, Json<JsonError>)> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        info!("Authorization header missing");
        return Err((StatusCode::UNAUTHORIZED, Json(JsonError::unauthorized())));
    };

    info!("Received Authorization Header: {}", auth_header);

    // check bearer authorization to see if it matches
    if store.api_token_check(auth_header) {
        Ok(next.run(req).await)
    } else {
        info!("Authorization token does NOT match");
        Err((StatusCode::UNAUTHORIZED, Json(JsonError::unauthorized())))
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsonError {
    error: String,
}

impl JsonError {
    pub const fn _new(error: String) -> Self {
        Self { error }
    }

    pub fn unauthorized() -> Self {
        Self {
            error: "Unauthorized".into(),
        }
    }

    pub fn _internal() -> Self {
        Self {
            error: "Internal Server Error".into(),
        }
    }
}
