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
        info!("TaskManger starting");
        for (id, task) in tasks {
            let (tx, rx) = tokio::sync::oneshot::channel::<()>();

            let runer = TaskRunner::new(task.clone(), tx);

            let log_id = id.clone();
            let log_task = task.clone();

            self.tasks.insert(id.clone(), runer);
            self.runtime.spawn(async move {
                tokio::select! {
                    _ = task.run() => {
                        println!("{} auto stop", id);
                    },
                    _ = rx => {
                        println!("{} manual stop", id);
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

        let runer = TaskRunner::new(task.clone(), tx);

        self.tasks.insert(id.clone(), runer);
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
        crate::r_db::remove_task(&String::from(id))?;
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
                        println!("{} auto stop", id);
                    },
                    _ = rx => {
                        println!("{} manual stop", id);
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
            // Arc::try_unwrap(tx).unwrap().send(()).unwrap();
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

        loop {
            {
                let lua = mlua::Lua::new();

                let globals = lua.globals();

                let package_path = globals
                    .get::<_, mlua::Table>("package")
                    .unwrap()
                    .get::<_, String>("path")
                    .unwrap();

                let package_path = format!("{};./scrcpy/?.lua", package_path);

                globals
                    .get::<_, mlua::Table>("package")
                    .unwrap()
                    .set("path", package_path)
                    .unwrap();

                let print_person = lua
                    .create_function(|_, (name, age): (String, u8)| {
                        println!("{} is {} years old!", name, age);
                        Ok(())
                    })
                    .unwrap();
                globals.set("print_person", print_person).unwrap();

                let script_content =
                    std::fs::read_to_string(self.path.clone()).expect("Failed to read script file");
                let script = lua.load(script_content.clone());

                script.exec().unwrap();
                
                // let scrcpy_out = script.eval::<Option<mlua::Value>>().unwrap();

                // if let Some(scrcpy_out) = scrcpy_out {
                //     if scrcpy_out.is_table() {
                //         let scrcpy_json = serde_json::to_string(&scrcpy_out).unwrap();
                //         println!("{}", scrcpy_json);
                //     } else {
                //         println!("{:?}", scrcpy_out);
                //     }
                // } else {
                //     println!("None");
                // }
            }
            let now = chrono::offset::Local::now();
            let next = schedule.upcoming(chrono::offset::Local).next().unwrap();
            let duration = (next - now).to_std().unwrap();

            tokio::time::sleep(duration).await;
        }

        // 加载一个 Lua 脚本文件
    }
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
        PathBuf::from("scrcpy/test.lua"),
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
