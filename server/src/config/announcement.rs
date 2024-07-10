use std::{error::Error, fs, sync::OnceLock};

use serde::Deserialize;

use super::ConfigError;

#[derive(Debug, Deserialize)]
pub struct AnnouncementConfig {
    pub announcements: Vec<AnnouncementConfigItem>,
}

#[derive(Debug, Deserialize)]
pub struct AnnouncementConfigItem {
    pub index: u32,
    pub order: u32,
    pub title: String,
    pub published_at: u64,
    pub body: String,
}

static CONFIG: OnceLock<AnnouncementConfig> = OnceLock::new();

pub fn get_config() -> &'static AnnouncementConfig {
    CONFIG.get_or_init(|| toml::from_str(&fs::read_to_string("./config/announcement.toml").unwrap()).unwrap())
}
