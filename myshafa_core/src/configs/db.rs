//! The database struct configuration

use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

/// The struct of database configuration
///
/// # Example
/// Here is an example of how to use it.
///
/// ```rust
/// use libs::config::app::AppConfig;
///
/// let app_config = AppConfig::new("/path/to/config.yaml");
/// let db_connection_string = app_config.db.get_uri();
/// ```
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DbConfig {
    /// The username of the database
    pub username: String,

    /// The password of the database
    pub password: String,

    /// The host of the database
    pub host: Ipv4Addr,

    /// The port of the database
    pub port: u16,

    // The name of the database
    pub name: String,
}

impl DbConfig {
    /// The method will returns a database connection string.
    pub fn get_uri(&self) -> String {
        let mut username_and_password = String::from("");
        if !self.username.is_empty() && !self.password.is_empty() {
            username_and_password = format!("{}:{}@", self.username, self.password);
        }

        format!(
            "mongodb://{}{}:{}",
            username_and_password, self.host, self.port
        )
    }
}
