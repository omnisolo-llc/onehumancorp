use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct CalProvider {
    metadata: ProviderMetadata,
}

impl CalProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "cal".to_string(),
                name: "Cal.com Scheduling".to_string(),
                category: "scheduling".to_string(),
                base_url: "https://api.cal.com".to_string(),
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
