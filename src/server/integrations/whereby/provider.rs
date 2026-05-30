use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use super::client::{WherebyClientWrapper, RealWherebyClient};
use std::sync::Arc;

pub struct WherebyProvider {
    client: Arc<dyn WherebyClientWrapper>,
    metadata: ProviderMetadata,
}

impl WherebyProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealWherebyClient::new(api_key);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "whereby".to_string(),
                name: "Whereby Video Conferencing".to_string(),
                category: "video".to_string(),
                base_url: "https://api.whereby.dev".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn WherebyClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "whereby".to_string(),
                name: "Whereby Video Conferencing".to_string(),
                category: "video".to_string(),
                base_url: "https://api.whereby.dev".to_string(),
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

    pub async fn create_meeting(&self, meeting_name: &str) -> Result<String, String> {
        self.client.create_meeting(meeting_name).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockWherebyClient;

    #[async_trait]
    impl WherebyClientWrapper for MockWherebyClient {
        async fn create_meeting(&self, meeting_name: &str) -> Result<String, String> {
            Ok(format!("https://whereby.com/mock-{}", meeting_name))
        }
    }

    #[test]
    fn test_whereby_provider_metadata() {
        let provider = WherebyProvider::new("key".to_string());
        assert_eq!(provider.metadata.id, "whereby");
    }

    #[tokio::test]
    async fn test_whereby_create_meeting() {
        let provider = WherebyProvider::with_client(Arc::new(MockWherebyClient));
        let url = provider.create_meeting("testroom").await.unwrap();
        assert!(url.contains("testroom"));
    }
}
