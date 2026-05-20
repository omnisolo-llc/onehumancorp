use super::client::{ManychatClientWrapper, RealManychatClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ManychatProvider {
    client: Arc<dyn ManychatClientWrapper>,
    metadata: ProviderMetadata,
}

impl ManychatProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealManychatClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "manychat".to_string(),
                name: "Manychat".to_string(),
                category: "social".to_string(),
                base_url: "https://api.manychat.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn ManychatClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "manychat".to_string(),
                name: "Manychat".to_string(),
                category: "social".to_string(),
                base_url: "https://api.manychat.com".to_string(),
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

    pub async fn send_message(&self, subscriber_id: &str, message: &str) -> Result<(), String> {
        self.client.send_message(subscriber_id, message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockManychatClient {
        sent_messages: Arc<AtomicUsize>,
    }

    impl ManychatClientWrapper for MockManychatClient {
        fn send_message(&self, _subscriber_id: &str, _message: &str) -> Result<(), String> {
            self.sent_messages.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_manychat_provider_integration() {
        let sent = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockManychatClient { sent_messages: sent.clone() });
        let provider = ManychatProvider::with_client(mock);

        provider.send_message("123", "Test message").await.unwrap();
        assert_eq!(sent.load(Ordering::SeqCst), 1);
    }
}
