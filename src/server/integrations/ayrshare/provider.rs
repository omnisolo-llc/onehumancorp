use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct AyrshareProvider {
    pub metadata: IntegrationProvider,
}

impl AyrshareProvider {
    pub fn new() -> Self {
        Self {
            metadata: IntegrationProvider {
                metadata: ProviderMetadata {
                    id: "ayrshare".to_string(),
                    name: "Ayrshare".to_string(),
                    category: "Social Media".to_string(),
                    base_url: "https://app.ayrshare.com/api".to_string(),
                }
            },
        }
    }
}
