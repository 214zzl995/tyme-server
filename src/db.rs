use futures::executor::block_on;

use sqlx::{
    any::{install_default_drivers, AnyPoolOptions},
    migrate::{MigrateDatabase, Migrator},
    Any, Pool,
};

use crate::{header::Header, message::RecMessage, task::Task, tyme_config};

static MIGRATOR: Migrator = sqlx::migrate!();

lazy_static! {
    pub static ref DB_POOL: Pool<Any> = {
        let config = tyme_config.lock().clone();

        install_default_drivers();

        let pool: anyhow::Result<Pool<Any>> = block_on(async {
            if !Any::database_exists(&config.database).await? {
                Any::create_database(&config.database).await?;
            }
            let pool = AnyPoolOptions::new()
                .max_connections(5)
                .connect(&config.database)
                .await?;

            Ok(pool)
        });
        match pool {
            Ok(pool) => pool,
            Err(err) => {
                log::error!("Error creating the database pool: {}", err);
                std::process::exit(1);
            }
        }
    };
}

pub async fn db_init() -> anyhow::Result<()> {
    MIGRATOR.run(&*DB_POOL).await?;
    Ok(())
}

pub async fn get_msg_by_id(id: &str) -> anyhow::Result<Option<RecMessage>> {
    // let msg:Option<RecMessage> = sqlx::query_as(
    //     r#"select m.id,m.topic,m.qos,m.retain,m.mine,m.timestamp,m.sender,m.receiver,m.type,m.raw,m.html from message m where m.id = ?"#
    //      ).bind(id)
    //     .fetch_optional(&*DB_POOL)
    //     .await?;
    Ok(None)
}

pub async fn get_msg_by_header(header_id: &str) -> anyhow::Result<Vec<RecMessage>> {
    // let msgs = sqlx::query!("SELECT * FROM message WHERE header_id = ?", header_id)
    //     .fetch_all(&*DB_POOL)
    //     .await?;
    Ok(vec![])
}

impl RecMessage {
    pub async fn insert(&self, header: &Header) -> anyhow::Result<()> {
        Ok(())
    }
}

pub async fn get_all_task() -> anyhow::Result<Vec<(String, Task)>> {
    Ok(vec![])
}

impl Task {
    pub async fn insert(&self) -> anyhow::Result<String> {
        Ok("".to_string())
    }

    pub async fn remove(id: &String) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn update(&self, id: &String) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Header {
    pub fn insert(&self) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn remove(id: &String) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn update(&self, id: &String) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn get_all_msg(&self) -> anyhow::Result<Vec<RecMessage>> {
        Ok(vec![])
    }

    pub fn get_all_header() -> anyhow::Result<Vec<Header>> {
        let mut headers = vec![];
        headers.push(Header {
            id: None,
            topic: "system/#".to_string(),
            qos: 2,
        });
        Ok(headers)
    }
}
