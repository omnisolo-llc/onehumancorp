use super::client::JitsiClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
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

impl JitsiProvider {
    pub async fn create_meeting(&self, meeting_name: &str) -> Result<String, String> {
        self._client.create_meeting(meeting_name).await
    }
}
