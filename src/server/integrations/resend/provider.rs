use super::client::{ResendClientWrapper, RealResendClient};
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
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
                name: "Resend Email Marketing".to_string(),
                category: "email_marketing".to_string(),
                base_url: "https://api.resend.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn ResendClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "resend".to_string(),
                name: "Resend Email Marketing".to_string(),
                category: "email_marketing".to_string(),
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

    pub async fn send_email(&self, to: &str, from: &str, subject: &str, html_body: &str) -> Result<(), String> {
        self.client.send_email(to, from, subject, html_body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockResendClient;

    #[async_trait]
    impl ResendClientWrapper for MockResendClient {
        async fn send_email(&self, _to: &str, _from: &str, _subject: &str, _html_body: &str) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_resend_provider_new() {
        let provider = ResendProvider::new("test_key".to_string());
        assert_eq!(provider.metadata.id, "resend");
        assert_eq!(provider.metadata.category, "email_marketing");
    }

    #[test]
    fn test_resend_provider_to_integration_provider() {
        let provider = ResendProvider::new("test_key".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "resend");
    }

    #[tokio::test]
    async fn test_resend_provider_send_email() {
        let mock_client = Arc::new(MockResendClient);
        let provider = ResendProvider::with_client(mock_client);
        let result = provider.send_email("test@example.com", "sender@example.com", "Subject", "<p>Body</p>").await;
        assert!(result.is_ok());
    }
}
