use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct TikTokProvider {
    pub metadata: ProviderMetadata,
}

impl TikTokProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "tiktok".to_string(),
                name: "TikTok".to_string(),
                category: "social".to_string(),
                base_url: "https://api.tiktok.com".to_string(),
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
