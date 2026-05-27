use super::client::MailerliteClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct MailerliteProvider {
    _client: Arc<MailerliteClient>,
    metadata: ProviderMetadata,
}

impl MailerliteProvider {
    pub fn new(api_key: String) -> Self {
        let client = MailerliteClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "mailerlite".to_string(),
                name: "MailerLite".to_string(),
                category: "email_marketing".to_string(),
                base_url: "https://connect.mailerlite.com/api/v2".to_string(),
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
    fn test_mailerlite_provider_new() {
        let provider = MailerliteProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "mailerlite");
        assert_eq!(provider.metadata.category, "email_marketing");
    }

    #[test]
    fn test_mailerlite_provider_into() {
        let provider = MailerliteProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "mailerlite");
    }
}
