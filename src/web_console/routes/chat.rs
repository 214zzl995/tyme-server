use std::{collections::HashMap, net::SocketAddr, ops::ControlFlow, sync::Arc};

use axum::{
    extract::{
        ws::{Message as wsMessage, WebSocket},
        ConnectInfo, Path, WebSocketUpgrade,
    },
    response::IntoResponse,
    Json, TypedHeader,
};
use futures::{SinkExt, StreamExt};
use parking_lot::Mutex;
use serde_json::json;
use tokio::sync::mpsc::{self, Sender};
use tower_sessions::Session;

use crate::message::Message;

lazy_static! {
    static ref CLINTS: Arc<Mutex<HashMap<String, Sender<wsMessage>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

#[allow(clippy::unused_async)]
pub async fn send(Json(msg): Json<Message>) -> impl IntoResponse {
    match crate::clint::publish(msg.clone()).await {
        Ok(_) => Json(json!({"result": "ok", "message": "Push success"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

#[allow(clippy::unused_async)]
pub async fn get_mqtt_user() -> impl IntoResponse {
    Json(json!({"result": "ok", "message": "Get success", "user": crate::SYSCONIFG.lock().get_clint_name()}))
}

#[allow(clippy::unused_async)]
pub async fn get_all_toppic() -> impl IntoResponse {
    Json(json!({"result": "ok", "topics": crate::clint::TOPICS.clone()}))
}

#[allow(clippy::unused_async)]
pub async fn subscribe_topic(Json(topics): Json<Vec<String>>) -> impl IntoResponse {
    match crate::clint::subscribe_topic(topics).await {
        Ok(_) => Json(json!({"result": "ok", "message": "Push success"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

#[allow(clippy::unused_async)]
pub async fn get_chat_msg(Path(header): Path<String>) -> impl IntoResponse {
    println!("get_chat_msg:{}", header);
    let msgs: Vec<Message> = crate::r_db::get_msg_by_topic_name(&header).unwrap();

    Json(json!({"result": "ok", "data": msgs}))
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
    println!("`{user_agent}` at {addr} connected.");

    ws.on_upgrade(move |socket| handle_socket(socket, addr, session))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr, session: Session) {
    if socket.send(wsMessage::Ping(vec![1, 2, 3])).await.is_ok() {
        println!("Pinged {who}...");
    } else {
        println!("Could not send ping {who}!");
        return;
    }

    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(msg, who, session.clone()).is_break() {
                return;
            }
        } else {
            println!("client {who} abruptly disconnected");
            return;
        }
    }

    let (mut sink, mut stream) = socket.split();

    let (sender, mut receiver) = mpsc::channel::<wsMessage>(16);

    let mut send_task = tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            if sink.send(message.into()).await.is_err() {
                break;
            }
        }
    });

    CLINTS.lock().insert(session.id().0.to_string(), sender);

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
                Ok(_) => println!("Send task completed"),
                Err(_) => println!("Send task failed")
            }
            recv_task.abort();
        }
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => println!("Received {b} messages"),
                Err(b) => println!("Error receiving messages {b:?}")
            }
            send_task.abort();
        }
    }

    println!("Websocket context {who} destroyed");
}

fn process_message(msg: wsMessage, who: SocketAddr, session: Session) -> ControlFlow<(), ()> {
    match msg {
        wsMessage::Text(t) => {
            println!(">>> {who} sent str: {t:?}");
        }
        wsMessage::Binary(d) => {
            println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        wsMessage::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {who} somehow sent close message without CloseFrame");
            }
            remove_clint(&session);

            return ControlFlow::Break(());
        }

        wsMessage::Pong(v) => {
            println!(">>> {who} sent pong with {v:?}");
        }
        wsMessage::Ping(v) => {
            println!(">>> {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}

pub fn remove_clint(session: &Session) {
    let user_id = session.id().0.to_string();

    CLINTS.lock().remove(&user_id);
}

pub async fn _ws_send(session: Session, msg: &Message) {
    let user_id = session.id().0.to_string();

    let msg = serde_json::to_string(msg).unwrap();
    let msg = wsMessage::Text(msg);

    let clint = {
        let mut clints = CLINTS.lock();
        if let Some(clint) = clints.get_mut(&user_id) {
            Some(clint.clone())
        } else {
            None
        }
    };

    if let Some(clint) = clint {
        clint.send(msg).await.unwrap();
    } else {
        return;
    }
}

pub async fn ws_send_all(msg: &Message) {
    let msg = serde_json::to_string(msg).unwrap();
    let msg = wsMessage::Text(msg);

    let clints = {
        let clints = CLINTS.lock();
        clints.clone()
    };

    for (_, clint) in clints {
        clint.send(msg.clone()).await.unwrap();
    }
}
