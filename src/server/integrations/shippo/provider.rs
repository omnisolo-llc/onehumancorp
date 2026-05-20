use super::client::{ShippoClientWrapper, RealShippoClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ShippoProvider {
    client: Arc<dyn ShippoClientWrapper>,
    metadata: ProviderMetadata,
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
            }
        }
    }

    pub async fn get_shipping_rates(&self, from_zip: &str, to_zip: &str, weight: f32) -> Result<String, String> {
        self.client.get_shipping_rates(from_zip, to_zip, weight).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockShippoClient {
        called: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl ShippoClientWrapper for MockShippoClient {
        async fn get_shipping_rates(&self, _from_zip: &str, _to_zip: &str, _weight: f32) -> Result<String, String> {
            self.called.fetch_add(1, Ordering::SeqCst);
            Ok("mock_rate".to_string())
        }
    }

    #[tokio::test]
    async fn test_shippo_provider_integration() {
        let called = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockShippoClient { called: called.clone() });
        let provider = ShippoProvider::with_client(mock);

        provider.get_shipping_rates("12345", "67890", 1.0).await.unwrap();
        assert_eq!(called.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_shippo_provider_new() {
        let provider = ShippoProvider::new("key".to_string());
        assert_eq!(provider.metadata.id, "shippo");
        assert_eq!(provider.metadata.category, "shipping");
    }

    #[test]
    fn test_shippo_provider_to_integration_provider() {
        let provider = ShippoProvider::new("key".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "shippo");
    }
}
