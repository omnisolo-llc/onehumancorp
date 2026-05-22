use super::client::ManychatClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ManychatProvider {
    _client: Arc<ManychatClient>,
    metadata: ProviderMetadata,
}

impl ManychatProvider {
    pub fn new(access_token: String) -> Self {
        let client = ManychatClient::new(access_token);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "manychat".to_string(),
                name: "Manychat".to_string(),
                category: "social_media".to_string(),
                base_url: "https://api.manychat.com/fb".to_string(),
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

    pub async fn send_message(&self, platform: &str, to: &str, body: &str) -> Result<(), String> {
        self._client.send_message(platform, to, body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manychat_provider_new() {
        let provider = ManychatProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "manychat");
        assert_eq!(provider.metadata.category, "social_media");
    }

    #[test]
    fn test_manychat_provider_into() {
        let provider = ManychatProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "manychat");
    }
}
