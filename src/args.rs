use std::path::PathBuf;

use structopt::StructOpt;

lazy_static! {
    pub static ref START_PARAM: StartParam = StartParam::from_args();
}

#[derive(Debug, StructOpt)]
pub struct StartParam {
    #[structopt(
        short = "w",
        long = "workdir",
        parse(from_os_str),
        default_value = "./"
    )]
    pub word_dir: PathBuf,

    #[structopt(short = "c", long = "conifg", parse(from_os_str))]
    pub conf_file: Option<PathBuf>,

    #[structopt(short = "i", long = "init")]
    pub init: bool,
}

impl StartParam {
    pub fn get_config_file(&self) -> PathBuf {
        if let Some(conf_file) = &self.conf_file {
            return conf_file.clone();
        }

        #[cfg(target_os = "linux")]
        use std::path::Path;
        #[cfg(target_os = "linux")]
        let config_path = Path::new("/etc/tyme_conf");
        #[cfg(target_os = "linux")]
        if config_path.exists() {
            return config_path.to_path_buf();
        }

        let config_path = crate::start_param.word_dir.join("config.toml");
        if config_path.exists() {
            return config_path;
        }

        let home_dir = dirs::home_dir();
        if let Some(mut home_path) = home_dir {
            home_path.push(".tyme_conf");
            if home_path.exists() {
                return home_path;
            }
        }
        self.word_dir.clone().join("config.toml")
    }
}
