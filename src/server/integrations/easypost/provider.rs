use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct EasyPostProvider {
    pub metadata: IntegrationProvider,
}

impl EasyPostProvider {
    pub fn new() -> Self {
        Self {
            metadata: IntegrationProvider {
                metadata: ProviderMetadata {
                    id: "easypost".to_string(),
                    name: "EasyPost".to_string(),
                    category: "Shipping & Logistics".to_string(),
                    base_url: "https://api.easypost.com/v2".to_string(),
                }
            },
        }
    }
}
