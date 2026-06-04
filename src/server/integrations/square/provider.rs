use super::client::SquareClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;
use serde_json::Value;

pub struct SquareProvider {
    _client: Arc<SquareClient>,
    metadata: ProviderMetadata,
}

impl SquareProvider {
    pub fn new(access_token: String) -> Self {
        let client = SquareClient::new(access_token);

        Self {
            _client: Arc::new(client),
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

    pub async fn list_catalog(&self) -> Result<Value, String> {
        self._client.list_catalog().await
    }

    pub async fn batch_retrieve_inventory_counts(&self, catalog_object_ids: Vec<String>) -> Result<Value, String> {
        self._client.batch_retrieve_inventory_counts(catalog_object_ids).await
    }

    pub async fn batch_change_inventory(&self, idempotency_key: String, physical_count: i32, catalog_object_id: String, location_id: String, state: String) -> Result<Value, String> {
        self._client.batch_change_inventory(idempotency_key, physical_count, catalog_object_id, location_id, state).await
    }
}
