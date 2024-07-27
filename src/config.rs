use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub discord: Discord,
}

#[derive(Deserialize)]
pub struct Discord {
    pub auth_token: String,
    pub channels: Vec<String>,
}

impl Config {
    pub fn load() -> Self
    {
        let config = fs::read_to_string(".discogram/config.toml").expect("Failed to read config.toml");
        return toml::from_str(&config).expect("Failed to parse config.toml");
    }
}