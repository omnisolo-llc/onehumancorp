use super::client::{DailyCoClientWrapper, RealDailyCoClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct DailyCoProvider {
    client: Arc<dyn DailyCoClientWrapper>,
    metadata: ProviderMetadata,
}

impl DailyCoProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealDailyCoClient::new(api_key);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "dailyco".to_string(),
                name: "Daily.co".to_string(),
                category: "video".to_string(),
                base_url: "https://api.daily.co".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn DailyCoClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "dailyco".to_string(),
                name: "Daily.co".to_string(),
                category: "video".to_string(),
                base_url: "https://api.daily.co".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn create_room(&self, booking_id: &str) -> Result<String, String> {
        self.client.create_room(booking_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockDailyCoClient {
        rooms_created: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl DailyCoClientWrapper for MockDailyCoClient {
        async fn create_room(&self, booking_id: &str) -> Result<String, String> {
            self.rooms_created.fetch_add(1, Ordering::SeqCst);
            Ok(format!("https://ohc.daily.co/mock-{}", booking_id))
        }
    }

    #[tokio::test]
    async fn test_dailyco_provider_integration() {
        let created = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockDailyCoClient { rooms_created: created.clone() });
        let provider = DailyCoProvider::with_client(mock);

        let res = provider.create_room("booking1").await.unwrap();
        assert_eq!(res, "https://ohc.daily.co/mock-booking1");
        assert_eq!(created.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_dailyco_provider_new() {
        let provider = DailyCoProvider::new("token".to_string());
        assert_eq!(provider.metadata.id, "dailyco");
        assert_eq!(provider.metadata.category, "video");
    }

    #[test]
    fn test_dailyco_provider_into() {
        let provider = DailyCoProvider::new("token".to_string());
        let integration = provider.into_integration_provider();
        assert_eq!(integration.metadata.id, "dailyco");
    }
}
