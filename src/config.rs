use anyhow::Result;
use std::{fmt, collections::HashMap};
use crate::domain::user::User;

#[derive(Clone, serde::Deserialize)]
pub struct Config {
    pub application: Application,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Application {
    pub listen_address: String,
    pub dev_users: Option<HashMap<String, User>>,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .set_default("application.listen_address", "127.0.0.1:8080")?
            .add_source(config::Environment::with_prefix("BACKEND_VAR").separator("__"))
            .build()?
            .try_deserialize()
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
Configuration
Bind address:                      {bind_address}
Dev users:  	                   {users}
",
            bind_address = self.application.listen_address,
            users = serde_json::to_string(&self.application.dev_users).unwrap()
        )
    }
}
