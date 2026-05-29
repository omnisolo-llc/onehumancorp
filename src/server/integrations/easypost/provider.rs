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

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: ProviderMetadata {
                id: self.metadata.id,
                name: self.metadata.name,
                category: self.metadata.category,
                base_url: self.metadata.base_url,
            }
        }
    }
}

impl EasyPostProvider {
    pub async fn create_shipment(&self, to_address: &str, from_address: &str, parcel_details: &str) -> Result<String, String> {
        self._client.create_shipment(to_address, from_address, parcel_details).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easypost_provider_new() {
        let provider = EasyPostProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "easypost");
    }

    #[test]
    fn test_easypost_provider_into() {
        let provider = EasyPostProvider::new("test_token".to_string());
        let integration = provider.into_integration_provider();
        assert_eq!(integration.metadata.id, "easypost");
    }

    #[tokio::test]
    async fn test_easypost_provider_create_shipment() {
        let provider = EasyPostProvider::new("test_token".to_string());
        let result = provider.create_shipment("to", "from", "parcel").await;
        assert!(result.is_ok());
    }
}
