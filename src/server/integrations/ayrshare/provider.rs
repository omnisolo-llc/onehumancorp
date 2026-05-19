use super::client::{AyrshareClientWrapper, RealAyrshareClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct AyrshareProvider {
    client: Arc<dyn AyrshareClientWrapper>,
    metadata: ProviderMetadata,
}

impl AyrshareProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealAyrshareClient::new(api_key);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "ayrshare".to_string(),
                name: "Ayrshare Social".to_string(),
                category: "social".to_string(),
                base_url: "https://api.ayrshare.com".to_string(),
            },
        }
    }

    pub fn to_integration_provider(&self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata.clone(),
        }
    }
}
