use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::EasyPostClient;
use std::sync::Arc;

pub struct EasyPostProvider {
    metadata: ProviderMetadata,
    client: Option<Arc<EasyPostClient>>,
}

impl EasyPostProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "easypost".to_string(),
                name: "EasyPost Shipping".to_string(),
                category: "shipping".to_string(),
                base_url: "https://api.easypost.com/v2".to_string(),
            },
            client: None,
        }
    }

    pub fn with_api_key(api_key: String) -> Self {
        let mut provider = Self::new();
        provider.client = Some(Arc::new(EasyPostClient::new(api_key)));
        provider
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn get_shipping_rates(&self, from_zip: &str, to_zip: &str, weight_oz: f32) -> Result<Vec<String>, String> {
        if let Some(client) = &self.client {
            client.calculate_shipping_rates(from_zip, to_zip, weight_oz).await
        } else {
            Err("EasyPost client not initialized".to_string())
        }
    }

    pub async fn purchase_label(&self, rate_id: &str) -> Result<String, String> {
        if let Some(client) = &self.client {
            client.buy_shipping_label(rate_id).await
        } else {
            Err("EasyPost client not initialized".to_string())
        }
    }
}
