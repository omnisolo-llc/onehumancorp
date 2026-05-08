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
                id: "cal_com".to_string(),
                name: "Cal.com Scheduling".to_string(),
                category: "calendar".to_string(),
                base_url: "https://api.cal.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn CalComClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "cal_com".to_string(),
                name: "Cal.com Scheduling".to_string(),
                category: "calendar".to_string(),
                base_url: "https://api.cal.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn get_bookings(&self) -> Result<(), String> {
        self.client.get_bookings().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockCalComClient {
        api_calls: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl CalComClientWrapper for MockCalComClient {
        async fn get_bookings(&self) -> Result<(), String> {
            self.api_calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_cal_com_provider_integration() {
        let calls = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockCalComClient { api_calls: calls.clone() });
        let provider = CalComProvider::with_client(mock);

        provider.get_bookings().await.unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_cal_com_provider_new() {
        let provider = CalComProvider::new("api_key".to_string());
        assert_eq!(provider.metadata.id, "cal_com");
        assert_eq!(provider.metadata.category, "calendar");
    }

    #[test]
    fn test_cal_com_provider_into() {
        let provider = CalComProvider::new("api_key".to_string());
        let integration = provider.into_integration_provider();
        assert_eq!(integration.metadata.id, "cal_com");
    }
}
