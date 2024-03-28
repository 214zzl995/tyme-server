use std::{collections::HashMap, net::SocketAddr, ops::ControlFlow, sync::Arc};

use askama::Template;
use axum::{
    extract::{
        ws::{Message as wsMessage, WebSocket},
        ConnectInfo, Path, WebSocketUpgrade,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, TypedHeader,
};
use futures::{SinkExt, StreamExt};
use log::info;
use parking_lot::Mutex;
use serde_json::json;
use tokio::sync::mpsc::{self, Sender};
use tower_sessions::Session;

lazy_static! {
    static ref WSCLINTS: Arc<Mutex<HashMap<String, Sender<wsMessage>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

#[derive(serde::Deserialize)]
pub struct MsgParams {
    pub id: String,
}

#[derive(Template)]
#[template(path = "msg.html")]
struct MsgTemplate {
    name: String,
    content: String,
}

#[allow(clippy::unused_async)]
pub async fn send(Json(msg): Json<crate::message::SendMessage>) -> impl IntoResponse {
    match crate::clint::publish(msg.clone()).await {
        Ok(_) => Json(json!({"result": "ok", "message": "Push success"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

#[allow(clippy::unused_async)]
pub async fn get_mqtt_user() -> impl IntoResponse {
    Json(
        json!({"result": "ok", "message": "Get success", "user": crate::tyme_config.lock().get_clint_name()}),
    )
}

#[allow(clippy::unused_async)]
pub async fn get_all_toppic() -> impl IntoResponse {
    Json(json!({"result": "ok", "topics": crate::headers.lock().clone()}))
}

#[allow(clippy::unused_async)]
pub async fn subscribe_topic(Json(topics): Json<Vec<crate::header::Header>>) -> impl IntoResponse {
    match crate::clint::subscribe_topic(topics).await {
        Ok(_) => Json(json!({"result": "ok", "message": "Push success"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

#[allow(clippy::unused_async)]
pub async fn get_chat_msg(Path(header): Path<String>) -> impl IntoResponse {
    match crate::db::get_msg_by_header(&header).await {
        Ok(msgs) => Json(json!({"result": "ok", "data": msgs})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

#[allow(clippy::unused_async)]
pub async fn msg(Path(id): Path<String>) -> impl IntoResponse {
    if let Ok(msg) = crate::db::get_msg_by_id(&id).await {
        if let Some(msg) = msg {
            let template = MsgTemplate {
                name: "丁真珍珠".to_string(),
                content: msg.content.html.unwrap(),
            };
            template.into_response()
        } else {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Msg Not Found".to_string())
                .unwrap()
                .into_response()
        }
    } else {
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Get Error".to_string())
            .unwrap()
            .into_response()
    }
}

#[allow(clippy::unused_async)]
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    session: Session,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    info!("`{user_agent}` at {addr} connected.");

    ws.on_upgrade(move |socket| handle_socket(socket, addr, session))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr, session: Session) {
    if socket.send(wsMessage::Ping(vec![1, 2, 3])).await.is_ok() {
        info!("Pinged {who}...");
    } else {
        info!("Could not send ping {who}!");
        return;
    }

    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(msg, who, session.clone()).is_break() {
                return;
            }
        } else {
            info!("client {who} abruptly disconnected");
            return;
        }
    }

    let (mut sink, mut stream) = socket.split();

    let (sender, mut receiver) = mpsc::channel::<wsMessage>(16);

    let mut send_task = tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            if sink.send(message).await.is_err() {
                break;
            }
        }
    });

    WSCLINTS.lock().insert(session.id().0.to_string(), sender);

    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;

        while let Some(Ok(msg)) = stream.next().await {
            cnt += 1;
            if process_message(msg, who, session.clone()).is_break() {
                break;
            }
        }
        cnt
    });

    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(_) => info!("Send task completed"),
                Err(_) => info!("Send task failed")
            }
            recv_task.abort();
        }
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => info!("Received {b} messages"),
                Err(b) => info!("Error receiving messages {b:?}")
            }
            send_task.abort();
        }
    }

    info!("Websocket context {who} destroyed");
}

fn process_message(msg: wsMessage, who: SocketAddr, session: Session) -> ControlFlow<(), ()> {
    match msg {
        wsMessage::Text(t) => {
            info!(">>> {who} sent str: {t:?}");
        }
        wsMessage::Binary(d) => {
            info!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        wsMessage::Close(c) => {
            if let Some(cf) = c {
                info!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                info!(">>> {who} somehow sent close message without CloseFrame");
            }
            remove_clint(&session);

            return ControlFlow::Break(());
        }

        wsMessage::Pong(v) => {
            info!(">>> {who} sent pong with {v:?}");
        }
        wsMessage::Ping(v) => {
            info!(">>> {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}

pub fn remove_clint(session: &Session) {
    let user_id = session.id().0.to_string();

    WSCLINTS.lock().remove(&user_id);
}

pub async fn _ws_send(
    session: Session,
    header: crate::header::Header,
    msg: &crate::message::RecMessage,
) {
    let user_id = session.id().0.to_string();

    let msg = json!({"header": header, "msg": msg});
    let msg = serde_json::to_string(&msg).unwrap();
    let msg = wsMessage::Text(msg);

    let clint = {
        let mut clints = WSCLINTS.lock();
        clints.get_mut(&user_id).map(|clint| clint.clone())
    };

    if let Some(clint) = clint {
        clint.send(msg).await.unwrap();
    }
}

pub async fn ws_send_all(header: &crate::header::Header, msg: &crate::message::RecMessage) {
    let msg = json!({"header": header, "msg": msg});
    let msg = serde_json::to_string(&msg).unwrap();
    let msg = wsMessage::Text(msg);

    let clints = {
        let clints = WSCLINTS.lock();
        clints.clone()
    };

    for (_, clint) in clints {
        clint.send(msg.clone()).await.unwrap();
    }
}
