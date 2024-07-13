use std::{fs, sync::OnceLock};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bind: String,
    pub database_url: String,
    pub client_public_key: String,
    pub server_secret_key: String,
    pub announcements: Vec<AnnouncementItem>,
}

#[derive(Debug, Deserialize)]
pub struct AnnouncementItem {
    pub index: u32,
    pub order: u32,
    pub title: String,
    pub published_at: u64,
    pub body: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get() -> &'static Config {
    CONFIG.get_or_init(
        || toml::from_str(
            &fs::read_to_string("./config.toml")
                .expect("Could not read config.toml")
            ).expect("Could not parse config.toml")
    )
}
