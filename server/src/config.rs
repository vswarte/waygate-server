pub mod announcement;

use std::io;

use thiserror::Error;

pub fn init() -> Result<(), ConfigError> {
    // Validate all the config files by parsing them once
    let _ = dbg!(announcement::get_config());

    Ok(())
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Io error {0}")]
    Io(#[from] io::Error),
    #[error("Deserialization failed {0}")]
    Deserialize(#[from] toml::de::Error)
}
