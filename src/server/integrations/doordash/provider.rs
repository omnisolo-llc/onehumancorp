use super::client::{DoorDashClient, DeliveryQuote};
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct DoorDashProvider {
    _client: Arc<DoorDashClient>,
    metadata: ProviderMetadata,
}

impl DoorDashProvider {
    pub fn new(api_key: String) -> Self {
        let client = DoorDashClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "doordash".to_string(),
                name: "DoorDash Drive".to_string(),
                category: "delivery".to_string(),
                base_url: "https://openapi.doordash.com".to_string(),
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

impl DoorDashProvider {
    pub async fn get_delivery_quote(&self, pickup_address: &str, dropoff_address: &str) -> Result<DeliveryQuote, String> {
        self._client.get_delivery_quote(pickup_address, dropoff_address).await
    }

    pub async fn dispatch_delivery(&self, pickup_address: &str, dropoff_address: &str, order_id: &str) -> Result<String, String> {
        self._client.dispatch_delivery(pickup_address, dropoff_address, order_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doordash_provider_new() {
        let provider = DoorDashProvider::new("dummy_key".to_string());
        assert_eq!(provider.metadata.id, "doordash");
        assert_eq!(provider.metadata.category, "delivery");
        assert_eq!(provider.metadata.name, "DoorDash Drive");
    }

    #[test]
    fn test_doordash_provider_into() {
        let provider = DoorDashProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "doordash");
        assert_eq!(integration.metadata.category, "delivery");
    }
}
