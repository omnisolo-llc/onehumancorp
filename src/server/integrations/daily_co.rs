use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct DailyCoProvider {
    pub metadata: ProviderMetadata,
}

impl DailyCoProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "daily".to_string(),
                name: "Daily.co Video".to_string(),
                category: "video".to_string(),
                base_url: "https://api.daily.co/v1".to_string(),
            },
        }
    }

    pub fn to_integration_provider(&self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: ProviderMetadata {
                id: self.metadata.id.clone(),
                name: self.metadata.name.clone(),
                category: self.metadata.category.clone(),
                base_url: self.metadata.base_url.clone(),
            }
        }
    }
}
