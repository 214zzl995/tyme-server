use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::task::Task;

pub async fn get_all_task(State(task_manager): State<crate::TaskManager>) -> impl IntoResponse {
    match task_manager.get_all_task() {
        Ok(tasks) => Json(
            json!({"result": "ok", "tasks": tasks.into_iter().map(|(running,task)| json!({"task":task,"running":running})).collect::<Vec<_>>()}),
        ),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

pub async fn add_task(
    State(task_manager): State<crate::TaskManager>,
    Json(task): Json<Task>,
) -> impl IntoResponse {
    match task_manager.add_task(task).await {
        Ok(id) => Json(json!({"result": "ok","id":id})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

pub async fn restart_task(
    Path(id): Path<String>,
    State(task_manager): State<crate::TaskManager>,
) -> impl IntoResponse {
    match task_manager.restart_task(&id) {
        Ok(_) => Json(json!({"result": "ok"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

pub async fn remove_task(
    Path(id): Path<String>,
    State(task_manager): State<crate::TaskManager>,
) -> impl IntoResponse {
    match task_manager.remove_task(&id).await {
        Ok(_) => Json(json!({"result": "ok"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

pub async fn stop_task(
    Path(id): Path<String>,
    State(task_manager): State<crate::TaskManager>,
) -> impl IntoResponse {
    match task_manager.stop_task(&id) {
        Ok(_) => Json(json!({"result": "ok"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

pub async fn start_task(
    Path(id): Path<String>,
    State(task_manager): State<crate::TaskManager>,
) -> impl IntoResponse {
    match task_manager.start_task(&id) {
        Ok(_) => Json(json!({"result": "ok"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

pub async fn update_task(
    State(task_manager): State<crate::TaskManager>,
    Path(id): Path<String>,
    Json(task): Json<Task>,
) -> impl IntoResponse {
    match task_manager.update_task(&id, task).await {
        Ok(_) => Json(json!({"result": "ok"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

pub async fn get_all_script_file_name() -> impl IntoResponse {
    let path = crate::start_param.word_dir.join("script");
    let mut files = vec![];
    if path.exists() {
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let ex = path.extension();
            if path.is_file() && ex.is_some() && ex.unwrap() == "lua" {
                files.push(path.file_name().unwrap().to_str().unwrap().to_string());
            }
        }
    }

    Json(
        json!({"result": "ok", "scripts": files.into_iter().map(|file| json!({"value":file,"name":file})).collect::<Vec<_>>()}),
    )
}
