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
                category: "email_marketing".to_string(),
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
                category: "email_marketing".to_string(),
                base_url: "https://api.sendgrid.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
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
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockSendGridClient {
        sent_messages: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl SendGridClientWrapper for MockSendGridClient {
        async fn send_email(&self, _to: &str, _from: &str, _subject: &str, _html_body: &str) -> Result<(), String> {
            self.sent_messages.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_sendgrid_provider_integration() {
        let sent = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockSendGridClient { sent_messages: sent.clone() });
        let provider = SendGridProvider::with_client(mock);

        provider.send_email("to@example.com", "from@example.com", "Subject", "<h1>Test</h1>").await.unwrap();
        assert_eq!(sent.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_sendgrid_provider_new() {
        let provider = SendGridProvider::new("api_key".to_string());
        assert_eq!(provider.metadata.id, "sendgrid");
    }
}
