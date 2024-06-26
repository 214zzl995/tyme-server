use std::{net::SocketAddr, ops::ControlFlow};

use askama::Template;
use axum::{
    extract::{
        ws::{Message as wsMessage, WebSocket},
        ConnectInfo, Path, Query, State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, TypedHeader,
};
use futures::{SinkExt, StreamExt};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::{
    broadcast::{self, Receiver},
    mpsc::UnboundedSender,
};
use tower_sessions::Session;

use crate::{
    header::Header,
    message::{RecMessage, SendMessage},
};

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

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct PageParam {
    #[serde(default)]
    pub page_num: usize,
    #[serde(default)]
    pub page_size: usize,
}

#[allow(clippy::unused_async)]
pub async fn send(
    State(send_msg_tx): State<UnboundedSender<SendMessage>>,
    Json(msg): Json<crate::message::SendMessage>,
) -> impl IntoResponse {
    match send_msg_tx.send(msg.clone()) {
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
    match Header::get_db_headers().await {
        Ok(headers) => Json(json!({"result": "ok", "topics": headers})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

#[allow(clippy::unused_async)]
pub async fn subscribe_topic(
    State(sub_header_tx): State<UnboundedSender<Header>>,
    Json(topics): Json<crate::header::Header>,
) -> impl IntoResponse {
    match sub_header_tx.send(topics) {
        Ok(_) => Json(json!({"result": "ok", "message": "Push success"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

#[allow(clippy::unused_async)]
pub async fn get_all_messages_by_header(Path(id): Path<String>) -> impl IntoResponse {
    match crate::message::RecMessage::get_msg_by_header(&id).await {
        Ok(msgs) => Json(json!({"result": "ok", "data": msgs})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

#[allow(clippy::unused_async)]
pub async fn get_message_count_by_header(Path(id): Path<String>) -> impl IntoResponse {
    match crate::message::RecMessage::get_msg_count_by_header(&id).await {
        Ok(count) => Json(json!({"result": "ok", "count": count})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

#[allow(clippy::unused_async)]
pub async fn get_page_messages_by_header(
    Path(id): Path<String>,
    Query(page_param): Query<PageParam>,
) -> impl IntoResponse {
    match crate::message::RecMessage::get_page_msg_by_header(&id, &page_param).await {
        Ok(msgs) => Json(json!({"result": "ok", "data": msgs})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

#[allow(clippy::unused_async)]
pub async fn stand_alone_message(Path(id): Path<String>) -> impl IntoResponse {
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
    State(rec_msg_tx): State<broadcast::Sender<(Header, RecMessage)>>,
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

    ws.on_upgrade(move |socket| handle_socket(socket, addr, session, rec_msg_tx.subscribe()))
}

async fn handle_socket(
    mut socket: WebSocket,
    who: SocketAddr,
    session: Session,
    mut rec_msg_rx: Receiver<(Header, RecMessage)>,
) {
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

    let mut send_task = tokio::spawn(async move {
        while let Ok((header, msg)) = rec_msg_rx.recv().await {
            let msg = json!({"header": header, "msg": msg});
            let msg = serde_json::to_string(&msg).unwrap();
            let msg = wsMessage::Text(msg);
            if sink.send(msg).await.is_err() {
                info!("Error sending message to {who}");
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = stream.next().await {
            if process_message(msg, who, session.clone()).is_break() {
                break;
            }
        }
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
                Ok(_) => info!("Receive task completed"),
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
                    ">>> {}: {} sent close with code {} and reason `{}`",
                    session.id(),
                    who,
                    cf.code,
                    cf.reason
                );
            } else {
                info!(">>> {who} somehow sent close message without CloseFrame");
            }

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
