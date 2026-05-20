use super::client::{CalComClientWrapper, RealCalComClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct CalComProvider {
    client: Arc<dyn CalComClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl CalComProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealCalComClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "cal_com".to_string(),
                name: "Cal.com Scheduling".to_string(),
                category: "booking".to_string(),
                base_url: "https://api.cal.com/v1".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn CalComClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "cal_com".to_string(),
                name: "Cal.com Scheduling".to_string(),
                category: "booking".to_string(),
                base_url: "https://api.cal.com/v1".to_string(),
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
        calls: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl CalComClientWrapper for MockCalComClient {
        async fn get_bookings(&self) -> Result<(), String> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_cal_com_provider_integration() {
        let calls = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockCalComClient { calls: calls.clone() });
        let provider = CalComProvider::with_client(mock);

        provider.get_bookings().await.unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_cal_com_provider_new() {
        let provider = CalComProvider::new("token".to_string());
        assert_eq!(provider.metadata.id, "cal_com");
        assert_eq!(provider.metadata.category, "booking");
    }

    #[test]
    fn test_cal_com_provider_to_integration_provider() {
        let provider = CalComProvider::new("token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "cal_com");
    }
}
