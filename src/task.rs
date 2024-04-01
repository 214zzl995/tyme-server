use anyhow::Context;
use cron::Schedule;

use linked_hash_map::LinkedHashMap;
use log::{error, info};
use mlua::Lua;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, str::FromStr, sync::Arc};
use tokio::sync::oneshot::Sender;

use crate::config::TymeConfig;

#[derive(Clone)]
pub struct TaskManager {
    send_msg_tx: tokio::sync::mpsc::UnboundedSender<crate::message::SendMessage>,
    inner: Arc<Mutex<LinkedHashMap<String, TaskRunner>>>,
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

impl TaskManager {
    pub fn new(
        send_msg_tx: tokio::sync::mpsc::UnboundedSender<crate::message::SendMessage>,
    ) -> Self {
        Self {
            send_msg_tx,
            inner: Arc::new(Mutex::new(LinkedHashMap::new())),
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let tasks = Task::get_all_task().await?;
        for task in tasks.into_iter().filter(|task| task.auto_start) {
            
            let (tx, rx) = tokio::sync::oneshot::channel::<()>();

            let runner = TaskRunner::new(task.clone(), Some(tx));

            let log_id = task.id.clone();
            let log_task = task.clone();

            self.inner.lock().insert(task.id.clone(), runner);
            let lua = get_lua(self.send_msg_tx.clone());
            tokio::spawn(async move {
                tokio::select! {
                result = task.run(lua) => {
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

    pub fn stop_all(&self) -> anyhow::Result<()> {
        for (_, runner) in self.inner.lock().iter_mut() {
            runner.stop()?;
        }
        Ok(())
    }

    pub async fn add_task(&self, task: Task) -> anyhow::Result<String> {
        let id = task.insert().await?;

        let id_c = id.clone();

        let mut runner = TaskRunner::new(task.clone(), None);

        if task.clone().auto_start {
            let (tx, rx) = tokio::sync::oneshot::channel::<()>();
            let lua = get_lua(self.send_msg_tx.clone());
            tokio::spawn(async move {
                tokio::select! {
                    result = task.run(lua) => {
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

        self.inner.lock().insert(id_c.clone(), runner);

        Ok(id_c)
    }

    pub fn start_task(&self, id: &String) -> anyhow::Result<()> {
        let mut runner = self.inner.lock();
        let runner = runner
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

            let lua = get_lua(self.send_msg_tx.clone());
            tokio::spawn(async move {
                tokio::select! {
                result = task.run(lua) => {
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

    pub fn stop_task(&self, id: &String) -> anyhow::Result<()> {
        let mut runner = self.inner.lock();
        let runner = runner
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

    pub async fn update_task(&self, id: &String, task: Task) -> anyhow::Result<()> {
        task.update(id).await?;
        let running = self.get_running_status(id);

        if running {
            self.stop_task(id)?;
        }

        self.inner
            .lock()
            .insert(id.to_string(), TaskRunner::new(task.clone(), None));

        if running {
            self.start_task(id)?;
        }

        Ok(())
    }

    pub async fn remove_task(&self, id: &String) -> anyhow::Result<()> {
        self.stop_task(id)?;
        self.inner.lock().remove(id);

        Task::remove(&String::from(id)).await?;
        Ok(())
    }

    pub fn restart_task(&self, id: &String) -> anyhow::Result<()> {
        self.stop_task(id)?;
        self.start_task(&String::from(id))?;
        Ok(())
    }

    pub fn get_task(&self, id: &String) -> anyhow::Result<Task> {
        let mut runner = self.inner.lock();
        let runner = runner
            .get_mut(id)
            .ok_or(anyhow::anyhow!("Task Not Found"))?;
        Ok(runner.task.clone())
    }

    pub fn get_all_task(&self) -> anyhow::Result<Vec<(bool, Task)>> {
        let mut tasks = Vec::new();
        for (_, runner) in self.inner.lock().deref().iter() {
            tasks.push((runner.tx.is_some(), runner.task.clone()));
        }
        Ok(tasks)
    }

    pub fn get_running_status(&self, id: &String) -> bool {
        self.inner
            .lock()
            .deref()
            .get(id)
            .is_some_and(|f| f.tx.is_some())
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
    pub async fn run(&self, lua: Lua) -> anyhow::Result<()> {
        let schedule = Schedule::from_str(self.cron.as_str()).unwrap();

        let script_path = crate::start_param
            .word_dir
            .clone()
            .join("script")
            .join(&self.script);

        let script_content = tokio::fs::read_to_string(script_path).await?;

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

struct TymeUserData {
    send_msg_tx: tokio::sync::mpsc::UnboundedSender<crate::message::SendMessage>,
}

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
    tyme_user_data: &TymeUserData,
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
    tyme_user_data.send_msg_tx.send(msg).unwrap();
    Ok(())
}

async fn lua_send_markdown(
    _: &mlua::Lua,
    tyme_user_data: &TymeUserData,
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

    tyme_user_data.send_msg_tx.send(msg).unwrap();

    Ok(())
}

fn get_lua(
    send_msg_tx: tokio::sync::mpsc::UnboundedSender<crate::message::SendMessage>,
) -> mlua::Lua {
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

    let tyme_user_data = TymeUserData { send_msg_tx };

    lua.globals().set("tyme_sys", tyme_user_data).unwrap();

    lua
}
