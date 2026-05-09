use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct JitsiProvider {
    pub metadata: IntegrationProvider,
}

impl JitsiProvider {
    pub fn new() -> Self {
        Self {
            metadata: IntegrationProvider {
                metadata: ProviderMetadata {
                    id: "jitsi".to_string(),
                    name: "Jitsi Meet".to_string(),
                    category: "Video Conferencing".to_string(),
                    base_url: "https://meet.jit.si".to_string(),
                }
            },
        }
    }
}
