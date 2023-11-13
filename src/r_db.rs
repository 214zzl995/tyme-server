use anyhow::Context;
use rocksdb::{DBWithThreadMode, IteratorMode, MultiThreaded, Options};

use crate::Message;

lazy_static! {
    pub static ref RDB: DBWithThreadMode<MultiThreaded> = {
        let path = "data";
        let mut db_opts = Options::default();
        db_opts.create_missing_column_families(true);
        db_opts.create_if_missing(true);
        db_opts.set_max_write_buffer_number(16);

        let cfs = crate::SYSCONIFG.lock().clone().mqtt_config.topics;
    
        rocksdb::DB::open_cf(&db_opts, path,cfs).unwrap()
    };
}
pub fn get_msg_by_header_with_id(topic_name: &String,id:&String) -> Option<Message> {
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
            return Some(msg)
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
            let msg = bincode::deserialize::<Message>(&msg).unwrap();
            msg
        })
        .collect::<Vec<Message>>();

    let _ = msgs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

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
    let msg = bincode::serialize::<Message>(&msg)?;

    RDB.put_cf(&header, id, msg)?;

    Ok(())
}
