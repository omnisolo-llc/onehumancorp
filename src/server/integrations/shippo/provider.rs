use super::client::{ShippoClientWrapper, RealShippoClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ShippoProvider {
    _client: Arc<dyn ShippoClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl ShippoProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealShippoClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "shippo".to_string(),
                name: "Shippo".to_string(),
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

    pub async fn fetch_rates(&self, weight: f64, dimensions: &str) -> Result<Vec<String>, String> {
        self._client.fetch_rates(weight, dimensions).await
    }

    pub async fn purchase_label(&self, rate_id: &str) -> Result<String, String> {
        self._client.purchase_label(rate_id).await
    }
}
