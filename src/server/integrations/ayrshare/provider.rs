use super::client::AyrshareClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct AyrshareProvider {
    _client: Arc<AyrshareClient>,
    metadata: ProviderMetadata,
}

impl AyrshareProvider {
    pub fn new(api_key: String) -> Self {
        let client = AyrshareClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "ayrshare".to_string(),
                name: "Ayrshare".to_string(),
                category: "social_media".to_string(),
                base_url: "https://app.ayrshare.com/api".to_string(),
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

impl AyrshareProvider {
    pub async fn post_message(&self, message: &str, platforms: Vec<&str>) -> Result<(), String> {
        self._client.post_message(message, platforms).await
    }
}
