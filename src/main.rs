use config::TymeConfig;
use flexi_logger::{
    colored_detailed_format, Age, Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming,
    WriteMode,
};

#[macro_use]
extern crate lazy_static;
extern crate mime;
extern crate serde_json;
extern crate sqlx;

mod args;
// mod clint;
mod config;
mod db;
mod header;
mod message;
mod mqtt;
// mod subscribe;
mod task;
mod web_console;

pub use args::START_PARAM as start_param;
pub use config::TYME_CONFIG as tyme_config;
pub use db::DB_POOL as db_pool;
pub use header::HEADERS as headers;
pub use task::TASK_MANGER as task_manger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if start_param.init {
        TymeConfig::initial().unwrap();
    } else {
        log_init()?;

        if tyme_config.lock().first_start {
            web_console::run_guide_web_console().await?;
        }

        let (send_msg_tx, send_msg_rx) =
            tokio::sync::mpsc::unbounded_channel::<message::SendMessage>();

        let (rec_msg_tx, _) =
            tokio::sync::broadcast::channel::<(header::Header, message::RecMessage)>(0);

        db::db_init().await?;

        tokio::select! {
            res = mqtt::run_mqtt_clint(send_msg_rx, rec_msg_tx.clone()) => {
                match res {
                    Ok(_) => {}
                    Err(err) => {
                        log::error!("Mqtt Run Error:{}", err);
                        std::process::exit(1);
                    }
                }
            },
            res = web_console::run_web_console(send_msg_tx,rec_msg_tx.clone()) => {
                match res {
                    Ok(_) => {}
                    Err(err) => {
                        log::error!("WebConsole Error:{}", err);
                        std::process::exit(1);
                    }
                }
            },
            _= tokio::signal::ctrl_c() => {}
        };
    }

    Ok(())
}

fn log_init() -> anyhow::Result<()> {
    let log_location = start_param.word_dir.clone().join("log");
    if !log_location.exists() {
        std::fs::create_dir_all(&log_location)?;
    }
    let file_spec = FileSpec::default().directory(log_location);

    let _ = Logger::try_with_str("info,pago_mqtt=error,paho_mqtt_c=error")?
        .write_mode(WriteMode::BufferAndFlush)
        .log_to_file(file_spec)
        .duplicate_to_stderr(Duplicate::All)
        .format_for_stderr(colored_detailed_format)
        .format_for_stdout(colored_detailed_format)
        //https://upload.wikimedia.org/wikipedia/commons/1/15/Xterm_256color_chart.svg
        .set_palette(String::from("b196;208;28;7;8"))
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(7),
        )
        .start()?;
    Ok(())
}
