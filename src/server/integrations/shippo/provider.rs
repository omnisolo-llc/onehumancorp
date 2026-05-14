use super::client::{ShippoClientWrapper, RealShippoClient, ShippingRate};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ShippoProvider {
    client: Arc<dyn ShippoClientWrapper>,
    metadata: ProviderMetadata,
}

impl ShippoProvider {
    pub fn new(api_token: String) -> Self {
        let client = RealShippoClient::new(api_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "shippo".to_string(),
                name: "Shippo Logistics".to_string(),
                category: "shipping".to_string(),
                base_url: "https://api.goshippo.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn ShippoClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "shippo".to_string(),
                name: "Shippo Logistics".to_string(),
                category: "shipping".to_string(),
                base_url: "https://api.goshippo.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn get_rates(&self, address_to: &str, parcel: &str) -> Result<Vec<ShippingRate>, String> {
        self.client.get_rates(address_to, parcel).await
    }
}
