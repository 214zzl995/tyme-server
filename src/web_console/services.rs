use axum::{
    error_handling::HandleErrorLayer,
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    middleware,
    routing::{get, post},
    Router,
};
use std::{path::PathBuf, sync::Arc};
use tower::{BoxError, ServiceBuilder};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tower_sessions::{SessionManagerLayer, SessionStore};

use crate::config::SYSCONIFG;

use super::{
    middlewares, routes,
    store::{self, Store},
};

// *********
// FRONT END
// *********
// Front end to server svelte build bundle, css and index.html from public folder
pub fn front_public_route() -> Router {
    let front_end_path = SYSCONIFG
        .web_console_config
        .front_end_path
        .clone()
        .unwrap_or(PathBuf::from("./assets"));

    Router::new()
        .fallback_service(
            ServeDir::new(front_end_path).not_found_service(handle_error.into_service()),
        )
        .layer(TraceLayer::new_for_http())
}

#[allow(clippy::unused_async)]
async fn handle_error() -> (StatusCode, &'static str) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong accessing static files...",
    )
}

// ********
// BACK END
// ********
// Back end server built form various routes that are either public, require auth, or secure login
pub fn backend<Store: SessionStore>(
    session_layer: SessionManagerLayer<Store>,
    shared_state: Arc<store::Store>,
) -> Router {
    let session_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(session_layer);

    // could add tower::ServiceBuilder here to group layers, especially if you add more layers.
    // see https://docs.rs/axum/latest/axum/middleware/index.html#ordering
    Router::new()
        .merge(back_public_route())
        .merge(back_auth_route())
        .merge(back_token_route(shared_state))
        .layer(session_service)
}

// *********
// BACKEND NON-AUTH
// *********
//
pub fn back_public_route() -> Router {
    Router::new()
        .route("/auth/session", get(routes::session::data_handler)) // gets session data
        .route("/auth/login", post(routes::login)) // sets username in session
        .route("/auth/logout", get(routes::logout)) // deletes username in session
        .route("/test", get(routes::not_implemented_route))
}

// *********
// BACKEND SESSION
// *********
//
pub fn back_auth_route() -> Router<()> {
    Router::new()
        .route("/secure", get(routes::session::handler))
        .nest("/c", back_chat_route_c())
        .route_layer(middleware::from_fn(middlewares::user_secure))
}

// *********
// BACKEND API
// *********
//
// invoked with State that stores API that is checked by the `middleware::auth`
pub fn back_token_route<S>(state: Arc<Store>) -> Router<S> {
    Router::new()
        .route("/check", get(routes::api::handler))
        .nest("/a", back_chat_route_a(state.clone()))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::auth,
        ))
        .with_state(state)
}

pub fn _back_chat_route<S>(state: S) -> Router<S>
where
    S: Send + Sync + 'static + Clone,
{
    let route = Router::new().route("/send", post(routes::chat::handler));

    route.with_state(state)
}

pub fn back_chat_route_c() -> Router<()> {
    Router::new()
        .route("/send", post(routes::chat::handler))
        .route("/upload/:file_name", post(routes::file::upload_crt))
        .route(
            "/config",
            get(routes::sys::get_config).post(routes::sys::update_config),
        )
}

pub fn back_chat_route_a<S>(state: Arc<Store>) -> Router<S> {
    Router::new()
        .route("/send", post(routes::chat::handler))
        .with_state(state)
}
