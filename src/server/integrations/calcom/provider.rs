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

    pub async fn generate_booking_link(&self, event_type_id: i32, name: &str, email: &str) -> Result<String, String> {
        self.client.create_booking_link(event_type_id, name, email).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockCalComClient;

    #[async_trait]
    impl CalComClientWrapper for MockCalComClient {
        async fn create_booking_link(&self, event_type_id: i32, _name: &str, _email: &str) -> Result<String, String> {
            Ok(format!("https://cal.com/mock?id={}", event_type_id))
        }
    }

    #[tokio::test]
    async fn test_generate_booking_link() {
        let provider = CalComProvider {
            client: Arc::new(MockCalComClient),
            metadata: ProviderMetadata {
                id: "calcom".to_string(),
                name: "Cal.com".to_string(),
                category: "scheduling".to_string(),
                base_url: "url".to_string(),
            },
        };
        let link = provider.generate_booking_link(123, "Maya", "maya@example.com").await.unwrap();
        assert_eq!(link, "https://cal.com/mock?id=123");
    }
}
