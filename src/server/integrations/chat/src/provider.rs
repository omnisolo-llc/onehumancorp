use super::client::{ChatClientWrapper, RealChatClient};
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ChatProvider {
    client: Arc<dyn ChatClientWrapper>,
    metadata: ProviderMetadata,
}

impl ChatProvider {
    pub fn new(endpoint: String, auth_token: String) -> Self {
        let client = RealChatClient::new(endpoint, auth_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "chat_widget".to_string(),
                name: "Web Chat Widget".to_string(),
                category: "chat".to_string(),
                base_url: "https://example.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn ChatClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "chat_widget".to_string(),
                name: "Web Chat Widget".to_string(),
                category: "chat".to_string(),
                base_url: "https://example.com".to_string(),
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

    struct MockChatClient;

    #[async_trait]
    impl ChatClientWrapper for MockChatClient {
        async fn send_message(&self, _to: &str, _body: &str) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_chat_provider_new() {
        let provider = ChatProvider::new("phone_id".to_string(), "test_token".to_string());
        assert_eq!(provider.metadata.id, "chat_widget");
        assert_eq!(provider.metadata.category, "chat");
    }

    #[test]
    fn test_chat_provider_with_client() {
        let mock_client = Arc::new(MockChatClient);
        let provider = ChatProvider::with_client(mock_client);
        assert_eq!(provider.metadata.id, "chat_widget");
        assert_eq!(provider.metadata.category, "chat");
    }

    #[test]
    fn test_chat_provider_to_integration_provider() {
        let provider = ChatProvider::new("phone_id".to_string(), "test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "chat_widget");
    }

    #[tokio::test]
    async fn test_chat_provider_send_message() {
        let mock_client = Arc::new(MockChatClient);
        let provider = ChatProvider::with_client(mock_client);
        let result = provider.send_message("user", "hello").await;
        assert!(result.is_ok());
    }
}
