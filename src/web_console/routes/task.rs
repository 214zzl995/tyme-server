use axum::{extract::Path, response::IntoResponse, Json};
use serde_json::json;

use crate::task::Task;

pub async fn get_all_task() -> impl IntoResponse {
    match crate::task_manger.lock().get_all_task() {
        Ok(tasks) => Json(
            json!({"result": "ok", "tasks": tasks.into_iter().map(|(running,task)| json!({"task":task,"running":running})).collect::<Vec<_>>()}),
        ),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

pub async fn add_task(Json(task): Json<Task>) -> impl IntoResponse {
    match crate::task_manger.lock().add_task(task) {
        Ok(id) => Json(json!({"result": "ok","id":id})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

pub async fn restart_task(Path(id): Path<String>) -> impl IntoResponse {
    match crate::task_manger.lock().restart_task(&id) {
        Ok(_) => Json(json!({"result": "ok"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

pub async fn remove_task(Path(id): Path<String>) -> impl IntoResponse {
    match crate::task_manger.lock().remove_task(&id) {
        Ok(_) => Json(json!({"result": "ok"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

pub async fn stop_task(Path(id): Path<String>) -> impl IntoResponse {
    match crate::task_manger.lock().stop_task(&id) {
        Ok(_) => Json(json!({"result": "ok"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

pub async fn start_task(Path(id): Path<String>) -> impl IntoResponse {
    match crate::task_manger.lock().start_task(&id) {
        Ok(_) => Json(json!({"result": "ok"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

pub async fn update_task(Path(id): Path<String>, Json(task): Json<Task>) -> impl IntoResponse {
    match crate::task_manger.lock().update_task(&id, task) {
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
            if path.is_file()
                && path.file_name().unwrap() != "sys.lua"
                && ex.is_some()
                && ex.unwrap() == "lua"
            {
                files.push(path.file_name().unwrap().to_str().unwrap().to_string());
            }
        }
    }

    Json(
        json!({"result": "ok", "scripts": files.into_iter().map(|file| json!({"value":file,"name":file})).collect::<Vec<_>>()}),
    )
}
