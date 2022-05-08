use serde_derive::Deserialize;
use std::fmt;
use std::fmt::Debug;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub port: u16,
    pub host: String,
}

type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug, Clone)]
pub struct ConfigError {
    pub err: String,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl std::error::Error for ConfigError {}

pub fn get_config() -> Result<Config> {
    let def_config = Config {
        host: "127.0.0.1".to_owned(),
        port: 2022,
    };

    let file_content = match fs::read_to_string("config.toml") {
        Ok(ret) => ret,
        Err(_) => return Ok(def_config),
    };

    let config: Config = match toml::from_str(&file_content) {
        Ok(ret) => ret,
        Err(_) => return Ok(def_config),
    };
    Ok(config)
}

#[cfg(test)]
mod tests;
