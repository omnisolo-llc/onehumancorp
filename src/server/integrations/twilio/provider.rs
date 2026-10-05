use super::client::{MessageReceipt, MessageSendError, RealTwilioClient, TwilioClientWrapper};
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

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

    pub async fn send_sms(
        &self,
        to: &str,
        from: &str,
        body: &str,
    ) -> Result<MessageReceipt, MessageSendError> {
        // Mock checking opt-out status
        if self.is_opted_out(to).await {
            return Err(MessageSendError::OptedOut);
        }
        let receipt = self.client.send_sms(to, from, body).await?;
        tracing::info!(message_sid = %receipt.sid, "Twilio accepted SMS");
        Ok(receipt)
    }

    pub async fn send_whatsapp(
        &self,
        to: &str,
        from: &str,
        body: &str,
    ) -> Result<MessageReceipt, MessageSendError> {
        // Mock checking opt-out status
        if self.is_opted_out(to).await {
            return Err(MessageSendError::OptedOut);
        }
        let receipt = self.client.send_whatsapp(to, from, body).await?;
        tracing::info!(message_sid = %receipt.sid, "Twilio accepted WhatsApp message");
        Ok(receipt)
    }

    pub async fn provision_number(&self, area_code: &str) -> Result<String, String> {
        self.client.provision_number(area_code).await
    }
}

impl TwilioProvider {
    pub async fn is_opted_out(&self, _phone: &str) -> bool {
        // In a real app, query the DB for user communication preferences
        false
    }

    pub async fn handle_opt_out(&self, _phone: &str) -> Result<(), String> {
        // Handle STOP messages by updating DB
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockTwilioClient {
        sent_messages: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl TwilioClientWrapper for MockTwilioClient {
        async fn send_sms(
            &self,
            _to: &str,
            _from: &str,
            _body: &str,
        ) -> Result<MessageReceipt, MessageSendError> {
            self.sent_messages.fetch_add(1, Ordering::SeqCst);
            Ok(MessageReceipt {
                sid: "SM22222222222222222222222222222222".into(),
            })
        }

        async fn send_whatsapp(
            &self,
            _to: &str,
            _from: &str,
            _body: &str,
        ) -> Result<MessageReceipt, MessageSendError> {
            self.sent_messages.fetch_add(1, Ordering::SeqCst);
            Ok(MessageReceipt {
                sid: "SM22222222222222222222222222222222".into(),
            })
        }

        async fn provision_number(&self, _area_code: &str) -> Result<String, String> {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let last_four: u32 = rng.gen_range(1000..9999);
            Ok(format!("+1555123{}", last_four))
        }
    }

    #[tokio::test]
    async fn test_twilio_provider_integration() {
        let sent = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockTwilioClient {
            sent_messages: sent.clone(),
        });
        let provider = TwilioProvider::with_client(mock);

        let receipt = provider
            .send_sms("+1234567890", "+0987654321", "Test message")
            .await
            .unwrap();
        assert_eq!(receipt.sid, "SM22222222222222222222222222222222");
        assert_eq!(sent.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_twilio_provider_new() {
        let provider = TwilioProvider::new("sid".to_string(), "token".to_string());
        assert_eq!(provider.metadata.id, "twilio");
        assert_eq!(provider.metadata.category, "sms");
    }

    #[test]
    fn test_twilio_provider_into() {
        let provider = TwilioProvider::new("sid".to_string(), "token".to_string());
        let integration = provider.into_integration_provider();
        assert_eq!(integration.metadata.id, "twilio");
    }
}
