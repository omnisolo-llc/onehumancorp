use super::client::{CalcomClientWrapper, RealCalcomClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;
use serde_json::Value;

pub struct CalcomProvider {
    client: Arc<dyn CalcomClientWrapper>,
    metadata: ProviderMetadata,
}

impl CalcomProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealCalcomClient::new(api_key);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "calcom".to_string(),
                name: "Cal.com Scheduling".to_string(),
                category: "scheduling".to_string(),
                base_url: "https://api.cal.com/v1".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn CalcomClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "calcom".to_string(),
                name: "Cal.com Scheduling".to_string(),
                category: "scheduling".to_string(),
                base_url: "https://api.cal.com/v1".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn get_bookings(&self) -> Result<Value, String> {
        self.client.get_bookings().await
    }

    pub async fn create_booking(&self, booking_data: Value) -> Result<Value, String> {
        self.client.create_booking(booking_data).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use serde_json::json;

    struct MockCalcomClient {
        fetches: Arc<AtomicUsize>,
        creates: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl CalcomClientWrapper for MockCalcomClient {
        async fn get_bookings(&self) -> Result<Value, String> {
            self.fetches.fetch_add(1, Ordering::SeqCst);
            Ok(json!([{"id": "b1", "status": "accepted"}]))
        }

        async fn create_booking(&self, _booking_data: Value) -> Result<Value, String> {
            self.creates.fetch_add(1, Ordering::SeqCst);
            Ok(json!({"status": "success"}))
        }
    }

    #[tokio::test]
    async fn test_calcom_provider_integration() {
        let fetches = Arc::new(AtomicUsize::new(0));
        let creates = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockCalcomClient {
            fetches: fetches.clone(),
            creates: creates.clone(),
        });
        let provider = CalcomProvider::with_client(mock);

        provider.get_bookings().await.unwrap();
        assert_eq!(fetches.load(Ordering::SeqCst), 1);

        provider.create_booking(json!({"eventTypeId": 1})).await.unwrap();
        assert_eq!(creates.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_calcom_provider_new() {
        let provider = CalcomProvider::new("api_key".to_string());
        assert_eq!(provider.metadata.id, "calcom");
        assert_eq!(provider.metadata.category, "scheduling");
    }

    #[test]
    fn test_calcom_provider_into() {
        let provider = CalcomProvider::new("api_key".to_string());
        let integration = provider.into_integration_provider();
        assert_eq!(integration.metadata.id, "calcom");
    }
}
