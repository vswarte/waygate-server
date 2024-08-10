use std::error::Error;
use std::sync::OnceLock;
use std::fs::read_to_string;

use serde::Deserialize;

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

pub static ANNOUNCEMENTS: OnceLock<AnnouncementConfig> = OnceLock::new();

pub fn init_config() -> Result<(), Box<dyn Error>>{
    let announcements = toml::from_str(
        &read_to_string("./announcements.toml")?
    )?;
    let _ = ANNOUNCEMENTS.set(announcements);

    Ok(())
}
