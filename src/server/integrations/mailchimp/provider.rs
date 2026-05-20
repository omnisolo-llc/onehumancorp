use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct MailchimpProvider {
    metadata: ProviderMetadata,
}

impl MailchimpProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "mailchimp".to_string(),
                name: "Mailchimp Marketing".to_string(),
                category: "email_marketing".to_string(),
                base_url: "https://api.mailchimp.com".to_string(),
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
