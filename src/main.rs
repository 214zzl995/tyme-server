use std::{collections::HashMap, env};

use config::SysConfig;
use flexi_logger::{
    colored_detailed_format, Age, Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming,
    WriteMode,
};

#[macro_use]
extern crate lazy_static;
extern crate mime;
extern crate serde_json;

mod clint;
mod config;
mod message;
mod r_db;
mod subscribe;
mod web_console;

pub use clint::CLINT;
pub use config::SYSCONIFG;
pub use message::{Message, MessageContent, Topic};
use tokio::signal;

lazy_static! {
    pub static ref ARGS: HashMap<String, Option<String>> = {
        let mut args = env::args();
        let mut arg_map = HashMap::new();
        while let Some(arg) = args.next() {
            if arg.starts_with("-") {
                arg_map.insert(arg, args.next());
            }
        }
        arg_map
    };
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log_init()?;
    if env::args().nth(1) == Some("init".to_string()) {
        SysConfig::initial().unwrap();
    } else {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        tokio::select! {
           _= subscribe::subscribe() => {},
           _= web_console::run_web_console() => {},
           _= ctrl_c => {}
        }

        clint::diable_connect();
    };
    Ok(())
}

fn log_init() -> anyhow::Result<()> {
    let file_spec = FileSpec::default().directory(SYSCONIFG.lock().clone().log_location);

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
