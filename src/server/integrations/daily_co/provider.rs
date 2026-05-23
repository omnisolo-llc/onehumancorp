use super::client::DailyCoClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct DailyCoProvider {
    _client: Arc<DailyCoClient>,
    metadata: ProviderMetadata,
}

impl DailyCoProvider {
    pub fn new(api_key: String) -> Self {
        let client = DailyCoClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "daily_co".to_string(),
                name: "Daily.co".to_string(),
                category: "video".to_string(),
                base_url: "https://api.daily.co/v1".to_string(),
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

impl DailyCoProvider {
    pub async fn create_room(&self, room_name: &str) -> Result<String, String> {
        self._client.create_room(room_name).await
    }
}
