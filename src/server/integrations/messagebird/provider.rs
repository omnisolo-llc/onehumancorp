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
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::provider::MessageBirdProvider;

    #[test]
    fn test_messagebird_provider_metadata() {
        let provider = MessageBirdProvider::new("test_token".to_string());
        assert_eq!(
            provider.to_integration_provider().metadata.id,
            "messagebird"
        );
        assert_eq!(provider.to_integration_provider().metadata.category, "sms");
    }
}
