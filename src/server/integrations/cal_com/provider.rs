use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct CalComProvider {
    pub metadata: ProviderMetadata,
}

impl CalComProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "cal_com".to_string(),
                name: "Cal.com".to_string(),
                category: "calendar".to_string(),
                base_url: "http://localhost".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }
}
