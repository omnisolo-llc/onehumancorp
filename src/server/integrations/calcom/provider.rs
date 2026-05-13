use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct CalComProvider {
    metadata: ProviderMetadata,
}

impl CalComProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "calcom".to_string(),
                name: "Cal.com Scheduling".to_string(),
                category: "scheduling".to_string(),
                base_url: "https://api.cal.com/v1".to_string(),
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
            },
        }
    }
}
