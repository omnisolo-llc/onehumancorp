use super::client::{ZoomClientWrapper, ZoomClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ZoomProvider {
    _client: Arc<dyn ZoomClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl ZoomProvider {
    pub fn new(api_key: String) -> Self {
        let client = ZoomClient::new(api_key);

        Self {
            _client: Arc::new(client),
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
        self._client.create_meeting(topic).await
    }

    pub async fn get_oauth_url(&self, redirect_uri: &str) -> String {
        self._client.get_oauth_url(redirect_uri).await
    }

    pub async fn exchange_token(&self, code: &str, redirect_uri: &str) -> Result<String, String> {
        self._client.exchange_token(code, redirect_uri).await
    }
}
