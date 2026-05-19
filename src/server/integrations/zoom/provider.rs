use super::client::{ZoomClientWrapper, RealZoomClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ZoomProvider {
    client: Arc<dyn ZoomClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl ZoomProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealZoomClient::new(access_token);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "zoom".to_string(),
                name: "Zoom".to_string(),
                category: "video_conferencing".to_string(),
                base_url: "https://api.zoom.us/v2".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn ZoomClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "zoom".to_string(),
                name: "Zoom".to_string(),
                category: "video_conferencing".to_string(),
                base_url: "https://api.zoom.us/v2".to_string(),
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

    pub async fn create_meeting(&self, topic: &str, start_time: &str) -> Result<String, String> {
        self.client.create_meeting(topic, start_time).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockZoomClient {
        created_meetings: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl ZoomClientWrapper for MockZoomClient {
        async fn create_meeting(&self, _topic: &str, _start_time: &str) -> Result<String, String> {
            self.created_meetings.fetch_add(1, Ordering::SeqCst);
            Ok("mock_url".to_string())
        }
    }

    #[tokio::test]
    async fn test_zoom_provider_integration() {
        let created = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockZoomClient { created_meetings: created.clone() });
        let provider = ZoomProvider::with_client(mock);

        provider.create_meeting("Topic", "time").await.unwrap();
        assert_eq!(created.load(Ordering::SeqCst), 1);
    }
}
