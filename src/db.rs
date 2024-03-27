use futures::executor::block_on;
use sqlx::{
    any::AnyPoolOptions,
    migrate::{MigrateDatabase, Migrator},
    Any, Pool,
};

use crate::{config::Header, message::RecMessage, task::Task, tyme_config};

static MIGRATOR: Migrator = sqlx::migrate!();

lazy_static! {
    pub static ref DB_POOL: Pool<Any> = {
        let config = tyme_config.lock().clone();

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

pub async fn init() -> anyhow::Result<()> {
    MIGRATOR.run(&*DB_POOL).await?;
    Ok(())
}

pub fn get_msg_by_header_with_id(topic_name: &str, id: &str) -> anyhow::Result<Option<RecMessage>> {
    Ok(None)
}

pub fn get_msg_by_header_name(topic_name: &String) -> anyhow::Result<Vec<RecMessage>> {
    Ok(vec![])
}

impl RecMessage {
    pub fn insert(&self, header: &Header) -> anyhow::Result<()> {
        Ok(())
    }
}

pub fn get_all_task() -> anyhow::Result<Vec<(String, Task)>> {
    Ok(vec![])
}

impl Task {
    pub fn insert(&self) -> anyhow::Result<String> {
        Ok("".to_string())
    }

    pub fn remove(id: &String) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn update(&self, id: &String) -> anyhow::Result<()> {
        Ok(())
    }
}
