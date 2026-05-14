use std::sync::Arc;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::{ManyChatClientWrapper, RealManyChatClient};

pub struct ManyChatProvider {
    pub client: Arc<dyn ManyChatClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl ManyChatProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealManyChatClient::new(api_key);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "manychat".to_string(),
                name: "ManyChat Integration".to_string(),
                category: "social_media".to_string(),
                base_url: "https://api.manychat.com".to_string(),
            },
        }
    }
}
