use super::client::EasyPostClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct EasyPostProvider {
    _client: Arc<EasyPostClient>,
    metadata: ProviderMetadata,
}

impl EasyPostProvider {
    pub fn new(api_key: String) -> Self {
        let client = EasyPostClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "easypost".to_string(),
                name: "EasyPost".to_string(),
                category: "shipping".to_string(),
                base_url: "https://api.easypost.com/v2".to_string(),
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

impl EasyPostProvider {
    pub async fn create_shipment(&self, to_address: &str, from_address: &str, parcel_details: &str) -> Result<String, String> {
        self._client.create_shipment(to_address, from_address, parcel_details).await
    }
}
