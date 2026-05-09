use std::sync::Arc;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::{AyrshareClientWrapper, RealAyrshareClient};

pub struct AyrshareProvider {
    pub client: Arc<dyn AyrshareClientWrapper>,
    pub metadata: ProviderMetadata,
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
                base_url: "https://app.ayrshare.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }
}
