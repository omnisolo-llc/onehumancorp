use super::client::{EasyPostClientWrapper, RealEasyPostClient};
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
                name: "EasyPost Shipping".to_string(),
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
                name: "EasyPost Shipping".to_string(),
                category: "shipping".to_string(),
                base_url: "https://api.easypost.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn buy_label(&self, shipment_id: &str, rate_id: &str) -> Result<(), String> {
        self.client.buy_label(shipment_id, rate_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockEasyPostClient {
        bought_labels: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl EasyPostClientWrapper for MockEasyPostClient {
        async fn buy_label(&self, _shipment_id: &str, _rate_id: &str) -> Result<(), String> {
            self.bought_labels.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_easypost_provider_integration() {
        let bought = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockEasyPostClient { bought_labels: bought.clone() });
        let provider = EasyPostProvider::with_client(mock);

        provider.buy_label("shp_123", "rate_123").await.unwrap();
        assert_eq!(bought.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_easypost_provider_new() {
        let provider = EasyPostProvider::new("api_key".to_string());
        assert_eq!(provider.metadata.id, "easypost");
        assert_eq!(provider.metadata.category, "shipping");
    }

    #[test]
    fn test_easypost_provider_into() {
        let provider = EasyPostProvider::new("api_key".to_string());
        let integration = provider.into_integration_provider();
        assert_eq!(integration.metadata.id, "easypost");
    }
}
