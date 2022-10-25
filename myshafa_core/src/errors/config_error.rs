//! A config errors
use thiserror::Error;

/// Config errors
#[derive(Error, Debug)]
pub enum ConfigError {
    /// A config file not found
    #[error("a config file not found")]
    NotFound,
    /// Invalid config
    #[error("invalid config")]
    Invalid,
}
