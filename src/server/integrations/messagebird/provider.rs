use super::client::{MessagebirdClientWrapper, RealMessagebirdClient};
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct MessagebirdProvider {
    client: Arc<dyn MessagebirdClientWrapper>,
    metadata: ProviderMetadata,
}

impl MessagebirdProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealMessagebirdClient::new(api_key);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "messagebird".to_string(),
                name: "MessageBird".to_string(),
                category: "sms".to_string(),
                base_url: "https://rest.messagebird.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn MessagebirdClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "messagebird".to_string(),
                name: "MessageBird".to_string(),
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

    pub async fn send_sms(&self, to: &str, from: &str, body: &str) -> Result<(), String> {
        self.client.send_sms(to, from, body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockMessagebirdClient {
        sent_messages: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl MessagebirdClientWrapper for MockMessagebirdClient {
        async fn send_sms(&self, _to: &str, _from: &str, _body: &str) -> Result<(), String> {
            self.sent_messages.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_messagebird_provider_integration() {
        let sent = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockMessagebirdClient { sent_messages: sent.clone() });
        let provider = MessagebirdProvider::with_client(mock);

        provider.send_sms("+1234567890", "OHC", "Test message").await.unwrap();
        assert_eq!(sent.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_messagebird_provider_new() {
        let provider = MessagebirdProvider::new("test_token".to_string());
        assert_eq!(provider.metadata.id, "messagebird");
        assert_eq!(provider.metadata.category, "sms");
    }

    #[test]
    fn test_messagebird_provider_into() {
        let provider = MessagebirdProvider::new("test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "messagebird");
    }
}

impl MessagebirdProvider {
    pub async fn fetch_conversations(&self) -> Result<Vec<String>, String> {
        Ok(vec![])
    }
}
