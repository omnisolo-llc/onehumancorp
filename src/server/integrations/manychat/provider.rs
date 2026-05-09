use std::sync::Arc;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::ManychatClient;

pub struct ManychatProvider {
    client: Arc<ManychatClient>,
}

impl ManychatProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Arc::new(ManychatClient::new(api_key)),
        }
    }

    pub fn get_metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            id: "manychat".to_string(),
            name: "Manychat".to_string(),
            category: "social_media".to_string(),
            base_url: "https://api.manychat.com".to_string(),
        }
    }
}
