use super::client::JitsiClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct JitsiProvider {
    _client: Arc<JitsiClient>,
    metadata: ProviderMetadata,
}

impl JitsiProvider {
    pub fn new(api_key: String) -> Self {
        let client = JitsiClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "jitsi".to_string(),
                name: "Jitsi Meet".to_string(),
                category: "video_conferencing".to_string(),
                base_url: "https://api.jitsi.net".to_string(),
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

impl JitsiProvider {
    pub async fn create_meeting(&self, meeting_name: &str) -> Result<String, String> {
        self._client.create_meeting(meeting_name).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jitsi_provider_new() {
        let provider = JitsiProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "jitsi");
    }

    #[test]
    fn test_jitsi_provider_into() {
        let provider = JitsiProvider::new("test_token".to_string());
        let integration = provider.into_integration_provider();
        assert_eq!(integration.metadata.id, "jitsi");
    }

    #[tokio::test]
    async fn test_jitsi_provider_create_meeting() {
        let provider = JitsiProvider::new("test_token".to_string());
        let result = provider.create_meeting("test").await;
        assert!(result.is_ok());
    }
}
