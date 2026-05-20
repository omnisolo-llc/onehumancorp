use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct MessageBirdProvider {
    pub metadata: ProviderMetadata,
}

impl MessageBirdProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "messagebird".to_string(),
                name: "MessageBird".to_string(),
                category: "sms".to_string(),
                base_url: "https://rest.messagebird.com".to_string(),
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
