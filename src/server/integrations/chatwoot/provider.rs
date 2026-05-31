use super::client::ChatwootClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ChatwootProvider {
    _client: Arc<ChatwootClient>,
    metadata: ProviderMetadata,
}

impl ChatwootProvider {
    pub fn new(api_key: String, base_url: String) -> Self {
        let client = ChatwootClient::new(api_key, base_url.clone());

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "chatwoot".to_string(),
                name: "Chatwoot Omnichannel".to_string(),
                category: "chat".to_string(),
                base_url,
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

    pub async fn send_message(&self, account_id: &str, conversation_id: &str, content: &str) -> Result<(), String> {
        self._client.send_message(account_id, conversation_id, content).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chatwoot_provider_new() {
        let provider = ChatwootProvider::new("test_token".to_string(), "https://chatwoot.example.com".to_string());
        assert_eq!(provider.metadata.id, "chatwoot");
        assert_eq!(provider.metadata.category, "chat");
    }

    #[test]
    fn test_chatwoot_provider_into() {
        let provider = ChatwootProvider::new("test_token".to_string(), "https://chatwoot.example.com".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "chatwoot");
    }
}
