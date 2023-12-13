use crate::ARGS;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, File},
    io::Read,
    path::PathBuf,
    sync::Arc,
};

lazy_static! {
    pub static ref SYSCONIFG: Arc<Mutex<SysConfig>> =
        Arc::new(Mutex::new(SysConfig::obtain().expect("Config Error")));
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SysConfig {
    pub mqtt_config: MQTTConfig,
    pub web_console_config: WebConsoleConfig,
    pub log_location: PathBuf,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MQTTConfig {
    pub broker: String,
    pub port: i32,
    pub client_id: String,
    pub keep_alive_interval: Option<u64>,
    pub topics: Vec<String>,
    pub version: u32,
    pub lwt: Option<String>,
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

#[derive(Deserialize, Serialize, Clone)]
pub struct WebConsoleConfig {
    pub public: bool,
    pub user_name: String,
    pub password: String,
    pub port: u16,
    pub front_end_path: Option<PathBuf>,
    pub api_token: Option<String>,
}

impl SysConfig {
    fn obtain() -> anyhow::Result<Self> {
        let config_path: Option<Option<String>> = ARGS.get("-c").cloned();

        let config_file_path = if let Some(Some(path)) = config_path {
            let arg_path = PathBuf::from(path);

            if arg_path.is_dir() {
                return Err(anyhow::anyhow!("Illegal command parameter -c"));
            } else {
                arg_path
            }
        } else {
            let current_dir = env::current_dir().unwrap();
            current_dir.join("SysConig.toml")
        };

        //check exist
        if !config_file_path.exists() {
            return Err(anyhow::anyhow!("Configuration file does not exist"));
        }

        let mut str_val = String::new();

        File::open(config_file_path)
            .unwrap()
            .read_to_string(&mut str_val)
            .unwrap();

        let config: SysConfig = match toml_edit::de::from_str(&str_val) {
            Ok(config) => config,
            Err(err) => {
                return Err(anyhow::anyhow!(
                    "An error occurred while deserializing the configuration file. Reason for the error: {}",err
                ));
            }
        };
        if config.mqtt_config.ssl.enable && config.mqtt_config.ssl.trust_store.is_none() {
            return Err(anyhow::anyhow!(
                "trust_store cannot be empty when opening ssl connection"
            ));
        }

        if config.mqtt_config.auth.enable
            && (config.mqtt_config.auth.user_name.is_none()
                || config.mqtt_config.auth.password.is_none())
        {
            return  Err(anyhow::anyhow!(
                "When the identity authentication is Yes, the username and password cannot be empty."
            ));
        }

        Ok(config)
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

    pub fn get_clint_name(&self) -> String {
        format!("tyme-server-{}", self.mqtt_config.client_id)
    }
}

impl Default for SysConfig {
    fn default() -> Self {
        Self {
            mqtt_config: Default::default(),
            web_console_config: Default::default(),
            log_location: PathBuf::from("./log"),
        }
    }
}

impl Default for WebConsoleConfig {
    fn default() -> Self {
        Self {
            public: false,
            user_name: String::from("root"),
            password: nanoid::nanoid!(8),
            port: 12566,
            front_end_path: Default::default(),
            api_token: Default::default(),
        }
    }
}

impl Default for MQTTConfig {
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
            keep_alive_interval: Default::default(),
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
