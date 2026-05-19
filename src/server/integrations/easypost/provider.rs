use super::client::EasyPostClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct EasyPostProvider {
    pub client: EasyPostClient,
    pub metadata: ProviderMetadata,
}

impl EasyPostProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: EasyPostClient::new(api_key),
            metadata: ProviderMetadata {
                id: "easypost".to_string(),
                name: "EasyPost Shipping".to_string(),
                category: "logistics".to_string(),
                base_url: "https://api.easypost.com".to_string(),
            },
        }
    }

    pub fn to_integration_provider(&self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata.clone(),
        }
    }
}
