use mlua::IntoLua;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
    sync::Arc,
};

use crate::header::Header;

lazy_static! {
    pub static ref TYME_CONFIG: Arc<Mutex<TymeConfig>> = {
        let config = match TymeConfig::obtain() {
            Ok(config) => config,
            Err(err) => {
                log::error!("Config Error:{}", err);
                std::process::exit(1)
            }
        };
        Arc::new(Mutex::new(config))
    };
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TymeConfig {
    pub mqtt_config: MQTTConfig,
    pub web_console_config: WebConsoleConfig,
    pub database: String,

    #[serde(skip)]
    pub first_start: bool,

    #[serde(skip)]
    pub config_file: PathBuf,
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct MQTTConfig {
    pub broker: String,
    pub port: i32,
    pub client_id: String,
    pub keep_alive_interval: Option<u64>,
    pub auth: Auth,
    pub ssl: Ssl,
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Auth {
    pub enable: bool,
    pub username: Option<String>,
    pub password: Option<String>,
}

/// can watch paho_mqtt::SslOptions
#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Ssl {
    pub enable: bool,
    pub trust_store: Option<String>,
    pub key_store: Option<PathBuf>,
    pub private_key: Option<PathBuf>,
    pub private_key_password: Option<String>,
    pub ca_path: Option<PathBuf>,
    pub protos: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WebConsoleConfig {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub api_token: Option<String>,
}

impl TymeConfig {
    fn obtain() -> anyhow::Result<Self> {
        let config_file = crate::start_param.get_config_file();

        if !config_file.clone().exists() {
            log::warn!(
                "No configuration file found, using default configuration,Working location is: {}",
                config_file.display()
            );

            let mut config = TymeConfig::default();
            config.config_file = config_file;
            return Ok(config);
        } else {
            log::info!("Using configuration file: {}", config_file.display());
        }

        let mut config_val = String::new();

        File::open(config_file.clone())?
            .read_to_string(&mut config_val)
            .unwrap();

        let mut config: TymeConfig = match toml_edit::de::from_str(&config_val) {
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
            && (config.mqtt_config.auth.username.is_none()
                || config.mqtt_config.auth.password.is_none())
        {
            return  Err(anyhow::anyhow!(
                "When the identity authentication is Yes, the username and password cannot be empty."
            ));
        }

        config.first_start = false;
        config.config_file = config_file;

        Ok(config)
    }

    ///Generate initial config file
    pub fn initial() -> anyhow::Result<()> {
        let conf = crate::start_param.word_dir.join("config.toml");

        if !conf.exists() {
            match File::create(&conf) {
                Ok(_) => {
                    let config = TymeConfig::default();

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

    pub async fn update(&self) -> anyhow::Result<()> {
        let config_str = toml_edit::ser::to_string_pretty(&self)?;
        let config_file = {
            let mut loc_config = TYME_CONFIG.lock();
            loc_config.mqtt_config = self.mqtt_config.clone();
            loc_config.web_console_config = self.web_console_config.clone();
            loc_config.config_file.clone()
        };

        if !config_file.exists() {
            tokio::io::AsyncWriteExt::write_all(
                &mut tokio::fs::File::create(config_file).await?,
                config_str.as_bytes(),
            )
            .await?;
        } else {
            tokio::fs::write(config_file.clone(), config_str).await?;
        }

        Ok(())
    }
}

impl MQTTConfig {
    pub fn check(&self) -> anyhow::Result<()> {
        if self.ssl.enable && self.ssl.trust_store.is_none() {
            return Err(anyhow::anyhow!(
                "trust_store cannot be empty when opening ssl connection"
            ));
        }

        if self.auth.enable && (self.auth.username.is_none() || self.auth.password.is_none()) {
            return Err(anyhow::anyhow!(
                "When the identity authentication is Yes, the username and password cannot be empty."
            ));
        }
        Ok(())
    }
}

impl Default for TymeConfig {
    fn default() -> Self {
        Self {
            mqtt_config: Default::default(),
            web_console_config: Default::default(),
            first_start: true,
            config_file: Default::default(),
            database: Default::default(),
        }
    }
}

impl Default for WebConsoleConfig {
    fn default() -> Self {
        Self {
            username: String::from("root"),
            password: nanoid::nanoid!(8),
            port: 12566,
            api_token: Default::default(),
        }
    }
}

impl<'a> IntoLua<'a> for TymeConfig {
    fn into_lua(self, lua: &'a mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        table.set("mqtt_config", self.mqtt_config.into_lua(lua)?)?;
        table.set("web_console_config", self.web_console_config.into_lua(lua)?)?;
        table.into_lua(lua)
    }
}

impl<'a> IntoLua<'a> for MQTTConfig {
    fn into_lua(self, lua: &'a mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        table.set("broker", self.broker.into_lua(lua)?)?;
        table.set("port", self.port.into_lua(lua)?)?;
        table.set("client_id", self.client_id.into_lua(lua)?)?;
        table.set(
            "keep_alive_interval",
            self.keep_alive_interval.into_lua(lua)?,
        )?;
        table.set("auth", self.auth.into_lua(lua)?)?;
        table.set("ssl", self.ssl.into_lua(lua)?)?;
        table.into_lua(lua)
    }
}

impl<'a> IntoLua<'a> for WebConsoleConfig {
    fn into_lua(self, lua: &'a mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        table.set("username", self.username.into_lua(lua)?)?;
        table.set("password", self.password.into_lua(lua)?)?;
        table.set("port", self.port.into_lua(lua)?)?;
        table.set("api_token", self.api_token.into_lua(lua)?)?;
        table.into_lua(lua)
    }
}

impl<'a> IntoLua<'a> for Auth {
    fn into_lua(self, lua: &'a mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        table.set("enable", self.enable.into_lua(lua)?)?;
        table.set("username", self.username.into_lua(lua)?)?;
        table.set("password", self.password.into_lua(lua)?)?;
        table.into_lua(lua)
    }
}

impl<'a> IntoLua<'a> for Ssl {
    fn into_lua(self, lua: &'a mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        table.set("enable", self.enable.into_lua(lua)?)?;
        table.set(
            "trust_store",
            self.trust_store
                .as_ref()
                .map(|p| p.as_str().into_lua(lua))
                .unwrap_or_else(|| Ok(mlua::Value::Nil))?,
        )?;
        table.set(
            "key_store",
            self.key_store
                .as_ref()
                .map(|p| p.as_os_str().to_str().into_lua(lua))
                .unwrap_or_else(|| Ok(mlua::Value::Nil))?,
        )?;
        table.set(
            "private_key",
            self.private_key
                .as_ref()
                .map(|p| p.as_os_str().to_str().into_lua(lua))
                .unwrap_or_else(|| Ok(mlua::Value::Nil))?,
        )?;
        table.set(
            "private_key_password",
            self.private_key_password.into_lua(lua)?,
        )?;
        table.set(
            "ca_path",
            self.ca_path
                .as_ref()
                .map(|p| p.as_os_str().to_str().into_lua(lua))
                .unwrap_or_else(|| Ok(mlua::Value::Nil))?,
        )?;
        table.set("protos", self.protos.into_lua(lua)?)?;
        table.into_lua(lua)
    }
}

impl<'a> IntoLua<'a> for Header {
    fn into_lua(self, lua: &'a mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        table.set("topic", self.topic.into_lua(lua)?)?;
        table.set("qos", self.qos.into_lua(lua)?)?;
        table.into_lua(lua)
    }
}
