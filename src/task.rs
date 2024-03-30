use anyhow::Context;
use cron::Schedule;

use futures::executor::block_on;
use linked_hash_map::LinkedHashMap;
use log::{error, info};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};
use tokio::{sync::oneshot::Sender, task::block_in_place};

use crate::config::TymeConfig;

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

#[derive(Deserialize, Serialize, Clone, Debug, sqlx::FromRow)]
pub struct Task {
    pub id: String,
    pub script: String,
    pub cron: String,
    pub name: String,
    pub remark: Option<String>,
    pub max_executions: Option<u32>,
    pub auto_start: bool,
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

    pub async fn start(&mut self) -> anyhow::Result<()> {
        let tasks = crate::task::Task::get_all_task().await?;
        for task in tasks.into_iter().filter(|task| task.auto_start) {
            let (tx, rx) = tokio::sync::oneshot::channel::<()>();

            let runner = TaskRunner::new(task.clone(), Some(tx));

            let log_id = task.id.clone();
            let log_task = task.clone();

            self.tasks.insert(task.id.clone(), runner);
            self.runtime.spawn(async move {
                tokio::select! {
                result = task.run() => {
                    match result {
                        Ok(_) => {
                            info!("{} auto stop", task.id)
                        },
                        Err(e) => {
                            println!("{}", e);
                            error!("{} auto stop, error: {}", task.id, e)
                        }
                    }
                },
                _ = rx => {
                    info!("{} manual stop", task.id)
                },
                }
            });
            info!(
                "Task {}-[{}]:{} ---- starting",
                log_id, log_task.script, log_task.name
            )
        }
        info!("TaskManger started");
        Ok(())
    }

    pub fn stop_all(&mut self) -> anyhow::Result<()> {
        for (_, runner) in self.tasks.iter_mut() {
            runner.stop()?;
        }
        Ok(())
    }

    pub fn add_task(&mut self, task: Task) -> anyhow::Result<String> {
        let task_block = task.clone();

        let id = block_in_place(move || block_on(async { task_block.insert().await }))?;

        let id_c = id.clone();

        let mut runner = TaskRunner::new(task.clone(), None);

        if task.clone().auto_start {
            let (tx, rx) = tokio::sync::oneshot::channel::<()>();
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
            runner.tx = Some(tx);
        } else {
            runner.tx = None;
        }

        self.tasks.insert(id_c.clone(), runner);

        Ok(id_c)
    }

    pub fn start_task(&mut self, id: &String) -> anyhow::Result<()> {
        let runner = self
            .tasks
            .get_mut(id)
            .ok_or(anyhow::anyhow!("Task Not Found"))?;

        if runner.tx.is_some() {
            return Err(anyhow::anyhow!("Task is running, please stop it first"));
        }

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

    pub fn stop_task(&mut self, id: &String) -> anyhow::Result<()> {
        let runner = self
            .tasks
            .get_mut(id)
            .ok_or(anyhow::anyhow!("Task Not Found"))?;

        if runner.tx.is_some() {
            runner.stop()?;
        } else {
            return Err(anyhow::anyhow!(
                "Task is not running, please start it first"
            ));
        }
        runner.stop()?;
        Ok(())
    }

    pub fn update_task(&mut self, id: &String, task: Task) -> anyhow::Result<()> {
        let block_task = task.clone();
        block_in_place(move || block_on(async { block_task.update(id).await }))?;
        let running = self.get_running_status(id);

        if running {
            self.stop_task(id)?;
        }

        self.tasks
            .insert(id.to_string(), TaskRunner::new(task.clone(), None));

        if running {
            self.start_task(id)?;
        }

        Ok(())
    }

    pub fn remove_task(&mut self, id: &String) -> anyhow::Result<()> {
        self.stop_task(id)?;
        self.tasks.remove(id);
        block_in_place(move || block_on(async { Task::remove(&String::from(id)).await }))?;
        Ok(())
    }

    pub fn restart_task(&mut self, id: &String) -> anyhow::Result<()> {
        self.stop_task(id)?;
        self.start_task(&String::from(id))?;
        Ok(())
    }

    pub fn get_task(&self, id: &String) -> anyhow::Result<Task> {
        let runner = self
            .tasks
            .get(id)
            .ok_or(anyhow::anyhow!("Task Not Found"))?;
        Ok(runner.task.clone())
    }

    pub fn get_all_task(&self) -> anyhow::Result<Vec<(bool, Task)>> {
        let mut tasks = Vec::new();
        for (_, runner) in self.tasks.iter() {
            tasks.push((runner.tx.is_some(), runner.task.clone()));
        }
        Ok(tasks)
    }

    pub fn get_running_status(&self, id: &String) -> bool {
        self.tasks.get(id).is_some_and(|f| f.tx.is_some())
    }
}

impl TaskRunner {
    fn new(task: Task, tx: Option<Sender<()>>) -> Self {
        Self { tx, task }
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        if let Some(tx) = self.tx.take() {
            tx.send(()).unwrap();
        }

        Ok(())
    }
}

impl Task {
    /// 执行脚本
    /// let _ = script.exec().unwrap();
    /// let _ = script.call::<_, mlua::Value>(()).unwrap();
    /// let _ = script.eval::<mlua::Value>().unwrap();
    pub async fn run(&self) -> anyhow::Result<()> {
        let schedule = Schedule::from_str(self.cron.as_str()).unwrap();

        let lua = get_lua();

        let script_content = tokio::fs::read_to_string(format!("./script/{}", self.script)).await?;

        if let Some(max_executions) = self.max_executions {
            for _ in 0..max_executions {
                let now = chrono::offset::Local::now();
                let next = schedule
                    .upcoming(chrono::offset::Local)
                    .next()
                    .context("No upcoming dates")?;
                let duration = (next - now).to_std()?;
                tokio::time::sleep(duration).await;
                let script = lua.load(script_content.clone());
                script.exec()?;
            }
        } else {
            loop {
                let now = chrono::offset::Local::now();
                let next = schedule
                    .upcoming(chrono::offset::Local)
                    .next()
                    .context("No upcoming dates")?;
                let duration = (next - now).to_std()?;
                tokio::time::sleep(duration).await;
                let script = lua.load(script_content.clone());
                script.exec()?;
            }
        }
        Ok(())
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

fn get_sys_config(_: &mlua::Lua, _: &TymeUserData) -> mlua::Result<TymeConfig> {
    let sys_config = crate::tyme_config.lock().clone();
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

    let tyme_package_path = crate::start_param
        .word_dir
        .clone()
        .join("script")
        .join("?.lua");

    let tyme_sys_package_path = std::env::current_dir().unwrap().join("?.lua");

    #[cfg(target_os = "windows")]
    let tyme_package_cpath = crate::start_param
        .word_dir
        .clone()
        .join("script")
        .join("?.dll");

    #[cfg(not(target_os = "windows"))]
    let tyme_package_cpath = crate::start_param
        .word_dir
        .clone()
        .join("script")
        .join("?.so");

    let package_path = format!(
        "{};{};{}",
        package_path,
        tyme_sys_package_path.display(),
        tyme_package_path.display()
    );
    let package_cpath = format!("{};{}", package_cpath, tyme_package_cpath.display());

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
