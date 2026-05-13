use super::client::{CalComClientWrapper, RealCalComClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct CalComProvider {
    client: Arc<dyn CalComClientWrapper>,
    metadata: ProviderMetadata,
}

impl CalComProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealCalComClient::new(api_key);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "calcom".to_string(),
                name: "Cal.com".to_string(),
                category: "scheduling".to_string(),
                base_url: "https://api.cal.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn CalComClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "calcom".to_string(),
                name: "Cal.com".to_string(),
                category: "scheduling".to_string(),
                base_url: "https://api.cal.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn create_booking_link(&self, user_id: &str, hours: &str) -> Result<String, String> {
        self.client.create_booking_link(user_id, hours).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockCalComClient {
        links_created: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl CalComClientWrapper for MockCalComClient {
        async fn create_booking_link(&self, user_id: &str, _hours: &str) -> Result<String, String> {
            self.links_created.fetch_add(1, Ordering::SeqCst);
            Ok(format!("mock_link_{}", user_id))
        }
    }

    #[tokio::test]
    async fn test_calcom_provider_integration() {
        let created = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockCalComClient { links_created: created.clone() });
        let provider = CalComProvider::with_client(mock);

        let res = provider.create_booking_link("leo", "9-5").await.unwrap();
        assert_eq!(res, "mock_link_leo");
        assert_eq!(created.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_calcom_provider_new() {
        let provider = CalComProvider::new("token".to_string());
        assert_eq!(provider.metadata.id, "calcom");
        assert_eq!(provider.metadata.category, "scheduling");
    }

    #[test]
    fn test_calcom_provider_into() {
        let provider = CalComProvider::new("token".to_string());
        let integration = provider.into_integration_provider();
        assert_eq!(integration.metadata.id, "calcom");
    }
}
