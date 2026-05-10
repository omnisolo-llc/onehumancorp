use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::JitsiClient;
use std::sync::Arc;

pub struct JitsiProvider {
    metadata: ProviderMetadata,
    client: Option<Arc<JitsiClient>>,
}

impl JitsiProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "jitsi".to_string(),
                name: "Jitsi Meet".to_string(),
                category: "video_conferencing".to_string(),
                base_url: "https://meet.jit.si".to_string(),
            },
            client: None,
        }
    }

    pub fn with_base_url(base_url: String) -> Self {
        let mut provider = Self::new();
        provider.client = Some(Arc::new(JitsiClient::new(base_url)));
        provider
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn create_meeting_room(&self, room_prefix: &str) -> Result<String, String> {
         if let Some(client) = &self.client {
            client.generate_meeting_link(room_prefix).await
        } else {
            Err("Jitsi client not initialized".to_string())
        }
    }
}
