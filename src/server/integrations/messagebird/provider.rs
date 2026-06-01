use super::client::MessageBirdClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct MessageBirdProvider {
    _client: Arc<MessageBirdClient>,
    metadata: ProviderMetadata,
}

impl MessageBirdProvider {
    pub fn new(api_key: String) -> Self {
        let client = MessageBirdClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "messagebird".to_string(),
                name: "MessageBird".to_string(),
                category: "sms".to_string(),
                base_url: "https://rest.messagebird.com".to_string(),
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
