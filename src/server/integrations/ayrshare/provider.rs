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
                name: "Ayrshare Social Media".to_string(),
                category: "social".to_string(),
                base_url: "https://app.ayrshare.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn AyrshareClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "ayrshare".to_string(),
                name: "Ayrshare Social Media".to_string(),
                category: "social".to_string(),
                base_url: "https://app.ayrshare.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn post_message(&self, post: &str, platforms: Vec<&str>) -> Result<(), String> {
        self.client.post_message(post, platforms).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockAyrshareClient {
        sent_messages: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl AyrshareClientWrapper for MockAyrshareClient {
        async fn post_message(&self, _post: &str, _platforms: Vec<&str>) -> Result<(), String> {
            self.sent_messages.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_ayrshare_provider_integration() {
        let sent = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockAyrshareClient { sent_messages: sent.clone() });
        let provider = AyrshareProvider::with_client(mock);

        provider.post_message("Test message", vec!["twitter"]).await.unwrap();
        assert_eq!(sent.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_ayrshare_provider_new() {
        let provider = AyrshareProvider::new("api_key".to_string());
        assert_eq!(provider.metadata.id, "ayrshare");
        assert_eq!(provider.metadata.category, "social");
    }

    #[test]
    fn test_ayrshare_provider_into() {
        let provider = AyrshareProvider::new("api_key".to_string());
        let integration = provider.into_integration_provider();
        assert_eq!(integration.metadata.id, "ayrshare");
    }
}
