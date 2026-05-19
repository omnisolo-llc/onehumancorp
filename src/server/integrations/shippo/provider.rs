use super::client::{ShippoClientWrapper, RealShippoClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ShippoProvider {
    client: Arc<dyn ShippoClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl ShippoProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealShippoClient::new(api_key);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "shippo".to_string(),
                name: "Shippo".to_string(),
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
            },
        }
    }

    pub async fn create_shipment(&self, address_to: &str, address_from: &str, parcel: &str) -> Result<String, String> {
        self.client.create_shipment(address_to, address_from, parcel).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockShippoClient {
        created_shipments: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl ShippoClientWrapper for MockShippoClient {
        async fn create_shipment(&self, _address_to: &str, _address_from: &str, _parcel: &str) -> Result<String, String> {
            self.created_shipments.fetch_add(1, Ordering::SeqCst);
            Ok("mock_id".to_string())
        }
    }

    #[tokio::test]
    async fn test_shippo_provider_integration() {
        let created = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockShippoClient { created_shipments: created.clone() });
        let provider = ShippoProvider::with_client(mock);

        provider.create_shipment("to", "from", "parcel").await.unwrap();
        assert_eq!(created.load(Ordering::SeqCst), 1);
    }
}
