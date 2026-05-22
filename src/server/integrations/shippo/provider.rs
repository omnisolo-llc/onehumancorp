use super::client::ShippoClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ShippoProvider {
    _client: Arc<ShippoClient>,
    metadata: ProviderMetadata,
}

impl ShippoProvider {
    pub fn new(api_key: String) -> Self {
        let client = ShippoClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "shippo".to_string(),
                name: "Shippo Logistics".to_string(),
                category: "shipping".to_string(),
                base_url: "https://api.goshippo.com".to_string(),
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
