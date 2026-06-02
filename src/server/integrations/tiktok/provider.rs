use super::client::{TiktokClientWrapper, RealTiktokClient};
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct TiktokProvider {
    client: Arc<dyn TiktokClientWrapper>,
    metadata: ProviderMetadata,
}

impl TiktokProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealTiktokClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "tiktok".to_string(),
                name: "TikTok for Business API".to_string(),
                category: "social".to_string(),
                base_url: "https://business-api.tiktok.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn TiktokClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "tiktok".to_string(),
                name: "TikTok for Business API".to_string(),
                category: "social".to_string(),
                base_url: "https://business-api.tiktok.com".to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockTiktokClient;

    #[async_trait]
    impl TiktokClientWrapper for MockTiktokClient {
        async fn send_message(&self, _to: &str, _body: &str) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_tiktok_provider_new() {
        let provider = TiktokProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "tiktok");
        assert_eq!(provider.metadata.category, "social");
    }

    #[test]
    fn test_tiktok_provider_with_client() {
        let mock_client = Arc::new(MockTiktokClient);
        let provider = TiktokProvider::with_client(mock_client);
        assert_eq!(provider.metadata.id, "tiktok");
        assert_eq!(provider.metadata.category, "social");
    }

    #[test]
    fn test_tiktok_provider_to_integration_provider() {
        let provider = TiktokProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "tiktok");
    }

    #[tokio::test]
    async fn test_tiktok_provider_send_message() {
        let mock_client = Arc::new(MockTiktokClient);
        let provider = TiktokProvider::with_client(mock_client);
        let result = provider.send_message("user", "hello").await;
        assert!(result.is_ok());
    }
}
