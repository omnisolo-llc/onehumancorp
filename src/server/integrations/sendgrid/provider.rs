use super::client::SendGridClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct SendGridProvider {
    _client: Arc<SendGridClient>,
    metadata: ProviderMetadata,
}

impl SendGridProvider {
    pub fn new(api_key: String) -> Self {
        let client = SendGridClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "sendgrid".to_string(),
                name: "SendGrid Email".to_string(),
                category: "email".to_string(),
                base_url: "https://api.sendgrid.com/v3".to_string(),
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
        self._client.send_email(to, subject, body).await
    }
}
