//! The app config module contains the application configuration struct

use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

/// The struct of application configuration
///
/// # Example
/// Here is an example of how to use it.
///
/// ```rust
/// use libs::config::app::AppConfig;
///
/// let main_config = AppConfig::new("/path/to/config.yaml");
/// let server_config = main_config.server;
/// ```
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerConfig {
    /// The host of the server
    pub host: Ipv4Addr,

    /// The port of the server
    pub port: u16,
}
