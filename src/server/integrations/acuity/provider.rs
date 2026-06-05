use super::client::AcuityClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct AcuityProvider {
    _client: Arc<AcuityClient>,
    metadata: ProviderMetadata,
}

impl AcuityProvider {
    pub fn new(user_id: String, api_key: String) -> Self {
        let client = AcuityClient::new(user_id, api_key);

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

#[cfg(test)]
mod tests {
    use crate::integrations::acuity::provider::AcuityProvider;

    #[test]
    fn test_acuity_provider_metadata() {
        let provider = AcuityProvider::new("test_user".to_string(), "test_token".to_string());
        assert_eq!(provider.to_integration_provider().metadata.id, "acuity");
        assert_eq!(provider.to_integration_provider().metadata.category, "calendar");
    }
}
