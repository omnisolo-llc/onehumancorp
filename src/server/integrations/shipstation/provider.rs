use super::client::ShipStationClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ShipStationProvider {
    _client: Arc<ShipStationClient>,
    metadata: ProviderMetadata,
}

impl ShipStationProvider {
    pub fn new(api_key: String) -> Self {
        let client = ShipStationClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "shipstation".to_string(),
                name: "ShipStation".to_string(),
                category: "shipping".to_string(),
                base_url: "https://ssapi.shipstation.com".to_string(),
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
