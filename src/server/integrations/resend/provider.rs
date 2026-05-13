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
                name: "Resend".to_string(),
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
                name: "Resend".to_string(),
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

    pub async fn send_email(&self, to: &str, subject: &str, html: &str) -> Result<String, String> {
        self.client.send_email(to, subject, html).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockResendClient {
        emails_sent: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl ResendClientWrapper for MockResendClient {
        async fn send_email(&self, to: &str, _subject: &str, _html: &str) -> Result<String, String> {
            self.emails_sent.fetch_add(1, Ordering::SeqCst);
            Ok(format!("mock_email_{}", to))
        }
    }

    #[tokio::test]
    async fn test_resend_provider_integration() {
        let sent = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockResendClient { emails_sent: sent.clone() });
        let provider = ResendProvider::with_client(mock);

        let res = provider.send_email("user@example.com", "Hi", "<p>hi</p>").await.unwrap();
        assert_eq!(res, "mock_email_user@example.com");
        assert_eq!(sent.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_resend_provider_new() {
        let provider = ResendProvider::new("token".to_string());
        assert_eq!(provider.metadata.id, "resend");
        assert_eq!(provider.metadata.category, "email");
    }

    #[test]
    fn test_resend_provider_into() {
        let provider = ResendProvider::new("token".to_string());
        let integration = provider.into_integration_provider();
        assert_eq!(integration.metadata.id, "resend");
    }
}
