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
                category: "logistics".to_string(),
                base_url: "https://api.easypost.com/v2".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn create_shipping_label(&self, to: &str, from: &str, parcel: &str) -> Result<String, String> {
        self.client.create_shipment(to, from, parcel).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockEasyPostClient;

    #[async_trait]
    impl EasyPostClientWrapper for MockEasyPostClient {
        async fn create_shipment(&self, _to_address: &str, _from_address: &str, _parcel: &str) -> Result<String, String> {
            Ok("shp_test".to_string())
        }
    }

    #[tokio::test]
    async fn test_create_shipping_label() {
        let provider = EasyPostProvider {
            client: Arc::new(MockEasyPostClient),
            metadata: ProviderMetadata {
                id: "easypost".to_string(),
                name: "EasyPost".to_string(),
                category: "logistics".to_string(),
                base_url: "url".to_string(),
            },
        };
        let label = provider.create_shipping_label("to", "from", "parcel").await.unwrap();
        assert_eq!(label, "shp_test");
    }
}
