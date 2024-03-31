use std::{net::SocketAddr, sync::Arc};

use axum::Router;
use log::info;
use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedSender},
};
use tower_sessions::{MemoryStore, SessionManagerLayer};

use crate::{
    header::Header,
    message::{RecMessage, SendMessage},
};

mod middlewares;
mod routes;
mod services;
mod store;

pub async fn run_guide_web_console() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel::<()>(1);

    let addr = SocketAddr::from(([0, 0, 0, 0], 12566));

    let server = axum::Server::try_bind(&addr)?;

    let app = Router::new()
        .merge(services::front_public_route())
        .merge(services::guide_backend(tx));

    let server = server
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal(rx));

    info!("Guide Listening on {}", addr);

    let _ = server.await;

    Ok(())
}

pub async fn run_web_console(
    send_msg_tx: UnboundedSender<SendMessage>,
    rec_msg_tx: broadcast::Sender<(Header, RecMessage)>,
) -> anyhow::Result<()> {
    let (_tx, rx) = mpsc::channel::<()>(1);
    let config = crate::tyme_config.lock().clone();

    let addr = SocketAddr::from(([0, 0, 0, 0], config.web_console_config.port));

    let server = axum::Server::try_bind(&addr)?;

    let api_token = config
        .web_console_config
        .api_token
        .clone()
        .unwrap_or(nanoid::nanoid!(8));

    info!("WebConsole API Token:{}", api_token);

    let shared_state = Arc::new(store::Store::new(api_token));

    let session_store = MemoryStore::default();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_name("web_console.sid");

    let app = Router::new()
        .merge(services::front_public_route())
        .merge(services::backend(session_layer, shared_state, send_msg_tx, rec_msg_tx));

    let server = server
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal(rx));

    info!("WebConsole Listening on {}", addr);

    let _ = server.await;

    Ok(())
}

async fn shutdown_signal(mut rx: mpsc::Receiver<()>) {
    rx.recv().await;
}
