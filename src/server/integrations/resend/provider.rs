use super::client::{ResendClientWrapper, RealResendClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ResendProvider {
    client: Arc<dyn ResendClientWrapper>,
    metadata: ProviderMetadata,
}

impl ResendProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealResendClient::new(api_key);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "resend".to_string(),
                name: "Resend Email".to_string(),
                category: "email".to_string(),
                base_url: "https://api.resend.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn ResendClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "resend".to_string(),
                name: "Resend Email".to_string(),
                category: "email".to_string(),
                base_url: "https://api.resend.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn send_email(&self, to: &str, subject: &str, html: &str) -> Result<(), String> {
        self.client.send_email(to, subject, html).await
    }
}
