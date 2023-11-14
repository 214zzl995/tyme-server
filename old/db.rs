use crate::message::Message;
use bincode::config::Configuration;
use parking_lot::Mutex;

//sled = "1.0.0-alpha.118"
//bincode = { version = "2.0.0-rc.3", features = ["serde"] }

lazy_static! {
    static ref DB: Mutex<sled::Db> = {
        let config = sled::Config::default()
            .flush_every_ms(Some(1000))
            .path(std::path::Path::new("data"));
        let db = config.open().unwrap();
        Mutex::new(db)
    };
}

pub fn get_msg_by_id(id: String) -> anyhow::Result<Message> {
    let msg = DB
        .lock()
        .get(id.as_bytes())?
        .unwrap_or(Err(anyhow::anyhow!("not found"))?);

    let msg =
        bincode::decode_from_slice::<Message, Configuration>(&msg, bincode::config::standard())?.0;
    Ok(msg)
}

pub fn insert_msg(msg: Message) -> anyhow::Result<()> {
    let id = msg.id.clone().unwrap();
    let mut slice = [0u8; 100];
    let _ = bincode::encode_into_slice::<Message, Configuration>(
        msg,
        &mut slice,
        bincode::config::standard(),
    )?;
    let msg = sled::InlineArray::from(&slice);
    DB.lock().insert(id.as_bytes(), &msg)?;
    Ok(())
}
