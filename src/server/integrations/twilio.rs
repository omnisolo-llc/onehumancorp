pub mod provider {
    use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

    pub struct TwilioProvider {}

    impl TwilioProvider {
        pub fn new() -> Self {
            Self {}
        }

        pub fn into_integration_provider(self) -> IntegrationProvider {
            IntegrationProvider {
                metadata: ProviderMetadata {
                    id: "twilio".to_string(),
                    name: "Twilio".to_string(),
                    category: "SMS & Notifications".to_string(),
                    base_url: "https://twilio.com".to_string(),
                },
            }
        }

        pub async fn send_sms(&self, _to: &str, _message: &str) -> Result<String, String> {
            Ok("message_id_123".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::provider::*;

    #[test]
    fn test_twilio_provider_metadata() {
        let provider = TwilioProvider::new().into_integration_provider();
        assert_eq!(provider.metadata.id, "twilio");
        assert_eq!(provider.metadata.name, "Twilio");
        assert_eq!(provider.metadata.category, "SMS & Notifications");
        assert_eq!(provider.metadata.base_url, "https://twilio.com");
    }

    #[tokio::test]
    async fn test_twilio_send_sms() {
        let provider = TwilioProvider::new();
        let id = provider.send_sms("+1234567890", "test").await.unwrap();
        assert_eq!(id, "message_id_123");
    }
}
