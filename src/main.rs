use std::{collections::HashMap, env};

use config::SysConfig;

#[macro_use]
extern crate lazy_static;

mod clint;
mod config;
mod message;
mod subscribe;

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
    }else{
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.spawn(subscribe::subscribe());

        loop{}

        
    }
}
