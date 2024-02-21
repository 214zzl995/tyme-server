use axum::{response::IntoResponse, Json};
use serde_json::json;

use crate::{r_db, task::Task};

pub async fn get_all_task() -> impl IntoResponse {
    match r_db::get_all_task() {
        Ok(tasks) => Json(
            json!({"result": "ok", "tasks": tasks.into_iter().map(|(id,task)| json!({"value":id,"script":task.script ,"cron":task.cron,"name":task.name,"remark":task.remark})).collect::<Vec<_>>()}),
        ),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

pub async fn add_task(Json(task): Json<Task>) -> impl IntoResponse {
    match crate::task_manger.lock().add_task(task) {
        Ok(_) => Json(json!({"result": "ok"})),
        Err(e) => Json(json!({"result": "error", "message": e.to_string()})),
    }
}

pub async fn get_all_script_file_name() -> impl IntoResponse {
    let path = std::path::Path::new("./script");
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
