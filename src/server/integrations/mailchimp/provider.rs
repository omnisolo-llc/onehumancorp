use super::client::MailchimpClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct MailchimpProvider {
    _client: Arc<MailchimpClient>,
    metadata: ProviderMetadata,
}

impl MailchimpProvider {
    pub fn new(api_key: String) -> Self {
        let client = MailchimpClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "mailchimp".to_string(),
                name: "Mailchimp".to_string(),
                category: "email_marketing".to_string(),
                base_url: "https://placeholder.url".to_string(),
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
