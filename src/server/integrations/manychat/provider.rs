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
                base_url: "https://placeholder.url".to_string(),
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
