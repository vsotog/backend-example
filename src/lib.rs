#![feature(int_roundings)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]

use anyhow::Result;
use parking_lot::RwLock;
use std::sync::Arc;

pub mod api;
pub mod config;

#[derive(Clone)]
pub struct Backend {
    _config: config::Config,
    healthy: Arc<RwLock<bool>>,
}

impl Backend {
    pub fn new(config: config::Config) -> Self {
        Self {
            _config: config,
            healthy: Arc::new(RwLock::new(true)),
        }
    }

    pub fn is_healthy(&self) -> bool {
        *self.healthy.read()
    }

    fn _healthy(&self) {
        *self.healthy.write() = true;
    }

    fn _sick(&self) {
        *self.healthy.write() = false;
    }
}

/// Initialize application and its dependencies.
pub async fn initialize_app(cfg: config::Config) -> Result<Backend> {
    Ok(Backend::new(cfg))
}
