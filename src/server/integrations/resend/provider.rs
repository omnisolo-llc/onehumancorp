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
                name: "Resend Email API".to_string(),
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
                name: "Resend Email API".to_string(),
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
        self.client.send_email(to, subject, body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockResendClient;

    #[async_trait]
    impl ResendClientWrapper for MockResendClient {
        async fn send_email(&self, _to: &str, _subject: &str, _body: &str) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_resend_provider_new() {
        let provider = ResendProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "resend");
        assert_eq!(provider.metadata.category, "email");
    }

    #[test]
    fn test_resend_provider_with_client() {
        let mock_client = Arc::new(MockResendClient);
        let provider = ResendProvider::with_client(mock_client);
        assert_eq!(provider.metadata.id, "resend");
        assert_eq!(provider.metadata.category, "email");
    }

    #[test]
    fn test_resend_provider_to_integration_provider() {
        let provider = ResendProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "resend");
    }

    #[tokio::test]
    async fn test_resend_provider_send_email() {
        let mock_client = Arc::new(MockResendClient);
        let provider = ResendProvider::with_client(mock_client);
        let result = provider.send_email("user@example.com", "Hello", "World").await;
        assert!(result.is_ok());
    }
}
