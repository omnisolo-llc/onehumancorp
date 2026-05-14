use std::sync::Arc;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::{CalComClientWrapper, RealCalComClient};

pub struct CalComProvider {
    pub client: Arc<dyn CalComClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl CalComProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealCalComClient::new(api_key);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "calcom".to_string(),
                name: "Cal.com Integration".to_string(),
                category: "calendar".to_string(),
                base_url: "https://api.cal.com/v1".to_string(),
            },
        }
    }
}
