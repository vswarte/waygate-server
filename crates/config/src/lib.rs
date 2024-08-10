use std::error::Error;
use std::sync::OnceLock;
use std::fs::read_to_string;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub bind: String,
    pub database_url: String,
    pub client_public_key: String,
    pub server_secret_key: String,
    pub api_bind: String,
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct AnnouncementConfig {
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

pub static GENERAL: OnceLock<GeneralConfig> = OnceLock::new();
pub static ANNOUNCEMENTS: OnceLock<AnnouncementConfig> = OnceLock::new();

pub fn init_config() -> Result<(), Box<dyn Error>>{
    let general = toml::from_str(
        &read_to_string("./config.toml")?
    )?;
    let _ = GENERAL.set(general);


    let announcements = toml::from_str(
        &read_to_string("./announcements.toml")?
    )?;
    let _ = ANNOUNCEMENTS.set(announcements);

    Ok(())
}
