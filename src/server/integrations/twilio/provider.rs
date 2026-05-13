use super::client::{TwilioClientWrapper, RealTwilioClient, SmsStatus};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;
use std::collections::HashMap;

pub struct TwilioProvider {
    client: Arc<dyn TwilioClientWrapper>,
    metadata: ProviderMetadata,
}

impl TwilioProvider {
    pub fn new(account_sid: String, auth_token: String) -> Self {
        let client = RealTwilioClient::new(account_sid, auth_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "twilio".to_string(),
                name: "Twilio SMS".to_string(),
                category: "sms".to_string(),
                base_url: "https://api.twilio.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn TwilioClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "twilio".to_string(),
                name: "Twilio SMS".to_string(),
                category: "sms".to_string(),
                base_url: "https://api.twilio.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn send_sms(&self, to: &str, from: &str, body: &str, tenant_id: &str) -> Result<SmsStatus, String> {
        self.client.send_sms(to, from, body, tenant_id).await
    }

    pub async fn send_order_notification(&self, to: &str, from: &str, order_id: &str, customer_name: &str, amount: f64, tenant_id: &str) -> Result<SmsStatus, String> {
        let mut placeholders = HashMap::new();
        placeholders.insert("order_id".to_string(), order_id.to_string());
        placeholders.insert("customer_name".to_string(), customer_name.to_string());
        placeholders.insert("amount".to_string(), amount.to_string());

        self.client.send_templated_sms(to, from, "order_confirmation", placeholders, tenant_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockTwilioClient {
        sent_messages: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl TwilioClientWrapper for MockTwilioClient {
        async fn send_sms(&self, _to: &str, _from: &str, _body: &str, _tenant_id: &str) -> Result<SmsStatus, String> {
            self.sent_messages.fetch_add(1, Ordering::SeqCst);
            Ok(SmsStatus { sid: "mock".to_string(), status: "sent".to_string() })
        }
        async fn send_templated_sms(&self, _to: &str, _from: &str, _template_id: &str, _placeholders: std::collections::HashMap<String, String>, _tenant_id: &str) -> Result<SmsStatus, String> {
            self.sent_messages.fetch_add(1, Ordering::SeqCst);
             Ok(SmsStatus { sid: "mock".to_string(), status: "sent".to_string() })
        }
    }

    #[tokio::test]
    async fn test_twilio_provider_integration() {
        let sent = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockTwilioClient { sent_messages: sent.clone() });
        let provider = TwilioProvider::with_client(mock);

        provider.send_sms("+1234567890", "+0987654321", "Test message", "tenant1").await.unwrap();
        assert_eq!(sent.load(Ordering::SeqCst), 1);
    }
}
