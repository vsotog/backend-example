#![feature(int_roundings)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]

use anyhow::Result;

pub mod api;
pub mod config;
pub mod domain;

#[derive(Clone)]
pub struct Backend {
    config: config::Config,
}

impl Backend {
    pub fn new(config: config::Config) -> Self {
        Self { config: config }
    }
}

/// Initialize application and its dependencies.
pub async fn initialize_app(cfg: config::Config) -> Result<Backend> {
    Ok(Backend::new(cfg))
}
