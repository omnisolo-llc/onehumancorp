use super::client::NylasClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct NylasProvider {
    _client: Arc<NylasClient>,
    metadata: ProviderMetadata,
}

impl NylasProvider {
    pub fn new(access_token: String) -> Self {
        let client = NylasClient::new(access_token);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "nylas".to_string(),
                name: "Nylas".to_string(),
                category: "calendar".to_string(),
                base_url: "https://api.nylas.com".to_string(),
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
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::provider::NylasProvider;

    #[test]
    fn test_nylas_provider_metadata() {
        let provider = NylasProvider::new("test_token".to_string());
        assert_eq!(provider.to_integration_provider().metadata.id, "nylas");
        assert_eq!(
            provider.to_integration_provider().metadata.category,
            "calendar"
        );
    }
}
