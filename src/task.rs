use anyhow::Context;
use cron::Schedule;
use linked_hash_map::LinkedHashMap;
use log::{error, info};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr, sync::Arc};
use tokio::sync::oneshot::Sender;

use crate::{config::SysConfig, r_db};

lazy_static! {
    pub static ref TASK_MANGER: Arc<Mutex<TaskManager>> = Arc::new(Mutex::new(TaskManager::new()));
    static ref LUA: Mutex<mlua::Lua> = Mutex::new(get_lua());
}

pub struct TaskManager {
    runtime: tokio::runtime::Runtime,
    tasks: LinkedHashMap<String, TaskRunner>,
}

struct TaskRunner {
    tx: Option<Sender<()>>,
    task: Task,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Task {
    pub path: PathBuf,
    pub cron: String,
    pub name: String,
    pub remark: Option<String>,
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskManager {
    pub fn new() -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        Self {
            runtime,
            tasks: LinkedHashMap::new(),
        }
    }

    pub fn start(&mut self) {
        let tasks = crate::r_db::get_all_tasks().unwrap();
        for (id, task) in tasks {
            let (tx, rx) = tokio::sync::oneshot::channel::<()>();

            let runner = TaskRunner::new(task.clone(), tx);

            let log_id = id.clone();
            let log_task = task.clone();

            self.tasks.insert(id.clone(), runner);
            self.runtime.spawn(async move {
                tokio::select! {
                result = task.run() => {
                    match result {
                        Ok(_) => {
                            info!("{} auto stop", id)
                        },
                        Err(e) => {
                            println!("{}", e);
                            error!("{} auto stop, error: {}", id, e)
                        }
                    }
                },
                _ = rx => {
                    info!("{} manual stop", id)
                },
                }
            });
            info!(
                "Task {}-[{}]:{} ---- starting",
                log_id,
                log_task.path.as_os_str().to_str().unwrap(),
                log_task.name
            )
        }
        info!("TaskManger started");
    }

    pub fn stop_task(&mut self, id: &str) -> anyhow::Result<()> {
        let runner = self
            .tasks
            .get_mut(id)
            .ok_or(anyhow::anyhow!("Task Not Found"))?;
        runner.stop()?;
        Ok(())
    }

    pub fn stop_all(&mut self) -> anyhow::Result<()> {
        for (_, runner) in self.tasks.iter_mut() {
            runner.stop()?;
        }
        Ok(())
    }

    pub fn add_task(&mut self, task: Task) -> anyhow::Result<()> {
        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let id = r_db::add_task(&task)?;

        let runner = TaskRunner::new(task.clone(), tx);

        self.tasks.insert(id.clone(), runner);
        self.runtime.spawn(async move {
            tokio::select! {
                result = task.run() => {
                    match result {
                        Ok(_) => {
                            info!("{} auto stop", id)
                        },
                        Err(e) => {
                            println!("{}", e);
                            error!("{} auto stop, error: {}", id, e)
                        }
                    }
                },
                _ = rx => {
                    info!("{} manual stop", id)
                },
            }
        });
        Ok(())
    }

    pub fn remove_task(&mut self, id: &str) -> anyhow::Result<()> {
        self.stop_task(id)?;
        self.tasks.remove(id);
        r_db::remove_task(&String::from(id))?;
        Ok(())
    }

    pub fn get_task(&self, id: &str) -> anyhow::Result<Task> {
        let runner = self
            .tasks
            .get(id)
            .ok_or(anyhow::anyhow!("Task Not Found"))?;
        Ok(runner.task.clone())
    }

    pub fn get_all_task(&self) -> anyhow::Result<Vec<Task>> {
        let mut tasks = Vec::new();
        for (_, runner) in self.tasks.iter() {
            tasks.push(runner.task.clone());
        }
        Ok(tasks)
    }

    pub fn start_task(&mut self, id: &String) -> anyhow::Result<()> {
        let runner = self
            .tasks
            .get_mut(id)
            .ok_or(anyhow::anyhow!("Task Not Found"))?;

        let (tx, rx) = tokio::sync::oneshot::channel::<()>();

        if runner.tx.is_none() {
            runner.tx = Some(tx);
            let task = runner.task.clone();
            let id = id.clone();
            self.runtime.spawn(async move {
                tokio::select! {
                result = task.run() => {
                    match result {
                        Ok(_) => {
                            info!("{} auto stop", id)
                        },
                        Err(e) => {
                            println!("{}", e);
                            error!("{} auto stop, error: {}", id, e)
                        }
                    }
                },
                _ = rx => {
                    info!("{} manual stop", id)
                },
                }
            });
        } else {
            return Err(anyhow::anyhow!("Task is running, please stop it first"));
        }
        Ok(())
    }
}

impl TaskRunner {
    fn new(task: Task, tx: Sender<()>) -> Self {
        Self { tx: Some(tx), task }
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        if let Some(tx) = self.tx.take() {
            tx.send(()).unwrap();
        }

        Ok(())
    }
}

impl Task {
    pub fn new(path: PathBuf, cron: String, name: String, remark: Option<String>) -> Self {
        Self {
            path,
            cron,
            name,
            remark,
        }
    }

    /// 执行脚本
    /// let _ = script.exec().unwrap();
    /// let _ = script.call::<_, mlua::Value>(()).unwrap();
    /// let _ = script.eval::<mlua::Value>().unwrap();
    pub async fn run(&self) -> anyhow::Result<()> {
        let schedule = Schedule::from_str(self.cron.as_str()).unwrap();

        let lua = get_lua();

        let script_content = tokio::fs::read_to_string(self.path.clone()).await?;

        loop {
            let script = lua.load(script_content.clone());
            script.exec()?;

            let now = chrono::offset::Local::now();
            let next = schedule
                .upcoming(chrono::offset::Local)
                .next()
                .context("No upcoming dates")?;
            let duration = (next - now).to_std()?;

            tokio::time::sleep(duration).await;
        }
    }
}

struct TymeUserData;

impl mlua::UserData for TymeUserData {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("sys_config", get_sys_config);
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method("send_json", lua_send_json);
        methods.add_async_method("send_markdown", lua_send_markdown);
    }
}

fn get_sys_config<'a>(_: &mlua::Lua, _: &'a TymeUserData) -> mlua::Result<SysConfig> {
    let sys_config = crate::sys_config.lock().clone();
    Ok(sys_config)
}

async fn lua_send_json(
    _: &mlua::Lua,
    _: &TymeUserData,
    (topic, qos, ephemeral, json): (String, i32, bool, mlua::Value<'_>),
) -> mlua::Result<()> {
    let json_string = serde_json::to_string(&json).unwrap();

    let msg = crate::message::SendMessage {
        topic,
        qos,
        retain: None,
        receiver: None,
        ephemeral,
        message_type: "application/json".to_string(),
        raw: json_string,
    };

    crate::clint::publish(msg).await.unwrap();
    Ok(())
}

async fn lua_send_markdown(
    _: &mlua::Lua,
    _: &TymeUserData,
    (topic, qos, ephemeral, markdown): (String, i32, bool, mlua::Value<'_>),
) -> mlua::Result<()> {
    let markdown_string = markdown.to_string().unwrap();

    let msg = crate::message::SendMessage {
        topic,
        qos,
        retain: None,
        receiver: None,
        ephemeral,
        message_type: "text/markdown".to_string(),
        raw: markdown_string,
    };

    crate::clint::publish(msg).await.unwrap();

    Ok(())
}

fn get_lua() -> mlua::Lua {
    let lua = mlua::Lua::new();
    let package_path = lua
        .globals()
        .get::<_, mlua::Table>("package")
        .unwrap()
        .get::<_, String>("path")
        .unwrap();

    let package_cpath = lua
        .globals()
        .get::<_, mlua::Table>("package")
        .unwrap()
        .get::<_, String>("cpath")
        .unwrap();

    let package_path = format!("{};./script/?.lua", package_path);
    let package_cpath = format!("{};./script/?.so;./script/?.dll", package_cpath);

    lua.globals()
        .get::<_, mlua::Table>("package")
        .unwrap()
        .set("path", package_path)
        .unwrap();

    lua.globals()
        .get::<_, mlua::Table>("package")
        .unwrap()
        .set("cpath", package_cpath)
        .unwrap();

    lua.globals().set("tyme_sys", TymeUserData).unwrap();

    lua
}

#[test]
pub fn test() {
    let rn = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rn.spawn(async {
        tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        let mut task_manger = TASK_MANGER.lock();
        let _ = task_manger.stop_all();
    });

    rn.block_on(async {
        let mut task_manger = TASK_MANGER.lock();
        task_manger.start();
        parking_lot::MutexGuard::unlock_fair(task_manger);
        tokio::time::sleep(std::time::Duration::from_secs(35)).await;
    });
}

#[test]
fn add_task() {
    let tasks = Task::new(
        PathBuf::from("./script/test.lua"),
        "*/5 * * * * *".to_string(),
        "os".to_string(),
        None,
    );

    r_db::add_task(&tasks).unwrap();
}

#[test]
fn delete_all_task() {
    r_db::_delete_all_tasks().unwrap();
}

#[test]
fn get_all_task() {
    let tasks = r_db::get_all_tasks().unwrap();
    println!("{:?}", tasks);
}
