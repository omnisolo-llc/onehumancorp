use super::client::KarrioClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct KarrioProvider {
    _client: Arc<KarrioClient>,
    metadata: ProviderMetadata,
}

impl KarrioProvider {
    pub fn new(api_key: String) -> Self {
        let client = KarrioClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "karrio".to_string(),
                name: "Karrio Shipping".to_string(),
                category: "shipping".to_string(),
                base_url: "https://api.karrio.io/v1".to_string(),
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
