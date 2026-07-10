use super::client::{MetaClientWrapper, RealMetaClient};
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct MetaProvider {
    client: Arc<dyn MetaClientWrapper>,
    metadata: ProviderMetadata,
    phone_number_id: Option<String>,
}

impl MetaProvider {
    pub fn new(access_token: String, phone_number_id: Option<String>) -> Self {
        let client = RealMetaClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "meta".to_string(),
                name: "Meta Graph API (Facebook, Instagram, WhatsApp)".to_string(),
                category: "social".to_string(),
                base_url: "https://graph.facebook.com/v19.0".to_string(),
            },
            phone_number_id,
        }
    }

    pub fn with_client(client: Arc<dyn MetaClientWrapper>, phone_number_id: Option<String>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "meta".to_string(),
                name: "Meta Graph API (Facebook, Instagram, WhatsApp)".to_string(),
                category: "social".to_string(),
                base_url: "https://graph.facebook.com/v19.0".to_string(),
            },
            phone_number_id,
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
        self.client.send_message(platform, self.phone_number_id.as_deref(), to, body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockMetaClient;

    #[async_trait]
    impl MetaClientWrapper for MockMetaClient {
        async fn send_message(&self, _platform: &str, _from: Option<&str>, _to: &str, _body: &str) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_meta_provider_new() {
        let provider = MetaProvider::new("test_token".to_string(), None);
        assert_eq!(provider.metadata.id, "meta");
        assert_eq!(provider.metadata.category, "social");
    }

    #[test]
    fn test_meta_provider_with_client() {
        let mock_client = Arc::new(MockMetaClient);
        let provider = MetaProvider::with_client(mock_client, None);
        assert_eq!(provider.metadata.id, "meta");
        assert_eq!(provider.metadata.category, "social");
    }

    #[test]
    fn test_meta_provider_to_integration_provider() {
        let provider = MetaProvider::new("test_token".to_string(), None);
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "meta");
    }

    #[tokio::test]
    async fn test_meta_provider_send_message() {
        let mock_client = Arc::new(MockMetaClient);
        let provider = MetaProvider::with_client(mock_client, Some("12345".to_string()));
        let result = provider.send_message("whatsapp", "user", "hello").await;
        assert!(result.is_ok());
    }
}
