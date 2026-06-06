use super::client::SquareClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;
use serde_json::Value;

pub struct SquareProvider {
    client: Arc<SquareClient>,
    metadata: ProviderMetadata,
}

impl SquareProvider {
    pub fn new(access_token: String) -> Self {
        let client = SquareClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "square".to_string(),
                name: "Square POS".to_string(),
                category: "pos".to_string(),
                base_url: "https://connect.squareup.com/v2".to_string(),
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

    pub async fn get_catalog(&self) -> Result<Value, String> {
        self.client.get_catalog().await
    }

    pub async fn get_inventory(&self) -> Result<Value, String> {
        self.client.get_inventory().await
    }

    pub async fn update_inventory_count(&self, catalog_object_id: &str, quantity: i32, location_id: &str, state: &str) -> Result<Value, String> {
        self.client.update_inventory_count(catalog_object_id, quantity, location_id, state).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_init() {
        let provider = SquareProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "square");
        assert_eq!(integration.metadata.name, "Square POS");
    }
}
