use futures::executor::block_on;

use sqlx::{
    migrate::{MigrateDatabase, Migrator},
    mysql::MySqlPoolOptions,
    MySql, Pool,
};

use crate::{header::Header, message::RecMessage, task::Task, tyme_config, web_console::PageParam};

lazy_static! {
    static ref POOL: Pool<MySql> = {
        let config = tyme_config.lock().clone();

        let pool: anyhow::Result<Pool<MySql>> = block_on(async {
            if !MySql::database_exists(&config.database).await? {
                MySql::create_database(&config.database).await?;
            }
            let pool = MySqlPoolOptions::new()
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
    let migrate: Migrator = sqlx::migrate!();

    migrate.run(&*POOL).await?;
    Ok(())
}

pub async fn get_msg_by_id(id: &str) -> anyhow::Result<Option<RecMessage>> {
    let msg:Option<RecMessage> = sqlx::query_as(
        r#"select m.id,m.topic,m.qos,m.retain,m.mine,m.timestamp,m.sender,m.receiver,m.type,m.raw,m.html from message m where m.id = ?"#
         ).bind(id)
        .fetch_optional(&*POOL)
        .await?;

    Ok(msg)
}

impl RecMessage {
    pub async fn insert(&self, header_id: &String) -> anyhow::Result<String> {
        let id = nanoid::nanoid!();
        sqlx::query(r#"
        insert into message(id, topic, qos, retain, mine, sender, receiver, type, raw, html, header_id) values (?,?,?,?,?,?,?,?,?,?,?)
        "#).bind(&id)
        .bind(&self.topic)
        .bind(&self.qos)
        .bind(&self.retain)
        .bind(&self.mine)
        .bind(&self.sender)
        .bind(&self.receiver)
        .bind(&self.content.message_type)
        .bind(&self.content.raw)
        .bind(&self.content.html)
        .bind(&header_id)
        .execute(&*POOL).await?;

        Ok(id)
    }

    pub async fn get_msg_by_header(header_id: &str) -> anyhow::Result<Vec<RecMessage>> {
        let msgs:Vec<RecMessage> = sqlx::query_as(
        r#"select m.id,m.topic,m.qos,m.retain,m.mine,m.timestamp,m.sender,m.receiver,m.type,m.raw,m.html from message m,header h where m.header_id = h.id and h.id = ? order by timestamp desc"#
         ).bind(header_id)
        .fetch_all(&*POOL)
        .await?;
        Ok(msgs)
    }

    pub async fn get_msg_count_by_header(header_id: &str) -> anyhow::Result<i64> {
        let count: (i64,) = sqlx::query_as(
            r#"select count(*) from message m,header h where m.header_id = h.id and h.id = ?"#,
        )
        .bind(header_id)
        .fetch_one(&*POOL)
        .await?;
        Ok(count.0)
    }

    pub async fn get_page_msg_by_header(
        header_id: &str,
        page_param: &PageParam,
    ) -> anyhow::Result<Vec<RecMessage>> {
        let msgs:Vec<RecMessage> = sqlx::query_as(
        r#"select m.id,m.topic,m.qos,m.retain,m.mine,m.timestamp,m.sender,m.receiver,m.type,m.raw,m.html from message m,header h where m.header_id = h.id and h.id = ? order by timestamp desc limit ? offset ?"#
         ).bind(header_id)
        .bind(page_param.page_size as i64)
        .bind((page_param.page_size * page_param.page_num) as i64)
        .fetch_all(&*POOL).await?;
        Ok(msgs)
    }
}

impl Task {
    pub async fn insert(&self) -> anyhow::Result<String> {
        let id = nanoid::nanoid!();
        sqlx::query(r#"insert into task (id, script, cron, name, remark, max_executions, auto_start) values ( ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(&id)
            .bind(&self.script)
            .bind(&self.cron)
            .bind(&self.name)
            .bind(&self.remark)
            .bind(&self.max_executions)
            .bind(&self.auto_start)
            .execute(&*POOL)
            .await?;

        Ok(id)
    }

    pub async fn remove(id: &String) -> anyhow::Result<()> {
        sqlx::query(r#"delete from task where id = ?"#)
            .bind(id)
            .execute(&*POOL)
            .await?;
        Ok(())
    }

    pub async fn update(&self, id: &String) -> anyhow::Result<()> {
        sqlx::query(r#"update task set script = ?, cron = ?, name = ?, remark = ?, max_executions = ?, auto_start = ? where id = ?"#)
            .bind(&self.script)
            .bind(&self.cron)
            .bind(&self.name)
            .bind(&self.remark)
            .bind(&self.max_executions)
            .bind(&self.auto_start)
            .bind(id)
            .execute(&*POOL)
            .await?;
        Ok(())
    }

    pub async fn get_all_task() -> anyhow::Result<Vec<Task>> {
        let tasks = sqlx::query_as(r#"select t.id,t.script,t.cron,t.name,t.remark,t.max_executions,t.auto_start from task t"#)
            .fetch_all(&*POOL)
            .await?;
        Ok(tasks)
    }
}

impl Header {
    pub async fn _insert(&self) -> anyhow::Result<String> {
        let id = nanoid::nanoid!();
        sqlx::query(r#"insert into header (id, topic, qos) values (?, ?, ?)"#)
            .bind(&id)
            .bind(&self.topic)
            .bind(&self.qos)
            .execute(&*POOL)
            .await?;
        Ok(id)
    }

    pub async fn _remove(id: &String) -> anyhow::Result<()> {
        sqlx::query(r#"delete from header where id = ?"#)
            .bind(id)
            .execute(&*POOL)
            .await?;
        Ok(())
    }

    pub async fn _update(&self, id: &String) -> anyhow::Result<()> {
        sqlx::query(r#"update header set topic = ?, qos = ? where id = ?"#)
            .bind(&self.topic)
            .bind(&self.qos)
            .bind(id)
            .execute(&*POOL)
            .await?;
        Ok(())
    }

    pub async fn get_db_headers() -> anyhow::Result<Vec<Header>> {
        let headers = sqlx::query_as(r#"select h.id,h.topic,h.qos from header h"#)
            .fetch_all(&*POOL)
            .await?;
        Ok(headers)
    }
}
