use axum::{
    error_handling::HandleErrorLayer,
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    middleware,
    routing::{delete, get, post},
    Router,
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedSender},
};
use tower::{BoxError, ServiceBuilder};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tower_sessions::{SessionManagerLayer, SessionStore};

use crate::{
    header::Header,
    message::{RecMessage, SendMessage},
};

use super::{
    middlewares, routes,
    store::{self, Store},
};

pub fn front_public_route() -> Router {
    let front_end_path = PathBuf::from("./assets");

    Router::new()
        .fallback_service(
            ServeDir::new(front_end_path)
                .not_found_service(handle_error.into_service())
                .precompressed_gzip(),
        )
        .layer(TraceLayer::new_for_http())
}

#[allow(clippy::unused_async)]
async fn handle_error() -> (StatusCode, &'static str) {
    (StatusCode::INTERNAL_SERVER_ERROR, "Page not found...")
}

pub fn guide_backend(shutdown_rx: mpsc::Sender<()>) -> Router {
    Router::new().nest("/g", back_guide_route(shutdown_rx))
}

pub fn backend<Store: SessionStore>(
    session_layer: SessionManagerLayer<Store>,
    shared_state: Arc<store::Store>,
    send_msg_tx: UnboundedSender<SendMessage>,
    rec_msg_tx: broadcast::Sender<(Header, RecMessage)>,
    sub_header_tx: UnboundedSender<Header>,
    task_manager: crate::TaskManager,
) -> Router {
    let session_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(session_layer);

    Router::new()
        .merge(back_auth_route(
            send_msg_tx.clone(),
            rec_msg_tx,
            sub_header_tx.clone(),
            task_manager,
        ))
        .merge(back_token_route(
            shared_state,
            send_msg_tx.clone(),
            sub_header_tx.clone(),
        ))
        .merge(back_public_route())
        .layer(session_service)
        .with_state(())
}

fn back_public_route() -> Router<()> {
    Router::new()
        .route("/auth/session", get(routes::data_handler))
        .route("/auth/login", post(routes::login))
        .route("/auth/logout", get(routes::logout))
        .route("/test", get(routes::not_implemented_route))
}

fn back_auth_route(
    send_msg_tx: UnboundedSender<SendMessage>,
    rec_msg_tx: broadcast::Sender<(Header, RecMessage)>,
    sub_header_tx: UnboundedSender<Header>,
    task_manager: crate::TaskManager,
) -> Router<()> {
    Router::new()
        .route("/secure", get(routes::session_handler))
        .nest(
            "/c",
            back_chat_route_c(send_msg_tx, rec_msg_tx, sub_header_tx, task_manager),
        )
        .route_layer(middleware::from_fn(middlewares::user_secure))
}

fn back_token_route<S>(
    state: Arc<Store>,
    send_msg_tx: UnboundedSender<SendMessage>,
    sub_header_tx: UnboundedSender<Header>,
) -> Router<S> {
    Router::new()
        .route("/check", get(routes::api_handler))
        .nest(
            "/a",
            back_chat_route_a(state.clone(), send_msg_tx, sub_header_tx),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            middlewares::auth,
        ))
        .with_state(state)
}

fn back_chat_route<S>(
    state: S,
    send_msg_tx: UnboundedSender<SendMessage>,
    sub_header_tx: UnboundedSender<Header>,
) -> Router<S>
where
    S: Send + Sync + 'static + Clone,
{
    Router::new()
        .merge(back_chat_router_msg_send(send_msg_tx))
        .merge(back_chat_router_topic(sub_header_tx))
        .route("/get-all-topic", get(routes::get_all_toppic))
        .with_state(state)
}

fn back_chat_router_msg_send<S>(send_msg_tx: UnboundedSender<SendMessage>) -> Router<S> {
    Router::new()
        .route("/send", post(routes::send))
        .with_state(send_msg_tx)
}

fn back_chat_router_topic<S>(sub_header_tx: UnboundedSender<Header>) -> Router<S> {
    Router::new()
        .route("/subscribe-topics", post(routes::subscribe_topic))
        .with_state(sub_header_tx)
}

fn back_chat_route_c(
    send_msg_tx: UnboundedSender<SendMessage>,
    rec_msg_tx: broadcast::Sender<(Header, RecMessage)>,
    sub_header_tx: UnboundedSender<Header>,
    task_manager: crate::TaskManager,
) -> Router<()> {
    Router::new()
        .merge(back_chat_route((), send_msg_tx, sub_header_tx))
        .merge(script_file())
        .merge(back_config_route())
        .merge(back_chat_route_ws(rec_msg_tx))
        .merge(back_chat_route_task(task_manager))
        .route("/msgs/:header", get(routes::get_chat_msg))
        .route("/msg/:id", get(routes::msg))
        .route("/get-mqtt-user", get(routes::get_mqtt_user))
        .route("/script-file-name", get(routes::get_all_script_file_name))
}

fn back_chat_route_task<S>(task_manager: crate::TaskManager) -> Router<S> {
    Router::new()
        .route("/task", get(routes::get_all_task))
        .route("/task", post(routes::add_task))
        .route("/task/:id", delete(routes::remove_task))
        .route("/stop-task/:id", get(routes::stop_task))
        .route("/restart-task/:id", get(routes::restart_task))
        .route("/start-task/:id", get(routes::start_task))
        .route("/update-task/:id", post(routes::update_task))
        .with_state(task_manager)
}

fn back_chat_route_ws<S>(rec_msg_tx: broadcast::Sender<(Header, RecMessage)>) -> Router<S> {
    Router::new()
        .route("/ws", get(routes::ws_handler))
        .with_state(rec_msg_tx)
}

fn script_file() -> Router {
    let script_file_path = PathBuf::from("./script");
    Router::new()
        .nest_service(
            "/script-file",
            ServeDir::new(script_file_path)
                .not_found_service(handle_error.into_service())
                .precompressed_gzip(),
        )
        .layer(TraceLayer::new_for_http())
}

fn back_config_route<S>() -> Router<S> {
    Router::new()
        .route("/upload-crt/:file_name", post(routes::upload_crt))
        .route("/upload-script/:file_name", post(routes::upload_script))
        .route(
            "/config",
            get(routes::get_config).post(routes::update_config),
        )
        .with_state(())
}

fn back_chat_route_a<S>(
    state: Arc<Store>,
    send_msg_tx: UnboundedSender<SendMessage>,
    sub_header_tx: UnboundedSender<Header>,
) -> Router<S> {
    Router::new()
        .merge(back_chat_route(state.clone(), send_msg_tx, sub_header_tx))
        .with_state(state)
}

fn back_guide_route<S>(shutdown_rx: mpsc::Sender<()>) -> Router<S> {
    Router::new()
        .merge(back_config_route())
        .route("/guide-finish", get(routes::guide_finish))
        .with_state(shutdown_rx)
}
