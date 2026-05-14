use super::client::{ZoomClientWrapper, RealZoomClient, ZoomMeeting};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ZoomProvider {
    client: Arc<dyn ZoomClientWrapper>,
    metadata: ProviderMetadata,
}

impl ZoomProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealZoomClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "zoom".to_string(),
                name: "Zoom Conferencing".to_string(),
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
                name: "Zoom Conferencing".to_string(),
                category: "video".to_string(),
                base_url: "https://api.zoom.us".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn create_meeting(&self, topic: &str) -> Result<ZoomMeeting, String> {
        self.client.create_meeting(topic).await
    }
}
