use super::client::{ChatwootClientWrapper, RealChatwootClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ChatwootClientProvider {
    client: Arc<dyn ChatwootClientWrapper>,
    metadata: ProviderMetadata,
}

impl ChatwootClientProvider {
    pub fn new(api_key: String, base_url: String) -> Self {
        let client = RealChatwootClient::new(api_key, base_url);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "chatwoot".to_string(),
                name: "Chatwoot Unified Inbox".to_string(),
                category: "social_media".to_string(),
                base_url: "https://api.chatwoot.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn ChatwootClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "chatwoot".to_string(),
                name: "Chatwoot Unified Inbox".to_string(),
                category: "social_media".to_string(),
                base_url: "https://api.chatwoot.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }
}
