use super::client::ManychatClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ManychatProvider {
    _client: Arc<ManychatClient>,
    metadata: ProviderMetadata,
}

impl ManychatProvider {
    pub fn new(api_key: String) -> Self {
        let client = ManychatClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "manychat".to_string(),
                name: "Manychat".to_string(),
                category: "operations".to_string(),
                base_url: "https://api.manychat.com".to_string(),
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

    pub async fn fetch_conversations(&self) -> Result<Vec<String>, String> {
        self._client.fetch_conversations().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manychat_provider_new() {
        let provider = ManychatProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "manychat");
        assert_eq!(provider.metadata.category, "operations");
    }

    #[test]
    fn test_manychat_provider_into() {
        let provider = ManychatProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "manychat");
    }
}
