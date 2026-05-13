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
                category: "logistics".to_string(),
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
                category: "logistics".to_string(),
                base_url: "https://api.goshippo.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn buy_shipping_label(&self, order_id: &str) -> Result<String, String> {
        self.client.buy_shipping_label(order_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockShippoClient {
        labels_bought: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl ShippoClientWrapper for MockShippoClient {
        async fn buy_shipping_label(&self, order_id: &str) -> Result<String, String> {
            self.labels_bought.fetch_add(1, Ordering::SeqCst);
            Ok(format!("mock_label_{}.pdf", order_id))
        }
    }

    #[tokio::test]
    async fn test_shippo_provider_integration() {
        let bought = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockShippoClient { labels_bought: bought.clone() });
        let provider = ShippoProvider::with_client(mock);

        let res = provider.buy_shipping_label("order1").await.unwrap();
        assert_eq!(res, "mock_label_order1.pdf");
        assert_eq!(bought.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_shippo_provider_new() {
        let provider = ShippoProvider::new("token".to_string());
        assert_eq!(provider.metadata.id, "shippo");
        assert_eq!(provider.metadata.category, "logistics");
    }

    #[test]
    fn test_shippo_provider_into() {
        let provider = ShippoProvider::new("token".to_string());
        let integration = provider.into_integration_provider();
        assert_eq!(integration.metadata.id, "shippo");
    }
}
