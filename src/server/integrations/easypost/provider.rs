use super::client::{EasyPostClientWrapper, RealEasyPostClient, PackageDimensions};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct EasyPostProvider {
    client: Arc<dyn EasyPostClientWrapper>,
    metadata: ProviderMetadata,
}

impl EasyPostProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealEasyPostClient::new(api_key);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "easypost".to_string(),
                name: "EasyPost".to_string(),
                category: "shipping".to_string(),
                base_url: "https://api.easypost.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn EasyPostClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "easypost".to_string(),
                name: "EasyPost".to_string(),
                category: "shipping".to_string(),
                base_url: "https://api.easypost.com".to_string(),
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

    pub async fn calculate_shipping_rates(&self, destination_zip: &str, dimensions: &PackageDimensions) -> Result<f64, String> {
        self.client.calculate_shipping_rates(destination_zip, dimensions).await
    }

    pub async fn create_shipping_label(&self, order_id: &str, dimensions: &PackageDimensions) -> Result<String, String> {
        self.client.create_shipping_label(order_id, dimensions).await
    }
}
