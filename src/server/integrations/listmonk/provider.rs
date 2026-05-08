use super::client::{ListmonkClientWrapper, RealListmonkClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ListmonkProvider {
    client: Arc<dyn ListmonkClientWrapper>,
    metadata: ProviderMetadata,
}

impl ListmonkProvider {
    pub fn new(base_url: String, username: String, password: Option<String>) -> Self {
        let client = RealListmonkClient::new(base_url.clone(), username, password);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "listmonk".to_string(),
                name: "Listmonk Email Marketing".to_string(),
                category: "email".to_string(),
                base_url,
            },
        }
    }

    pub fn with_client(client: Arc<dyn ListmonkClientWrapper>, base_url: String) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "listmonk".to_string(),
                name: "Listmonk Email Marketing".to_string(),
                category: "email".to_string(),
                base_url,
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn send_campaign(&self, campaign_id: i32) -> Result<(), String> {
        self.client.send_campaign(campaign_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockListmonkClient {
        sent_campaigns: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl ListmonkClientWrapper for MockListmonkClient {
        async fn send_campaign(&self, _campaign_id: i32) -> Result<(), String> {
            self.sent_campaigns.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_listmonk_provider_integration() {
        let sent = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockListmonkClient { sent_campaigns: sent.clone() });
        let provider = ListmonkProvider::with_client(mock, "http://localhost:9000".to_string());

        provider.send_campaign(1).await.unwrap();
        assert_eq!(sent.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_listmonk_provider_new() {
        let provider = ListmonkProvider::new("http://localhost:9000".to_string(), "admin".to_string(), Some("pass".to_string()));
        assert_eq!(provider.metadata.id, "listmonk");
        assert_eq!(provider.metadata.category, "email");
    }

    #[test]
    fn test_listmonk_provider_into() {
        let provider = ListmonkProvider::new("http://localhost:9000".to_string(), "admin".to_string(), Some("pass".to_string()));
        let integration = provider.into_integration_provider();
        assert_eq!(integration.metadata.id, "listmonk");
    }
}
