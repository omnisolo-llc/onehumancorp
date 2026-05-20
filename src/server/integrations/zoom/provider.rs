use super::client::{ZoomClientWrapper, RealZoomClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ZoomProvider {
    client: Arc<dyn ZoomClientWrapper>,
    metadata: ProviderMetadata,
}

impl ZoomProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealZoomClient::new(api_key);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "zoom".to_string(),
                name: "Zoom".to_string(),
                category: "video".to_string(),
                base_url: "https://api.zoom.us".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn ZoomClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "zoom".to_string(),
                name: "Zoom".to_string(),
                category: "video".to_string(),
                base_url: "https://api.zoom.us".to_string(),
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

    pub async fn create_meeting(&self, topic: &str) -> Result<String, String> {
        self.client.create_meeting(topic).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockZoomClient {
        called: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl ZoomClientWrapper for MockZoomClient {
        async fn create_meeting(&self, _topic: &str) -> Result<String, String> {
            self.called.fetch_add(1, Ordering::SeqCst);
            Ok("link".to_string())
        }
    }

    #[tokio::test]
    async fn test_zoom_provider_integration() {
        let called = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockZoomClient { called: called.clone() });
        let provider = ZoomProvider::with_client(mock);

        provider.create_meeting("topic").await.unwrap();
        assert_eq!(called.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_zoom_provider_new() {
        let provider = ZoomProvider::new("key".to_string());
        assert_eq!(provider.metadata.id, "zoom");
    }

    #[test]
    fn test_zoom_provider_to_integration_provider() {
        let provider = ZoomProvider::new("key".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "zoom");
    }
}
