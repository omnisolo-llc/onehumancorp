use super::client::ResendClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ResendProvider {
    _client: Arc<ResendClient>,
    metadata: ProviderMetadata,
}

impl ResendProvider {
    pub fn new(api_key: String) -> Self {
        let client = ResendClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "resend".to_string(),
                name: "Resend Email".to_string(),
                category: "email".to_string(),
                base_url: "https://api.resend.com".to_string(),
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

    pub async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        self._client.send_email(to, subject, body, None).await
    }

    pub async fn send_email_with_from(&self, to: &str, subject: &str, body: &str, from: &str) -> Result<(), String> {
        self._client.send_email(to, subject, body, Some(from)).await
    }
}
