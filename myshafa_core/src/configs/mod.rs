//! Configurations module

use crate::errors::config_error::ConfigError;

use self::{server::ServerConfig, db::DbConfig};
use serde::{Deserialize, Serialize};

pub mod server;
pub mod db;

/// The struct of main configuration
///
/// # Example
/// Here is an example of how to use it.
///
/// ```rust
/// use libs::config::app::AppConfig;
/// AppConfig::new("/path/to/config.yaml");
/// ```
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub db: DbConfig,
}

impl AppConfig {
    /// The method will returns a configuration of the application.
    pub fn new(path: &str) -> Result<AppConfig, ConfigError> {
        let path = path.to_string();

        let file = std::fs::read_to_string(path.clone());
        if file.is_err() {
            return Err(ConfigError::NotFound);
        }

        let file = file.unwrap();

        let cfg: Result<AppConfig, serde_yaml::Error> = serde_yaml::from_str(&file);
        if cfg.is_err() {
            return Err(ConfigError::Invalid);
        }

        Ok(cfg.unwrap())
    }
}
