use super::client::AcuityClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct AcuityProvider {
    _client: Arc<AcuityClient>,
    metadata: ProviderMetadata,
}

impl AcuityProvider {
    pub fn new(api_key: String) -> Self {
        let client = AcuityClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "acuity".to_string(),
                name: "Acuity Scheduling".to_string(),
                category: "calendar".to_string(),
                base_url: "https://acuityscheduling.com/api/v1".to_string(),
            },
        }
    }

    pub fn to_integration_provider(&self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: ProviderMetadata {
                id: self.metadata.id.clone(),
                name: self.metadata.name.clone(),
                category: self.metadata.category.clone(),
                base_url: self.metadata.base_url.clone(),
            }
        }
    }
}
