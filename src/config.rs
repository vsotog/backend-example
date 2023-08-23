use anyhow::Result;
use std::fmt;

#[derive(Clone, serde::Deserialize)]
pub struct Config {
    pub application: Application,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Application {
    pub listen_address: String,
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
",
            bind_address = self.application.listen_address,
        )
    }
}
