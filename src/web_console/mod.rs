use std::{net::SocketAddr, sync::Arc};

use axum::Router;
use axum_sessions::{async_session::MemoryStore, SessionLayer};
use parking_lot::Mutex;
use tokio::{
    signal,
    sync::mpsc::{self, Sender},
};

use crate::config::SYSCONIFG;

mod middlewares;
mod routes;
mod services;
mod store;

lazy_static! {
    static ref SD_CANNEL: Mutex<Option<Sender<bool>>> = Mutex::new(None);
}

pub async fn run_web_console() -> anyhow::Result<()> {
    let host = if SYSCONIFG.web_console_config.public {
        [0, 0, 0, 0]
    } else {
        [127, 0, 0, 1]
    };

    let addr = SocketAddr::from((host, SYSCONIFG.web_console_config.port));

    let server = axum::Server::try_bind(&addr)?;

    let api_token = SYSCONIFG
        .web_console_config
        .api_token
        .clone()
        .unwrap_or(nanoid::nanoid!(8));

    println!("WebConsole API Token:{}", api_token);

    let shared_state = Arc::new(store::Store::new(api_token));

    let session_layer = SessionLayer::new(
        MemoryStore::new(),
        SYSCONIFG.web_console_config.secret.as_bytes(),
    )
    .with_cookie_name("Auth");

    let app = Router::new()
        .merge(services::front_public_route())
        .merge(services::backend(session_layer, shared_state));

    let server = server
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal());

    println!("WebConsole Listening on {}", addr);

    let _ = server.await;

    Ok(())
}

pub async fn shutdown_signal() {
    let (tx, mut rx) = mpsc::channel(1);

    SD_CANNEL.lock().replace(tx);

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
        _ = rx.recv() => {},
    }

    SD_CANNEL.lock().take();
}
