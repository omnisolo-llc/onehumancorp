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
                base_url: "https://server.api.mailchimp.com/3.0".to_string(),
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

    pub async fn sync_customer(&self, email: &str, tag: &str) -> Result<(), String> {
        self._client.sync_customer(email, tag).await
    }

    pub async fn send_campaign(&self, audience: &str, body: &str) -> Result<(), String> {
        self._client.send_campaign(audience, body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mailchimp_provider_new() {
        let provider = MailchimpProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "mailchimp");
        assert_eq!(provider.metadata.category, "email_marketing");
    }

    #[test]
    fn test_mailchimp_provider_into() {
        let provider = MailchimpProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "mailchimp");
    }
}
