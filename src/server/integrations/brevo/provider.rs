use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct BrevoProvider {
    metadata: ProviderMetadata,
}

impl BrevoProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "brevo".to_string(),
                name: "Brevo Conversations".to_string(),
                category: "social_inbox".to_string(),
                base_url: "https://api.brevo.com".to_string(),
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
