use std::sync::Arc;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::ShippoClient;

pub struct ShippoProvider {
    client: Arc<ShippoClient>,
}

impl ShippoProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Arc::new(ShippoClient::new(api_key)),
        }
    }

    pub fn get_metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            id: "shippo".to_string(),
            name: "Shippo".to_string(),
            category: "shipping".to_string(),
            base_url: "https://api.goshippo.com".to_string(),
        }
    }
}
