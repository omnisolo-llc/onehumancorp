use std::sync::Arc;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::{EasyPostClientWrapper, RealEasyPostClient};

pub struct EasyPostProvider {
    pub client: Arc<dyn EasyPostClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl EasyPostProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealEasyPostClient::new(api_key);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "easypost".to_string(),
                name: "EasyPost Shipping".to_string(),
                category: "logistics".to_string(),
                base_url: "https://api.easypost.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }
}
