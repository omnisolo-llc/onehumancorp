use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct ListmonkProvider {
    pub metadata: IntegrationProvider,
}

impl ListmonkProvider {
    pub fn new() -> Self {
        Self {
            metadata: IntegrationProvider {
                metadata: ProviderMetadata {
                    id: "listmonk".to_string(),
                    name: "Listmonk".to_string(),
                    category: "Email Marketing".to_string(),
                    base_url: "http://localhost:9000/api".to_string(),
                }
            },
        }
    }
}
