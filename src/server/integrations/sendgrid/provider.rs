use super::client::{SendGridClientWrapper, RealSendGridClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct SendGridProvider {
    client: Arc<dyn SendGridClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl SendGridProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealSendGridClient::new(api_key);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "sendgrid".to_string(),
                name: "SendGrid".to_string(),
                category: "email_marketing".to_string(),
                base_url: "https://api.sendgrid.com/v3".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn SendGridClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "sendgrid".to_string(),
                name: "SendGrid".to_string(),
                category: "email_marketing".to_string(),
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
            },
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
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockSendGridClient {
        sent_emails: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl SendGridClientWrapper for MockSendGridClient {
        async fn send_email(&self, _to: &str, _subject: &str, _body: &str) -> Result<(), String> {
            self.sent_emails.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_sendgrid_provider_integration() {
        let sent = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockSendGridClient { sent_emails: sent.clone() });
        let provider = SendGridProvider::with_client(mock);

        provider.send_email("test@example.com", "Subject", "Body").await.unwrap();
        assert_eq!(sent.load(Ordering::SeqCst), 1);
    }
}
