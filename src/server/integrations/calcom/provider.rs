use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct CalComProvider {
    pub metadata: IntegrationProvider,
}

impl CalComProvider {
    pub fn new() -> Self {
        Self {
            metadata: IntegrationProvider {
                metadata: ProviderMetadata {
                    id: "calcom".to_string(),
                    name: "Cal.com".to_string(),
                    category: "Calendar & Scheduling".to_string(),
                    base_url: "https://api.cal.com/v1".to_string(),
                }
            },
        }
    }
}
