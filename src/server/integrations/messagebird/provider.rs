use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use super::client::{MessageBirdClientWrapper, RealMessageBirdClient};
use std::sync::Arc;

pub struct MessageBirdProvider {
    client: Arc<dyn MessageBirdClientWrapper>,
    metadata: ProviderMetadata,
}

impl MessageBirdProvider {
    pub fn new(access_key: String) -> Self {
        let client = RealMessageBirdClient::new(access_key);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "messagebird".to_string(),
                name: "MessageBird SMS".to_string(),
                category: "sms".to_string(),
                base_url: "https://rest.messagebird.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn MessageBirdClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "messagebird".to_string(),
                name: "MessageBird SMS".to_string(),
                category: "sms".to_string(),
                base_url: "https://rest.messagebird.com".to_string(),
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

    pub async fn send_sms(&self, to: &str, body: &str) -> Result<(), String> {
        self.client.send_sms(to, body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockMessageBirdClient;

    #[async_trait]
    impl MessageBirdClientWrapper for MockMessageBirdClient {
        async fn send_sms(&self, _to: &str, _body: &str) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_messagebird_provider_metadata() {
        let provider = MessageBirdProvider::new("key".to_string());
        assert_eq!(provider.metadata.id, "messagebird");
    }

    #[tokio::test]
    async fn test_messagebird_send_sms() {
        let provider = MessageBirdProvider::with_client(Arc::new(MockMessageBirdClient));
        assert!(provider.send_sms("123", "Hello").await.is_ok());
    }
}
