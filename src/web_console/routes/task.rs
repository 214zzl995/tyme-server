use axum::{response::IntoResponse, Json};
use serde_json::json;

use crate::r_db;

pub async fn get_all_task() -> impl IntoResponse {
    match r_db::get_all_task(){
        Ok(tasks) => Json(json!({"result": "ok", "tasks": tasks})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    
    }

}
