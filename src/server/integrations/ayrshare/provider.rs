use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::AyrshareClient;
use std::sync::Arc;

pub struct AyrshareProvider {
    metadata: ProviderMetadata,
    client: Option<Arc<AyrshareClient>>,
}

impl AyrshareProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "ayrshare".to_string(),
                name: "Ayrshare Social Media".to_string(),
                category: "social_media".to_string(),
                base_url: "https://app.ayrshare.com/api".to_string(),
            },
            client: None,
        }
    }

    pub fn with_api_key(api_key: String) -> Self {
        let mut provider = Self::new();
        provider.client = Some(Arc::new(AyrshareClient::new(api_key)));
        provider
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn fetch_messages(&self) -> Result<Vec<String>, String> {
        if let Some(client) = &self.client {
            client.fetch_messages().await
        } else {
            Err("Ayrshare client not initialized".to_string())
        }
    }

    pub async fn send_reply(&self, platform: &str, user_id: &str, message: &str) -> Result<(), String> {
        if let Some(client) = &self.client {
            client.send_reply(platform, user_id, message).await
        } else {
            Err("Ayrshare client not initialized".to_string())
        }
    }

    pub async fn schedule_post(&self, content: &str, platforms: Vec<&str>) -> Result<String, String> {
        if let Some(client) = &self.client {
            client.schedule_post(content, platforms).await
        } else {
            Err("Ayrshare client not initialized".to_string())
        }
    }
}
