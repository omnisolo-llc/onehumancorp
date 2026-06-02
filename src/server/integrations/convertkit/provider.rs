use super::client::ConvertKitClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ConvertKitProvider {
    _client: Arc<ConvertKitClient>,
    metadata: ProviderMetadata,
}

impl ConvertKitProvider {
    pub fn new(api_secret: String) -> Self {
        let client = ConvertKitClient::new(api_secret);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "convertkit".to_string(),
                name: "ConvertKit".to_string(),
                category: "marketing".to_string(),
                base_url: "https://api.convertkit.com/v3".to_string(),
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
    use crate::integrations::convertkit::provider::ConvertKitProvider;

    #[test]
    fn test_convertkit_provider_metadata() {
        let provider = ConvertKitProvider::new("test_token".to_string());
        assert_eq!(provider.to_integration_provider().metadata.id, "convertkit");
        assert_eq!(provider.to_integration_provider().metadata.category, "marketing");
    }
}
