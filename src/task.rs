use cron::Schedule;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, str::FromStr, sync::Arc};
use tokio::sync::oneshot::Sender;

lazy_static! {
    static ref TASK_MANGER: Arc<Mutex<TaskManager>> = Arc::new(Mutex::new(TaskManager::new()));
}

pub struct TaskManager {
    runtime: tokio::runtime::Runtime,
    tasks: HashMap<String, TaskRunner>,
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

impl TaskManager {
    pub fn new() -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        Self {
            runtime,
            tasks: HashMap::new(),
        }
    }

    pub fn start(&mut self) {
        let tasks = [Task::new(
            PathBuf::from("scrcpy/os.lua"),
            "*/5 * * * * *".to_string(),
            "os".to_string(),
            None,
        )];

        for task in tasks {
            let (tx, rx) = tokio::sync::oneshot::channel::<()>();
            let id = nanoid::nanoid!();

            let runer = TaskRunner::new(task.clone(), tx);

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
        }
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

                let script_path = "scrcpy/os.lua";
                let script_content =
                    std::fs::read_to_string(script_path).expect("Failed to read script file");
                let script = lua.load(script_content.clone());
                let scrcpy_out = script.eval::<Option<mlua::Value>>().unwrap();

                if let Some(scrcpy_out) = scrcpy_out {
                    if scrcpy_out.is_table() {
                        let scrcpy_json = serde_json::to_string(&scrcpy_out).unwrap();
                        println!("{}", scrcpy_json);
                    } else {
                        println!("{:?}", scrcpy_out);
                    }
                } else {
                    println!("None");
                }
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
