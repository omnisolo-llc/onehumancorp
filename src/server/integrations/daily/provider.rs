use super::client::DailyClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct DailyProvider {
    _client: Arc<DailyClient>,
    metadata: ProviderMetadata,
}

impl DailyProvider {
    pub fn new(api_key: String) -> Self {
        let client = DailyClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "daily".to_string(),
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

    pub async fn create_meeting(&self, topic: &str) -> Result<String, String> {
        self._client.create_meeting(topic).await
    }
}
