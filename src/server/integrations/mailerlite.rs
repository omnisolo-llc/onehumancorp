use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct MailerLiteProvider {
    pub metadata: ProviderMetadata,
}

impl MailerLiteProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "mailerlite".to_string(),
                name: "MailerLite".to_string(),
                category: "email_marketing".to_string(),
                base_url: "https://connect.mailerlite.com/api".to_string(),
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
