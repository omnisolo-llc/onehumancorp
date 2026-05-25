use super::client::{ManychatClientWrapper, RealManychatClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ManychatProvider {
    _client: Arc<dyn ManychatClientWrapper>,
    pub metadata: ProviderMetadata,
    pub tenant_id: String,
}

impl ManychatProvider {
    pub fn new(api_key: String, tenant_id: String) -> Self {
        let client = RealManychatClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "manychat".to_string(),
                name: "Manychat".to_string(),
                category: "social_media".to_string(),
                base_url: "https://api.manychat.com".to_string(),
            },
            tenant_id,
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

    pub async fn fetch_inbox(&self) -> Result<Vec<String>, String> {
        self._client.fetch_inbox().await
    }

    pub async fn send_reply(&self, platform: &str, to: &str, body: &str) -> Result<(), String> {
        self._client.send_reply(platform, to, body).await
    }

    pub async fn get_oauth_url(&self, redirect_uri: &str) -> String {
        self._client.get_oauth_url(redirect_uri).await
    }

    pub async fn exchange_token(&self, code: &str, redirect_uri: &str) -> Result<String, String> {
        self._client.exchange_token(code, redirect_uri).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manychat_provider_new() {
        let provider = ManychatProvider::new("test_token".to_string(), "t1".to_string());
        assert_eq!(provider.metadata.id, "manychat");
        assert_eq!(provider.metadata.category, "social_media");
        assert_eq!(provider.tenant_id, "t1");
    }

    #[test]
    fn test_manychat_provider_into() {
        let provider = ManychatProvider::new("test_token".to_string(), "t1".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "manychat");
    }
}
