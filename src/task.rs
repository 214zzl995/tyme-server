use cron::Schedule;
use linked_hash_map::LinkedHashMap;
use log::info;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr, sync::Arc};
use tokio::sync::oneshot::Sender;

use crate::r_db;

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
        println!("我进来了噢！");
        for (id, task) in tasks {
            let (tx, rx) = tokio::sync::oneshot::channel::<()>();

            let runner = TaskRunner::new(task.clone(), tx);

            let log_id = id.clone();
            let log_task = task.clone();

            self.tasks.insert(id.clone(), runner);
            self.runtime.spawn(async move {
                tokio::select! {
                    _ = task.run() => {
                    info!("{} auto stop", id)
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
                _ = task.run() => {
                    info!("{} auto stop", id)
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
                   _ = task.run() => {
                    info!("{} auto stop", id)
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

    pub async fn run(&self) {
        let schedule = Schedule::from_str(self.cron.as_str()).unwrap();

        let lua = get_lua();

        let script_content = tokio::fs::read_to_string(self.path.clone())
            .await
            .expect("Failed to read script file");

        loop {
            let script = lua.load(script_content.clone());
            script.exec().unwrap();

            let now = chrono::offset::Local::now();
            let next = schedule.upcoming(chrono::offset::Local).next().unwrap();
            let duration = (next - now).to_std().unwrap();

            tokio::time::sleep(duration).await;
        }
    }
}

struct TymeUserData;

impl mlua::UserData for TymeUserData {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("sys_config", |_, _| Ok("".to_string()));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_method("send_json", lua_send_json);
        methods.add_async_method("send_markdown", lua_send_markdown);
    }
}

async fn lua_send_json(
    _: &mlua::Lua,
    _: &TymeUserData,
    (topic, json): (String, mlua::Value<'_>),
) -> mlua::Result<()> {
    let json_string = serde_json::to_string(&json).unwrap();
    let topic = crate::message::Topic {
        topic,
        header: None,
    };
    let msg = crate::message::Message {
        id: None,
        topic,
        retain: None,
        qos: 1,
        mine: None,
        timestamp: None,
        content: crate::message::MessageContent {
            message_type: "application/json".to_string(),
            raw: json_string,
            html: None,
        },
        sender: Some(crate::sys_config.lock().get_clint_name()),
        receiver: None,
    };

    crate::clint::publish(msg).await.unwrap();
    Ok(())
}

async fn lua_send_markdown(
    _: &mlua::Lua,
    _: &TymeUserData,
    (topic, markdown): (String, mlua::Value<'_>),
) -> mlua::Result<()> {
    let markdown_string = markdown.to_string().unwrap();
    let topic = crate::message::Topic {
        topic,
        header: None,
    };
    let msg = crate::message::Message {
        id: None,
        topic,
        retain: None,
        qos: 1,
        mine: None,
        timestamp: None,
        content: crate::message::MessageContent {
            message_type: "text/markdown".to_string(),
            raw: markdown_string,
            html: None,
        },
        sender: Some(crate::sys_config.lock().get_clint_name()),
        receiver: None,
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
        //解锁task_manger
        parking_lot::MutexGuard::unlock_fair(task_manger);
        tokio::time::sleep(std::time::Duration::from_secs(35)).await;
    });
}

#[test]
fn add_task() {
    let tasks = Task::new(
        PathBuf::from("../script/test.lua"),
        "*/5 * * * * *".to_string(),
        "os".to_string(),
        None,
    );

    r_db::add_task(&tasks).unwrap();
}

#[test]
fn fun() {
    let lua = mlua::Lua::new();
    let print_person = lua
        .create_function(|_, (name, age): (String, u8)| {
            println!("{} is {} years old!", name, age);
            Ok(())
        })
        .unwrap();
    lua.globals().set("print_person", print_person).unwrap();

    let _ = lua
        .load(
            r#"
                print_person("John", 25)
        "#,
        )
        .exec();
}

#[test]
fn scrcpy() {
    let lua = mlua::Lua::new();

    let globals = lua.globals();

    let package_path = globals
        .get::<_, mlua::Table>("package")
        .unwrap()
        .get::<_, String>("path")
        .unwrap();

    let package_path = format!("{};./script/?.lua", package_path);

    globals
        .get::<_, mlua::Table>("package")
        .unwrap()
        .set("path", package_path)
        .unwrap();

    // 加载一个 Lua 脚本文件
    let script_path = "script/os.lua";
    let script_content = std::fs::read_to_string(script_path).expect("Failed to read script file");

    let script = lua.load(script_content);

    // 执行脚本
    // let _ = script.exec().unwrap();
    // let _ = script.call::<_, mlua::Value>(()).unwrap();
    // let _ = script.eval::<mlua::Value>().unwrap();

    let os_info = script.eval::<mlua::Value>().unwrap();

    let os_info_json = serde_json::to_string_pretty(&os_info).unwrap();

    println!("{}", os_info_json);
}
