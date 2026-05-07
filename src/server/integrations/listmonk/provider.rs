use super::client::{ListmonkClientWrapper, RealListmonkClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ListmonkProvider {
    client: Arc<dyn ListmonkClientWrapper>,
    metadata: ProviderMetadata,
}

impl ListmonkProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealListmonkClient::new(api_key);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "listmonk".to_string(),
                name: "Listmonk Marketing".to_string(),
                category: "marketing".to_string(),
                base_url: "http://localhost:9000".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn trigger_marketing_email(&self, title: &str, body: &str, segment_id: i32) -> Result<String, String> {
        self.client.send_campaign(title, body, segment_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockListmonkClient;

    #[async_trait]
    impl ListmonkClientWrapper for MockListmonkClient {
        async fn send_campaign(&self, _title: &str, _body: &str, _segment_id: i32) -> Result<String, String> {
            Ok("list_test".to_string())
        }
    }

    #[tokio::test]
    async fn test_trigger_marketing_email() {
        let provider = ListmonkProvider {
            client: Arc::new(MockListmonkClient),
            metadata: ProviderMetadata {
                id: "listmonk".to_string(),
                name: "Listmonk".to_string(),
                category: "marketing".to_string(),
                base_url: "url".to_string(),
            },
        };
        let res = provider.trigger_marketing_email("title", "body", 1).await.unwrap();
        assert_eq!(res, "list_test");
    }
}
