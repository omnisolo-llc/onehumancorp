use super::client::JitsiClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct JitsiProvider {
    pub client: JitsiClient,
    pub metadata: ProviderMetadata,
}

impl JitsiProvider {
    pub fn new(domain: String) -> Self {
        Self {
            client: JitsiClient::new(domain.clone()),
            metadata: ProviderMetadata {
                id: "jitsi".to_string(),
                name: "Jitsi Meet".to_string(),
                category: "video".to_string(),
                base_url: format!("https://{}", domain),
            },
        }
    }

    pub fn to_integration_provider(&self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata.clone(),
        }
    }
}
