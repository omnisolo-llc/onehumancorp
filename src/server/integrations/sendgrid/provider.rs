use super::client::{SendGridClientWrapper, RealSendGridClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct SendGridProvider {
    client: Arc<dyn SendGridClientWrapper>,
    metadata: ProviderMetadata,
}

impl SendGridProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealSendGridClient::new(api_key);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "sendgrid".to_string(),
                name: "SendGrid Email".to_string(),
                category: "email".to_string(),
                base_url: "https://api.sendgrid.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn SendGridClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "sendgrid".to_string(),
                name: "SendGrid Email".to_string(),
                category: "email".to_string(),
                base_url: "https://api.sendgrid.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn send_email(&self, to: &str, subject: &str, content: &str) -> Result<(), String> {
        self.client.send_email(to, subject, content).await
    }
}
