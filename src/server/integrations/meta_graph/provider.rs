use super::client::{MetaGraphClientWrapper, RealMetaGraphClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct MetaGraphProvider {
    client: Arc<dyn MetaGraphClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl MetaGraphProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealMetaGraphClient::new(access_token);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "meta_graph".to_string(),
                name: "Meta Graph API".to_string(),
                category: "social_media".to_string(),
                base_url: "https://graph.facebook.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn MetaGraphClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "meta_graph".to_string(),
                name: "Meta Graph API".to_string(),
                category: "social_media".to_string(),
                base_url: "https://graph.facebook.com".to_string(),
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

    pub async fn send_message(&self, recipient_id: &str, message: &str) -> Result<(), String> {
        self.client.send_message(recipient_id, message).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockMetaGraphClient {
        sent_messages: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl MetaGraphClientWrapper for MockMetaGraphClient {
        async fn send_message(&self, _recipient_id: &str, _message: &str) -> Result<(), String> {
            self.sent_messages.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_meta_graph_provider_integration() {
        let sent = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockMetaGraphClient { sent_messages: sent.clone() });
        let provider = MetaGraphProvider::with_client(mock);

        provider.send_message("123", "Test message").await.unwrap();
        assert_eq!(sent.load(Ordering::SeqCst), 1);
    }
}
