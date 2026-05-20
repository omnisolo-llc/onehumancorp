use super::client::{ManychatClientWrapper, RealManychatClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ManychatProvider {
    client: Arc<dyn ManychatClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl ManychatProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealManychatClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "manychat".to_string(),
                name: "Manychat Unified Inbox".to_string(),
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
                name: "Manychat Unified Inbox".to_string(),
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

    pub async fn send_message(&self, to: &str, body: &str) -> Result<(), String> {
        self.client.send_message(to, body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockManychatClient {
        calls: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl ManychatClientWrapper for MockManychatClient {
        async fn send_message(&self, _to: &str, _body: &str) -> Result<(), String> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_manychat_provider_integration() {
        let calls = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockManychatClient { calls: calls.clone() });
        let provider = ManychatProvider::with_client(mock);

        provider.send_message("123", "hello").await.unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_manychat_provider_new() {
        let provider = ManychatProvider::new("token".to_string());
        assert_eq!(provider.metadata.id, "manychat");
        assert_eq!(provider.metadata.category, "social");
    }

    #[test]
    fn test_manychat_provider_to_integration_provider() {
        let provider = ManychatProvider::new("token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "manychat");
    }
}
