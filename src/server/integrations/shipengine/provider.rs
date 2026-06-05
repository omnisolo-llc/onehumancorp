use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipEngineConfig {
    pub api_key: String,
}

pub struct ShipEngineProvider {
    config: ShipEngineConfig,
}

impl ShipEngineProvider {
    pub fn new(config: ShipEngineConfig) -> Self {
        Self { config }
    }
}
