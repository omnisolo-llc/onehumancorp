use super::client::TikTokClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct TikTokProvider {
    _client: Arc<TikTokClient>,
    metadata: ProviderMetadata,
}

impl TikTokProvider {
    pub fn new(access_token: String) -> Self {
        let client = TikTokClient::new(access_token);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "tiktok".to_string(),
                name: "TikTok for Business".to_string(),
                category: "social".to_string(),
                base_url: "https://business-api.tiktok.com/open_api/v1.3".to_string(),
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
}
