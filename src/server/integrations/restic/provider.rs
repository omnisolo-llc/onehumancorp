use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::env;

pub struct ResticProvider {
    pub metadata: ProviderMetadata,
    pub is_supported: bool,
}

impl ResticProvider {
    pub fn new() -> Self {
        let mode = env::var("OHC_EXECUTION_MODE").unwrap_or_else(|_| "standalone".to_string());

        let is_supported = mode == "standalone";

        Self {
            metadata: ProviderMetadata {
                id: "restic".to_string(),
                name: "Restic Local Backup MCP".to_string(),
                category: "backup".to_string(),
                base_url: "local://restic".to_string(),
            },
            is_supported,
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
}
