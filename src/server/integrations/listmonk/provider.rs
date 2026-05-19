use super::client::ListmonkClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct ListmonkProvider {
    pub client: ListmonkClient,
    pub metadata: ProviderMetadata,
}

impl ListmonkProvider {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            client: ListmonkClient::new(base_url.clone(), api_key),
            metadata: ProviderMetadata {
                id: "listmonk".to_string(),
                name: "Listmonk Email".to_string(),
                category: "marketing".to_string(),
                base_url,
            },
        }
    }

    pub fn to_integration_provider(&self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata.clone(),
        }
    }
}
