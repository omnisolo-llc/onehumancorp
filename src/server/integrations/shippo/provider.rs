use super::client::{ShippoClientWrapper, RealShippoClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ShippoProvider {
    client: Arc<dyn ShippoClientWrapper>,
    metadata: ProviderMetadata,
}

impl ShippoProvider {
    pub fn new(api_key: String, base_url: String) -> Self {
        let client = RealShippoClient::new(api_key, base_url);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "shippo".to_string(),
                name: "Shippo Automated Labels".to_string(),
                category: "shipping".to_string(),
                base_url: "https://api.shippo.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn ShippoClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "shippo".to_string(),
                name: "Shippo Automated Labels".to_string(),
                category: "shipping".to_string(),
                base_url: "https://api.shippo.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }
}
