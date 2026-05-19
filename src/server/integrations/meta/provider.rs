use crate::integrations::catalog::ProviderMetadata;

pub struct MetaProvider {
    metadata: ProviderMetadata,
}

impl MetaProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "meta".to_string(),
                name: "Meta Graph API".to_string(),
                category: "social".to_string(),
                base_url: "https://graph.facebook.com".to_string(),
            },
        }
    }
}
