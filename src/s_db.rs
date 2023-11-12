use anyhow::Context;

use crate::message::Message;

lazy_static! {
    static ref DB: sled::Db = {
        let config = sled::Config::default()
            .flush_every_ms(Some(1000))
            .cache_capacity(13_1072)
            .path(std::path::Path::new("data"));
        let db = config.open().unwrap();
        db
    };
}

pub fn _get_msg_by_id(id: &String) -> anyhow::Result<Message> {
    let id = id.as_bytes();
    println!("{:?}", id);
    let msg = DB.get(id)?.context("not found")?;

    let msg = bincode::deserialize(&msg)?;

    Ok(msg)
}

pub fn get_msg_by_topic_name(topic_name: &String) -> anyhow::Result<Vec<Message>> {
    let topic_tree = DB.open_tree(topic_name).unwrap();

    let mut msgs = topic_tree
        .iter()
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
    let topic_tree_name = msg.topic.header.clone().context("Message not found header")?;
    let topic_tree = DB.open_tree(topic_tree_name).unwrap();

    let id = msg.id.clone().unwrap();
    let id = id.as_bytes();
    let msg = bincode::serialize::<Message>(&msg)?;
    let msg = sled::IVec::from_iter(msg);

    topic_tree.insert(id, msg)?;

    Ok(())
}
