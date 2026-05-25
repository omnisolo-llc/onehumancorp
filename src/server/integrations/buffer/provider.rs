use super::client::BufferClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct BufferProvider {
    _client: Arc<BufferClient>,
    metadata: ProviderMetadata,
}

impl BufferProvider {
    pub fn new(access_token: String) -> Self {
        let client = BufferClient::new(access_token);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "buffer".to_string(),
                name: "Buffer".to_string(),
                category: "social_media".to_string(),
                base_url: "https://api.bufferapp.com/1".to_string(),
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

impl BufferProvider {
    pub async fn get_messages(&self) -> Result<Vec<String>, String> {
        self._client.get_messages().await
    }

    pub async fn reply_message(&self, message_id: &str, reply: &str) -> Result<(), String> {
        self._client.reply_message(message_id, reply).await
    }
}
