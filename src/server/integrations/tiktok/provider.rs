use crate::integrations::catalog::ProviderMetadata;

pub struct TiktokProvider {
    metadata: ProviderMetadata,
}

impl TiktokProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "tiktok".to_string(),
                name: "TikTok API".to_string(),
                category: "social".to_string(),
                base_url: "https://open.tiktokapis.com".to_string(),
            },
        }
    }
}
