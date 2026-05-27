use super::client::MailerLiteClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct MailerLiteProvider {
    _client: Arc<MailerLiteClient>,
    pub metadata: ProviderMetadata,
}

impl MailerLiteProvider {
    pub fn new(api_key: String) -> Self {
        let client = MailerLiteClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "mailerlite".to_string(),
                name: "MailerLite".to_string(),
                category: "marketing".to_string(),
                base_url: "https://connect.mailerlite.com/api".to_string(),
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

    pub async fn sync_customer(&self, email: &str, name: &str) -> Result<(), String> {
        self._client.sync_customer(email, name).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mailerlite_provider_new() {
        let provider = MailerLiteProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "mailerlite");
        assert_eq!(provider.metadata.category, "marketing");
    }

    #[test]
    fn test_mailerlite_provider_into() {
        let provider = MailerLiteProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "mailerlite");
    }
}
