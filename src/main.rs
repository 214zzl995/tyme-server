use std::{collections::HashMap, env};

use config::SysConfig;
use tokio::signal;

#[macro_use]
extern crate lazy_static;
extern crate serde_json;


mod clint;
mod config;
mod message;
mod subscribe;
mod web_console;
mod db;

pub use config::SYSCONIFG;
pub use message::{Message, MessageContent, MessageType, Topic};
pub use clint::CLINT;

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
async fn main() {
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
           res = web_console::run_web_console() => {
                if let Err(err) = res {
                    println!("WebConsole Error:{}", err);
                }
           },
           _= ctrl_c => {}
        }

        clint::diable_connect();
    }
}
