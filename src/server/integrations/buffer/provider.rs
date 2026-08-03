use super::client::BufferClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
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
                category: "social".to_string(),
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

#[cfg(test)]
mod tests {
    use crate::provider::BufferProvider;

    #[test]
    fn test_buffer_provider_metadata() {
        let provider = BufferProvider::new("test_token".to_string());
        assert_eq!(provider.to_integration_provider().metadata.id, "buffer");
        assert_eq!(provider.to_integration_provider().metadata.category, "social");
    }
}
