use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct ZoomProvider {
    metadata: ProviderMetadata,
}

impl ZoomProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "zoom".to_string(),
                name: "Zoom API".to_string(),
                category: "video".to_string(),
                base_url: "https://api.zoom.us/v2".to_string(),
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
            },
        }
    }
}
