use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use super::client::{ShipEngineClientWrapper, RealShipEngineClient};
use std::sync::Arc;

pub struct ShipEngineProvider {
    client: Arc<dyn ShipEngineClientWrapper>,
    metadata: ProviderMetadata,
}

impl ShipEngineProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealShipEngineClient::new(api_key);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "shipengine".to_string(),
                name: "ShipEngine Logistics API".to_string(),
                category: "logistics".to_string(),
                base_url: "https://api.shipengine.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn ShipEngineClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "shipengine".to_string(),
                name: "ShipEngine Logistics API".to_string(),
                category: "logistics".to_string(),
                base_url: "https://api.shipengine.com".to_string(),
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

    pub async fn generate_label(&self, address: &str) -> Result<String, String> {
        self.client.generate_label(address).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockShipEngineClient;

    #[async_trait]
    impl ShipEngineClientWrapper for MockShipEngineClient {
        async fn generate_label(&self, _address: &str) -> Result<String, String> {
            Ok("mock_label".to_string())
        }
    }

    #[test]
    fn test_shipengine_provider_metadata() {
        let provider = ShipEngineProvider::new("key".to_string());
        assert_eq!(provider.metadata.id, "shipengine");
    }

    #[tokio::test]
    async fn test_shipengine_generate_label() {
        let provider = ShipEngineProvider::with_client(Arc::new(MockShipEngineClient));
        let url = provider.generate_label("123 Main St").await.unwrap();
        assert_eq!(url, "mock_label");
    }
}
