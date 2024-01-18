use std::sync::Arc;

use anyhow::Context;
use rocksdb::{
    BoundColumnFamily, DBWithThreadMode, IteratorMode, LogLevel, MultiThreaded, Options,
};

use crate::{task::Task, Message};
const TASK_CF_NAME: &str = "tasks";

lazy_static! {
    pub static ref RDB: DBWithThreadMode<MultiThreaded> = {
        let path = "data";
        let mut db_opts = Options::default();
        db_opts.create_missing_column_families(true);
        db_opts.create_if_missing(true);
        db_opts.set_max_write_buffer_number(16);
        db_opts.set_disable_auto_compactions(true);
        db_opts.set_keep_log_file_num(5);
        db_opts.set_log_level(LogLevel::Warn);

        let mut cfs = crate::sys_config.lock().clone().mqtt_config.topics;

        cfs.push(TASK_CF_NAME.to_string());

        rocksdb::DB::open_cf(&db_opts, path, cfs).unwrap()
    };
}
pub fn get_msg_by_header_with_id(topic_name: &String, id: &String) -> Option<Message> {
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
            let msg = bincode::deserialize::<Message>(&msg).unwrap();
            Some(msg)
        }
        None => None,
    }
}

pub fn get_msg_by_header_name(topic_name: &String) -> anyhow::Result<Vec<Message>> {
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
            
            bincode::deserialize::<Message>(&msg).unwrap()
        })
        .collect::<Vec<Message>>();

    msgs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    Ok(msgs)
}

pub fn insert_msg(msg: &Message) -> anyhow::Result<()> {
    let cf_name = msg
        .topic
        .header
        .clone()
        .context("Message not found header")?;
    let cf_options = Options::default();

    let header = match RDB.cf_handle(cf_name.clone().as_str()) {
        Some(h) => h,
        None => {
            RDB.create_cf(cf_name.clone(), &cf_options)?;
            RDB.cf_handle(cf_name.clone().as_str())
                .context("Message not found header")?
        }
    };

    let id = msg.id.clone().unwrap();
    let id = id.as_bytes();
    let msg = bincode::serialize::<Message>(msg)?;

    RDB.put_cf(&header, id, msg)?;

    Ok(())
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

pub fn get_all_tasks() -> anyhow::Result<Vec<(String, Task)>> {
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

pub fn add_task(task: &Task) -> anyhow::Result<String> {
    let header = get_task_header()?;

    let id = nanoid::nanoid!();
    let id_b = id.as_bytes();
    let task = bincode::serialize::<Task>(task)?;

    RDB.put_cf(&header, id_b, task)?;

    Ok(id)
}

pub fn remove_task(id: &String) -> anyhow::Result<()> {
    let header = get_task_header()?;

    let id_b = id.as_bytes();

    RDB.delete_cf(&header, id_b)?;

    Ok(())
}
