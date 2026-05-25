use super::client::{TwilioClientWrapper, TwilioClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct TwilioProvider {
    _client: Arc<dyn TwilioClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl TwilioProvider {
    pub fn new(api_key: String) -> Self {
        let client = TwilioClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "twilio".to_string(),
                name: "Twilio".to_string(),
                category: "sms".to_string(),
                base_url: "https://api.twilio.com".to_string(),
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
        self._client.send_sms(to, body).await
    }

    pub async fn register_opt_in(&self, to: &str) -> Result<(), String> {
        self._client.register_opt_in(to).await
    }
}
