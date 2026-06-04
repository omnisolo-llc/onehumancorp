use super::client::ShipengineClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ShipengineProvider {
    _client: Arc<ShipengineClient>,
    metadata: ProviderMetadata,
}

impl ShipengineProvider {
    pub fn new(api_key: String) -> Self {
        let client = ShipengineClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "shipengine".to_string(),
                name: "ShipEngine".to_string(),
                category: "logistics".to_string(),
                base_url: "https://api.shipengine.com/v1".to_string(),
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

#[cfg(test)]
mod tests {
    use crate::provider::ShipengineProvider;

    #[test]
    fn test_shipengine_provider_metadata() {
        let provider = ShipengineProvider::new("test_token".to_string());
        assert_eq!(provider.to_integration_provider().metadata.id, "shipengine");
        assert_eq!(provider.to_integration_provider().metadata.category, "logistics");
    }
}
