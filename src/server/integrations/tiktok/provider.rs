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
                name: "TikTok App for Business API".to_string(),
                category: "social".to_string(),
                base_url: "https://business-api.tiktok.com/open_api/v1.3".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn TikTokClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "tiktok".to_string(),
                name: "TikTok App for Business API".to_string(),
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

    pub async fn send_message(&self, platform: &str, to: &str, body: &str) -> Result<(), String> {
        self.client.send_message(platform, to, body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockTikTokClient;

    #[async_trait]
    impl TikTokClientWrapper for MockTikTokClient {
        async fn send_message(&self, _platform: &str, _to: &str, _body: &str) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_tiktok_provider_new() {
        let provider = TikTokProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "tiktok");
        assert_eq!(provider.metadata.category, "social");
    }

    #[test]
    fn test_tiktok_provider_with_client() {
        let mock_client = Arc::new(MockTikTokClient);
        let provider = TikTokProvider::with_client(mock_client);
        assert_eq!(provider.metadata.id, "tiktok");
        assert_eq!(provider.metadata.category, "social");
    }

    #[test]
    fn test_tiktok_provider_to_integration_provider() {
        let provider = TikTokProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "tiktok");
    }

    #[tokio::test]
    async fn test_tiktok_provider_send_message() {
        let mock_client = Arc::new(MockTikTokClient);
        let provider = TikTokProvider::with_client(mock_client);
        let result = provider.send_message("tiktok", "user", "hello").await;
        assert!(result.is_ok());
    }
}
