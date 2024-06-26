use axum::{response::IntoResponse, Json};
use log::info;
use serde::Deserialize;
use serde_json::json;
use tower_sessions::Session;

/// route to handle log in
#[allow(clippy::unused_async)]
#[allow(clippy::missing_panics_doc)]
pub async fn login(session: Session, Json(login): Json<Login>) -> impl IntoResponse {
    info!("Logging in user: {}", login.username);

    if check_password(&login.username, &login.password) {
        session.insert("user_id", login.username).unwrap();
        Json(json!({"result": "ok"}))
    } else {
        Json(json!({"result": "error", "message": "Invalid username or password"}))
    }
}

/// route to handle log out
#[allow(clippy::unused_async)]
pub async fn logout(session: Session) -> impl IntoResponse {
    let user = session.get_value("user_id").unwrap_or_default();
    info!("Logging out user: {}", user);

    // drop session
    session.delete();
    Json(json!({"result": "ok"}))
}

// assume all passwords work
fn check_password(username: &str, password: &str) -> bool {
    let con_conf = crate::tyme_config.lock().clone();
    let con_conf = con_conf.web_console_config;
    username.eq(&con_conf.username) && password.eq(&con_conf.password)
}

#[derive(Deserialize)]
pub struct Login {
    username: String,
    password: String,
}
