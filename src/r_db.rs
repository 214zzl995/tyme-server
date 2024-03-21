use std::sync::Arc;

use anyhow::Context;
use rocksdb::{
    BoundColumnFamily, DBWithThreadMode, IteratorMode, LogLevel, MultiThreaded, Options,
};

use crate::{config::Header, message::RecMessage, task::Task};
const TASK_CF_NAME: &str = "tasks";

lazy_static! {
    pub static ref RDB: DBWithThreadMode<MultiThreaded> = {
        let path = "data";
        let mut db_opts = Options::default();
        db_opts.create_missing_column_families(true);
        db_opts.create_if_missing(true);
        db_opts.set_max_write_buffer_number(16);
        db_opts.set_disable_auto_compactions(true);
        db_opts.set_keep_log_file_num(1);
        db_opts.set_log_level(LogLevel::Warn);

        //可以直接使用 topic存储 当加入同名topic时  直接修改同名topic的值
        let mut cfs = crate::sys_config.lock().mqtt_config.get_topics_string();

        cfs.push(TASK_CF_NAME.to_string());

        rocksdb::DB::open_cf(&db_opts, path, cfs).unwrap()
    };
}
pub fn get_msg_by_header_with_id(topic_name: &String, id: &String) -> Option<RecMessage> {
    let cf_name = topic_name.clone();
    let cf_options = Options::default();

    let header = match RDB.cf_handle(cf_name.clone().as_str()) {
        Some(h) => h,
        None => {
            RDB.create_cf(cf_name.clone(), &cf_options).unwrap();
            RDB.cf_handle(cf_name.clone().as_str()).unwrap()
        }
    };

    let id = id.as_bytes();

    match RDB.get_cf(&header, id).unwrap() {
        Some(msg) => {
            let msg = bincode::deserialize::<RecMessage>(&msg).unwrap();
            Some(msg)
        }
        None => None,
    }
}

pub fn get_msg_by_header_name(topic_name: &String) -> anyhow::Result<Vec<RecMessage>> {
    let cf_name = topic_name.clone();
    let cf_options = Options::default();

    let header = match RDB.cf_handle(cf_name.clone().as_str()) {
        Some(h) => h,
        None => {
            RDB.create_cf(cf_name.clone(), &cf_options)?;
            RDB.cf_handle(cf_name.clone().as_str())
                .context("Message not found header")?
        }
    };

    let mut msgs = RDB
        .full_iterator_cf(&header, IteratorMode::Start)
        .map(|x| {
            let (_, msg) = x.unwrap();

            bincode::deserialize::<RecMessage>(&msg).unwrap()
        })
        .collect::<Vec<RecMessage>>();

    msgs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    Ok(msgs)
}

impl RecMessage {
    pub fn insert(&self, header: &Header) -> anyhow::Result<()> {
        let cf_name = header.topic.clone();
        let cf_options = Options::default();

        let header = match RDB.cf_handle(cf_name.clone().as_str()) {
            Some(h) => h,
            None => {
                RDB.create_cf(cf_name.clone(), &cf_options)?;
                RDB.cf_handle(cf_name.clone().as_str())
                    .context("Message not found header")?
            }
        };

        let id = self.id.clone();
        let id = id.as_bytes();
        let msg = bincode::serialize::<RecMessage>(self)?;

        RDB.put_cf(&header, id, msg)?;

        Ok(())
    }
}

fn get_task_header() -> anyhow::Result<Arc<BoundColumnFamily<'static>>> {
    let cf_options = Options::default();

    let header = match RDB.cf_handle(TASK_CF_NAME) {
        Some(h) => h,
        None => {
            RDB.create_cf(TASK_CF_NAME, &cf_options)?;
            RDB.cf_handle(TASK_CF_NAME).unwrap()
        }
    };

    Ok(header)
}

pub fn get_all_task() -> anyhow::Result<Vec<(String, Task)>> {
    let header = get_task_header()?;

    let tasks = RDB
        .full_iterator_cf(&header, IteratorMode::Start)
        .map(|x| {
            let (id, task) = x.unwrap();
            let id = String::from_utf8(id.to_vec()).unwrap();
            let task = bincode::deserialize::<Task>(&task).unwrap();
            (id, task)
        })
        .collect::<Vec<(String, Task)>>();

    Ok(tasks)
}

impl Task {
    pub fn insert(&self) -> anyhow::Result<String> {
        let header = get_task_header()?;

        let id = nanoid::nanoid!();
        let id_b = id.as_bytes();
        let task = bincode::serialize::<Task>(self)?;

        RDB.put_cf(&header, id_b, task)?;

        Ok(id)
    }

    pub fn remove(id: &String) -> anyhow::Result<()> {
        let header = get_task_header()?;

    let id_b = id.as_bytes();

    RDB.delete_cf(&header, id_b)?;

    Ok(())
    }
}


pub fn _delete_all_tasks() -> anyhow::Result<()> {
    let tasks = get_all_task()?;

    let header = get_task_header()?;

    for (id, _) in tasks {
        let id_b = id.as_bytes();
        RDB.delete_cf(&header, id_b)?;
    }

    Ok(())
}
