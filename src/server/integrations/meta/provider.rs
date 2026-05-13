use super::client::{MetaClientWrapper, RealMetaClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct MetaProvider {
    client: Arc<dyn MetaClientWrapper>,
    metadata: ProviderMetadata,
}

impl MetaProvider {
    pub fn new(page_access_token: String) -> Self {
        let client = RealMetaClient::new(page_access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "meta".to_string(),
                name: "Meta Inbox".to_string(),
                category: "social".to_string(),
                base_url: "https://graph.facebook.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn MetaClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "meta".to_string(),
                name: "Meta Inbox".to_string(),
                category: "social".to_string(),
                base_url: "https://graph.facebook.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn send_reply(&self, recipient_id: &str, message: &str) -> Result<(), String> {
        self.client.send_reply(recipient_id, message).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockMetaClient {
        replies_sent: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl MetaClientWrapper for MockMetaClient {
        async fn send_reply(&self, _recipient_id: &str, _message: &str) -> Result<(), String> {
            self.replies_sent.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_meta_provider_integration() {
        let sent = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockMetaClient { replies_sent: sent.clone() });
        let provider = MetaProvider::with_client(mock);

        provider.send_reply("user1", "Hi").await.unwrap();
        assert_eq!(sent.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_meta_provider_new() {
        let provider = MetaProvider::new("token".to_string());
        assert_eq!(provider.metadata.id, "meta");
        assert_eq!(provider.metadata.category, "social");
    }

    #[test]
    fn test_meta_provider_into() {
        let provider = MetaProvider::new("token".to_string());
        let integration = provider.into_integration_provider();
        assert_eq!(integration.metadata.id, "meta");
    }
}
