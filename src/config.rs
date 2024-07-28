use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub general: General,
    pub telegram: Telegram,
    pub discord: Discord,
    pub channels: Vec<Channel>,
}

#[derive(Deserialize)]
pub struct General {
    pub trigger: u64
}

#[derive(Deserialize)]
pub struct Telegram {
    pub bot_token: String,
    pub chat_id: String,
}

#[derive(Deserialize)]
pub struct Discord {
    pub auth_token: String,
}

#[derive(Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: String,
}

impl Config {
    pub fn load() -> Self
    {
        let config = fs::read_to_string(".discogram/config.toml").expect("Failed to read config.toml");
        return toml::from_str(&config).expect("Failed to parse config.toml");
    }
}