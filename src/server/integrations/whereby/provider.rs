use super::client::WherebyClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct WherebyProvider {
    _client: Arc<WherebyClient>,
    pub metadata: ProviderMetadata,
}

impl WherebyProvider {
    pub fn new(api_key: String) -> Self {
        let client = WherebyClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "whereby".to_string(),
                name: "Whereby Video Conferencing".to_string(),
                category: "video".to_string(),
                base_url: "https://api.whereby.dev/v1".to_string(),
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

    pub async fn generate_meeting_for_booking(&self, _booking_id: &str, topic: &str) -> Result<String, String> {
        let link = self.create_meeting(topic).await?;
        Ok(link)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whereby_provider_new() {
        let provider = WherebyProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "whereby");
        assert_eq!(provider.metadata.category, "video");
    }

    #[test]
    fn test_whereby_provider_into() {
        let provider = WherebyProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "whereby");
    }
}
