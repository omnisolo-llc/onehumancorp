use super::client::{AyrshareClientWrapper, RealAyrshareClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct AyrshareProvider {
    client: Arc<dyn AyrshareClientWrapper>,
    metadata: ProviderMetadata,
}

impl AyrshareProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealAyrshareClient::new(api_key);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "ayrshare".to_string(),
                name: "Ayrshare Social".to_string(),
                category: "social".to_string(),
                base_url: "https://api.ayrshare.com/api".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn cross_post(&self, post: &str, platforms: Vec<String>) -> Result<String, String> {
        self.client.post_message(post, platforms).await
    }

    pub async fn fetch_inbox(&self) -> Result<Vec<String>, String> {
        self.client.get_inbox().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockAyrshareClient;

    #[async_trait]
    impl AyrshareClientWrapper for MockAyrshareClient {
        async fn post_message(&self, _post: &str, _platforms: Vec<String>) -> Result<String, String> {
            Ok("ayr_test".to_string())
        }
        async fn get_inbox(&self) -> Result<Vec<String>, String> {
            Ok(vec!["msg1".to_string()])
        }
    }

    #[tokio::test]
    async fn test_ayrshare_provider() {
        let provider = AyrshareProvider {
            client: Arc::new(MockAyrshareClient),
            metadata: ProviderMetadata {
                id: "ayrshare".to_string(),
                name: "Ayrshare".to_string(),
                category: "social".to_string(),
                base_url: "url".to_string(),
            },
        };
        let post = provider.cross_post("test", vec![]).await.unwrap();
        assert_eq!(post, "ayr_test");
        let inbox = provider.fetch_inbox().await.unwrap();
        assert_eq!(inbox.len(), 1);
    }
}
