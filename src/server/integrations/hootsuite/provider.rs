use super::client::HootsuiteClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct HootsuiteProvider {
    _client: Arc<HootsuiteClient>,
    metadata: ProviderMetadata,
}

impl HootsuiteProvider {
    pub fn new(api_key: String) -> Self {
        let client = HootsuiteClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "hootsuite".to_string(),
                name: "Hootsuite".to_string(),
                category: "social_media".to_string(),
                base_url: "https://platform.hootsuite.com/v1".to_string(),
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

impl HootsuiteProvider {
    pub async fn post_message(&self, message: &str, platforms: Vec<&str>) -> Result<(), String> {
        self._client.post_message(message, platforms).await
    }
}
