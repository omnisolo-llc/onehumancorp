use super::client::{NylasClientWrapper, RealNylasClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct NylasProvider {
    client: Arc<dyn NylasClientWrapper>,
    metadata: ProviderMetadata,
}

impl NylasProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealNylasClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "nylas".to_string(),
                name: "Nylas".to_string(),
                category: "calendar".to_string(),
                base_url: "https://api.nylas.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn NylasClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "nylas".to_string(),
                name: "Nylas".to_string(),
                category: "calendar".to_string(),
                base_url: "https://api.nylas.com".to_string(),
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

    pub async fn get_free_busy(&self, time_min: &str, time_max: &str) -> Result<String, String> {
        self.client.get_free_busy(time_min, time_max).await
    }

    pub async fn create_event(&self, summary: &str, start_time: &str, end_time: &str) -> Result<String, String> {
        self.client.create_event(summary, start_time, end_time).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockNylasClient;

    #[async_trait]
    impl NylasClientWrapper for MockNylasClient {
        async fn get_free_busy(&self, time_min: &str, time_max: &str) -> Result<String, String> {
            Ok(format!("mock free/busy from {} to {}", time_min, time_max))
        }

        async fn create_event(&self, summary: &str, start_time: &str, end_time: &str) -> Result<String, String> {
            Ok(format!("mock event '{}' from {} to {}", summary, start_time, end_time))
        }
    }

    #[tokio::test]
    async fn test_nylas_provider_new() {
        let provider = NylasProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "nylas");
        assert_eq!(provider.metadata.category, "calendar");
    }

    #[tokio::test]
    async fn test_nylas_provider_into() {
        let provider = NylasProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "nylas");
    }

    #[tokio::test]
    async fn test_nylas_provider_get_free_busy() {
        let mock_client = Arc::new(MockNylasClient);
        let provider = NylasProvider::with_client(mock_client);
        let result = provider.get_free_busy("2023-01-01T00:00:00Z", "2023-01-02T00:00:00Z").await;
        assert_eq!(result.unwrap(), "mock free/busy from 2023-01-01T00:00:00Z to 2023-01-02T00:00:00Z");
    }

    #[tokio::test]
    async fn test_nylas_provider_create_event() {
        let mock_client = Arc::new(MockNylasClient);
        let provider = NylasProvider::with_client(mock_client);
        let result = provider.create_event("Test Event", "2023-01-01T00:00:00Z", "2023-01-01T01:00:00Z").await;
        assert_eq!(result.unwrap(), "mock event 'Test Event' from 2023-01-01T00:00:00Z to 2023-01-01T01:00:00Z");
    }
}
