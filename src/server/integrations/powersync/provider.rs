use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::env;

pub struct PowerSyncProvider {
    pub metadata: ProviderMetadata,
    pub is_mock: bool,
    pub base_url: String,
}

impl PowerSyncProvider {
    pub fn new() -> Self {
        let mode = env::var("OHC_EXECUTION_MODE").unwrap_or_else(|_| "standalone".to_string());
        let headless = env::var("OHC_HEADLESS").unwrap_or_else(|_| "false".to_string());

        let is_mock = mode == "cloud" && headless != "true";

        let host = env::var("POWERSYNC_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("POWERSYNC_PORT").unwrap_or_else(|_| "8080".to_string());

        let base_url = if is_mock {
            "mock://powersync".to_string()
        } else {
            format!("http://{}:{}", host, port)
        };

        Self {
            metadata: ProviderMetadata {
                id: "powersync".to_string(),
                name: "PowerSync Hybrid Data Synchronization".to_string(),
                category: "database_sync".to_string(),
                base_url: base_url.clone(),
            },
            is_mock,
            base_url,
        }
    }

    pub fn to_integration_provider(&self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: ProviderMetadata {
                id: self.metadata.id.clone(),
                name: self.metadata.name.clone(),
                category: self.metadata.category.clone(),
                base_url: self.metadata.base_url.clone(),
            },
        }
    }

    pub async fn get_sync_lag(&self) -> Result<u64, String> {
        if self.is_mock {
            return Ok(0);
        }
        Ok(42) // Example logic
    }

    pub async fn get_queue_size(&self) -> Result<u64, String> {
        if self.is_mock {
            return Ok(0);
        }
        Ok(100) // Example logic
    }
}
