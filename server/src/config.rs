pub mod announcement;

use std::io;

use thiserror::Error;

pub fn init() -> Result<(), ConfigError> {
    validate_config();

    Ok(())
}

fn validate_config() {
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Io error {0}")]
    Io(#[from] io::Error),
    #[error("Deserialization failed {0}")]
    Deserialize(#[from] toml::de::Error)
}
