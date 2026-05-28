use super::client::ResendClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ResendProvider {
    _client: Arc<ResendClient>,
    pub metadata: ProviderMetadata,
}

impl ResendProvider {
    pub fn new(api_key: String) -> Self {
        let client = ResendClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "resend".to_string(),
                name: "Resend".to_string(),
                category: "marketing".to_string(),
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

    pub async fn send_email(&self, to: &str, subject: &str, html_body: &str) -> Result<(), String> {
        self._client.send_email(to, subject, html_body).await
    }
}
