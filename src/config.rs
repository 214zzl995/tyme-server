use crate::{message::Message, ARGS};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::Read,
    path::PathBuf,
    process,
};

lazy_static! {
    pub static ref SYSCONIFG: SysConfig = SysConfig::obtain();
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SysConfig {
    pub broker: String,
    pub port: i32,
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub topics:Vec<String>,
    pub version: u32,
    pub lwt: Option<Message>,
    pub auth: Auth,
    pub ssl: Ssl,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Auth {
    pub enable: bool,
    pub user_name: Option<String>,
    pub password: Option<String>,
}

/// can watch paho_mqtt::SslOptions
#[derive(Deserialize, Serialize, Clone)]
pub struct Ssl {
    pub enable: bool,
    pub trust_store: Option<PathBuf>,
    pub key_store: Option<PathBuf>,
    pub private_key: Option<PathBuf>,
    pub private_key_password: Option<String>,
    pub ca_path: Option<PathBuf>,
    pub protos: Option<Vec<String>>,
}

impl SysConfig {
    fn obtain() -> Self {
        let config_path: Option<Option<String>> = ARGS.get("-c").cloned();

        let config_file_path = if let Some(Some(path)) = config_path {
            let arg_path = PathBuf::from(path);

            if arg_path.is_dir() {
                eprintln!("Illegal command parameter -c");
                process::exit(1);
            } else {
                arg_path
            }
        } else {
            let current_dir = env::current_dir().unwrap();
            current_dir.join("SysConig.toml")
        };

        //check exist
        if !config_file_path.exists() {
            eprintln!("Configuration file does not exist");
            process::exit(1);
        }

        let mut str_val = String::new();

        File::open(config_file_path)
            .unwrap()
            .read_to_string(&mut str_val)
            .unwrap();

        let config: SysConfig = match toml_edit::de::from_str(&str_val) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("An error occurred while deserializing the configuration file. Reason for the error: {}",err);
                process::exit(1);
            }
        };
        if config.ssl.enable && config.ssl.trust_store.is_none() {
            eprintln!("trust_store cannot be empty when opening ssl connection");
            process::exit(1);
        }

        if config.auth.enable && (config.auth.user_name.is_none() || config.auth.password.is_none())
        {
            eprintln!("When the identity authentication is Yes, the username and password cannot be empty.");
            process::exit(1);
        }

        config
    }

    ///Generate initial config file
    pub fn initial() -> anyhow::Result<()> {
        let current_dir = env::current_dir()?;
        let conf = current_dir.join("SysConig.toml");

        if !conf.exists() {
            match File::create(&conf) {
                Ok(_) => {
                    let config = SysConfig::default();

                    let config_str = toml_edit::ser::to_string_pretty(&config).unwrap();

                    fs::write(&conf, config_str).unwrap();
                }
                Err(err) => panic!("Unable to add app profile exception:{}", err),
            }
        }
        Ok(())
    }
}

impl Default for SysConfig {
    fn default() -> Self {
        Self {
            broker: Default::default(),
            port: Default::default(),
            client_id: Default::default(),
            version: Default::default(),
            lwt: Default::default(),
            auth: Default::default(),
            ssl: Default::default(),
            topics: Default::default(),
        }
    }
}

impl Default for Auth {
    fn default() -> Self {
        Self {
            enable: Default::default(),
            user_name: Default::default(),
            password: Default::default(),
        }
    }
}

impl Default for Ssl {
    fn default() -> Self {
        Self {
            enable: Default::default(),
            trust_store: Default::default(),
            key_store: Default::default(),
            private_key: Default::default(),
            private_key_password: Default::default(),
            ca_path: Default::default(),
            protos: Default::default(),
        }
    }
}
