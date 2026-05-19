use super::client::CalComClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct CalComProvider {
    pub client: CalComClient,
    pub metadata: ProviderMetadata,
}

impl CalComProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: CalComClient::new(api_key),
            metadata: ProviderMetadata {
                id: "calcom".to_string(),
                name: "Cal.com Scheduling".to_string(),
                category: "scheduling".to_string(),
                base_url: "https://api.cal.com".to_string(),
            },
        }
    }

    pub fn to_integration_provider(&self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata.clone(),
        }
    }
}
