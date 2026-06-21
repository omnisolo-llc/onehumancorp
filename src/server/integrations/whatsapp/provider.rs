use super::client::{WhatsAppClientWrapper, RealWhatsAppClient};
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct WhatsAppProvider {
    client: Arc<dyn WhatsAppClientWrapper>,
    metadata: ProviderMetadata,
    phone_number_id: String,
}

impl WhatsAppProvider {
    pub fn new(access_token: String, phone_number_id: String) -> Self {
        let client = RealWhatsAppClient::new(access_token);

        Self {
            client: Arc::new(client),
            phone_number_id,
            metadata: ProviderMetadata {
                id: "whatsapp".to_string(),
                name: "WhatsApp Cloud API".to_string(),
                category: "messaging".to_string(),
                base_url: "https://graph.facebook.com/v19.0".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn WhatsAppClientWrapper>, phone_number_id: String) -> Self {
        Self {
            client,
            phone_number_id,
            metadata: ProviderMetadata {
                id: "whatsapp".to_string(),
                name: "WhatsApp Cloud API".to_string(),
                category: "messaging".to_string(),
                base_url: "https://graph.facebook.com/v19.0".to_string(),
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

    pub async fn send_message(&self, to: &str, body: &str) -> Result<(), String> {
        self.client.send_message(&self.phone_number_id, to, body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockWhatsAppClient;

    #[async_trait]
    impl WhatsAppClientWrapper for MockWhatsAppClient {
        async fn send_message(&self, _phone_number_id: &str, _to: &str, _body: &str) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_whatsapp_provider_new() {
        let provider = WhatsAppProvider::new("test_token".to_string(), "12345".to_string());
        assert_eq!(provider.metadata.id, "whatsapp");
        assert_eq!(provider.metadata.category, "messaging");
        assert_eq!(provider.phone_number_id, "12345");
    }

    #[test]
    fn test_whatsapp_provider_with_client() {
        let mock_client = Arc::new(MockWhatsAppClient);
        let provider = WhatsAppProvider::with_client(mock_client, "12345".to_string());
        assert_eq!(provider.metadata.id, "whatsapp");
        assert_eq!(provider.metadata.category, "messaging");
    }

    #[test]
    fn test_whatsapp_provider_to_integration_provider() {
        let provider = WhatsAppProvider::new("test_token".to_string(), "12345".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "whatsapp");
    }

    #[tokio::test]
    async fn test_whatsapp_provider_send_message() {
        let mock_client = Arc::new(MockWhatsAppClient);
        let provider = WhatsAppProvider::with_client(mock_client, "12345".to_string());
        let result = provider.send_message("user", "hello").await;
        assert!(result.is_ok());
    }
}
