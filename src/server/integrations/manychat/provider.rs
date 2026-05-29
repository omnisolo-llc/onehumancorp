use super::client::{ManychatClient, ManychatConversation};
use super::commerce::CommerceConversationHandoff;
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
            },
        }
    }

    pub async fn fetch_conversations(&self) -> Result<Vec<ManychatConversation>, String> {
        self._client.fetch_conversations().await
    }

    pub async fn fetch_commerce_handoffs(
        &self,
        tenant_id: &str,
    ) -> Result<Vec<CommerceConversationHandoff>, String> {
        let conversations = self.fetch_conversations().await?;
        Ok(conversations
            .iter()
            .map(|conversation| {
                CommerceConversationHandoff::from_manychat(tenant_id.to_string(), conversation)
            })
            .collect())
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

    #[tokio::test]
    async fn test_manychat_provider_fetches_typed_conversation_context() {
        let provider = ManychatProvider::new("test_token".to_string());
        let conversations = provider.fetch_conversations().await.unwrap();
        assert_eq!(conversations[0].channel, "instagram");
        assert_eq!(conversations[0].messages[0].direction, "inbound");
    }

    #[tokio::test]
    async fn test_manychat_provider_builds_commerce_handoff() {
        let provider = ManychatProvider::new("test_token".to_string());
        let handoffs = provider
            .fetch_commerce_handoffs("tenant_123")
            .await
            .unwrap();

        assert_eq!(handoffs[0].tenant_id, "tenant_123");
        assert_eq!(handoffs[0].source_channel, "instagram");
        assert!(handoffs[0].checkout_seed.quote_required);
        assert_eq!(handoffs[0].checkout_seed.payment_provider, "stripe");
    }
}
