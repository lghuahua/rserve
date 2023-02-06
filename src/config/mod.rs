use crate::cli::Cli;
use confy::ConfyError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::net::IpAddr;
use std::{
    env,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub public: PathBuf,
    pub port: u16,
    pub host: IpAddr,
    pub headers: Vec<Header>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            public: PathBuf::from_str("./").unwrap(),
            port: 8008,
            host: "127.0.0.1".parse().unwrap(),
            headers: vec![],
        }
    }
}

pub fn load_config(path: &Path) -> Config {
    let cfg: Result<Config, ConfyError> = confy::load_path(path);
    match cfg {
        Ok(cfg) => cfg,
        Err(_e) => Config::default(),
    }
}

impl TryFrom<Cli> for Config {
    type Error = ConfyError;
    fn try_from(cli: Cli) -> Result<Self, Self::Error> {
        let root_path = if cli.path.is_some() {
            cli.path.unwrap()
        } else {
            env::current_dir().ok().unwrap()
        };

        Ok(Config {
            public: root_path,
            port: cli.port,
            host: "127.0.0.1".parse().unwrap(),
            headers: vec![],
        })
    }
}
