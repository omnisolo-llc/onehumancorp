use std::sync::Arc;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::{JitsiClientWrapper, RealJitsiClient};

pub struct JitsiProvider {
    pub client: Arc<dyn JitsiClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl JitsiProvider {
    pub fn new(base_url: String) -> Self {
        let client = RealJitsiClient::new(base_url.clone());

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "jitsi".to_string(),
                name: "Jitsi Meet".to_string(),
                category: "video".to_string(),
                base_url,
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }
}
