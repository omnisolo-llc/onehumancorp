use super::client::{TikTokClientWrapper, RealTikTokClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct TikTokProvider {
    client: Arc<dyn TikTokClientWrapper>,
    metadata: ProviderMetadata,
}

impl TikTokProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealTikTokClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "tiktok".to_string(),
                name: "TikTok Business API".to_string(),
                category: "social".to_string(),
                base_url: "https://business-api.tiktok.com/open_api".to_string(),
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
