use super::client::ListmonkClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ListmonkProvider {
    _client: Arc<ListmonkClient>,
    metadata: ProviderMetadata,
}

impl ListmonkProvider {
    pub fn new(api_key: String) -> Self {
        let client = ListmonkClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "listmonk".to_string(),
                name: "Listmonk".to_string(),
                category: "email_marketing".to_string(),
                base_url: "http://localhost:9000/api".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
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

impl ListmonkProvider {
    pub async fn send_campaign(&self, list_id: &str, template_id: &str, subject: &str, body: &str) -> Result<(), String> {
        self._client.send_campaign(list_id, template_id, subject, body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_listmonk_provider_new() {
        let provider = ListmonkProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "listmonk");
    }

    #[test]
    fn test_listmonk_provider_into() {
        let provider = ListmonkProvider::new("test_token".to_string());
        let integration = provider.into_integration_provider();
        assert_eq!(integration.metadata.id, "listmonk");
    }
}
