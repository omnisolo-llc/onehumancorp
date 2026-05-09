use std::sync::Arc;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::{ListmonkClientWrapper, RealListmonkClient};

pub struct ListmonkProvider {
    pub client: Arc<dyn ListmonkClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl ListmonkProvider {
    pub fn new(base_url: String, api_key: String) -> Self {
        let client = RealListmonkClient::new(base_url.clone(), api_key, None);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "listmonk".to_string(),
                name: "Listmonk Newsletter".to_string(),
                category: "marketing".to_string(),
                base_url,
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }
}
