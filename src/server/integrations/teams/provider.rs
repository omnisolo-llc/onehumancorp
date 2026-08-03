use super::client::TeamsClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct TeamsProvider {
    _client: Arc<TeamsClient>,
    metadata: ProviderMetadata,
}

impl TeamsProvider {
    pub fn new(access_token: String) -> Self {
        let client = TeamsClient::new(access_token);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "teams".to_string(),
                name: "Microsoft Teams".to_string(),
                category: "video".to_string(),
                base_url: "https://graph.microsoft.com/v1.0".to_string(),
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
    use crate::provider::TeamsProvider;

    #[test]
    fn test_teams_provider_metadata() {
        let provider = TeamsProvider::new("test_token".to_string());
        assert_eq!(provider.to_integration_provider().metadata.id, "teams");
        assert_eq!(
            provider.to_integration_provider().metadata.category,
            "video"
        );
    }
}
