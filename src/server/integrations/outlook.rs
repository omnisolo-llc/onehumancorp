use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct OutlookProvider {
    pub metadata: ProviderMetadata,
}

impl OutlookProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "outlook".to_string(),
                name: "Outlook Calendar".to_string(),
                category: "calendar".to_string(),
                base_url: "https://graph.microsoft.com".to_string(),
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
