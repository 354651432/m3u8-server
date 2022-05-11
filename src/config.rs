use serde_derive::Deserialize;
use std::fmt;
use std::fmt::Debug;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub bind: String,
    pub proxy: String,
    pub threads: usize,
    pub user_agent: String,
}

pub fn get_config() -> Config {
    let config = Config {
        bind: "127.0.0.1:2022".to_string(),
        proxy: "127.0.0.1:10808".to_string(),
        threads: 17,
        user_agent: "rust".to_string(),
    };

    let content = match fs::read_to_string("m3u8_downloader.toml") {
        Ok(content) => content,
        Err(err) => match fs::read_to_string("~/m3u8_downloader.toml") {
            Ok(content) => content,
            Err(_) => return config,
        },
    };

    match toml::from_str(&content) {
        Ok(config) => config,
        Err(err) => {
            return config;
        }
    }
}
