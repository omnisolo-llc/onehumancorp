use super::client::{RealWhatsAppCloudClient, WhatsAppCloudClientWrapper};
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct WhatsAppCloudProvider {
    client: Arc<dyn WhatsAppCloudClientWrapper>,
    metadata: ProviderMetadata,
}

impl WhatsAppCloudProvider {
    pub fn new(phone_number_id: String, access_token: String) -> Self {
        let client = RealWhatsAppCloudClient::new(phone_number_id, access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "whatsapp_cloud_api".to_string(),
                name: "WhatsApp Cloud API".to_string(),
                category: "social".to_string(),
                base_url: "https://graph.facebook.com/v19.0".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn WhatsAppCloudClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "whatsapp_cloud_api".to_string(),
                name: "WhatsApp Cloud API".to_string(),
                category: "social".to_string(),
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
        self.client.send_message(to, body).await
    }

    pub async fn send_template(&self, to: &str, template_name: &str, language_code: &str) -> Result<(), String> {
        self.client.send_template(to, template_name, language_code).await
    }

    pub async fn send_media(&self, to: &str, media_type: &str, media_id_or_url: &str, caption: Option<&str>) -> Result<(), String> {
        self.client.send_media(to, media_type, media_id_or_url, caption).await
    }

    pub async fn send_interactive_buttons(&self, to: &str, body_text: &str, buttons: Vec<(&str, &str)>) -> Result<(), String> {
        self.client.send_interactive_buttons(to, body_text, buttons).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockWhatsAppCloudClient;

    #[async_trait]
    impl WhatsAppCloudClientWrapper for MockWhatsAppCloudClient {
        async fn send_message(&self, _to: &str, _body: &str) -> Result<(), String> {
            Ok(())
        }
        async fn send_template(&self, _to: &str, _template_name: &str, _language_code: &str) -> Result<(), String> {
            Ok(())
        }
        async fn send_media(&self, _to: &str, _media_type: &str, _media_id_or_url: &str, _caption: Option<&str>) -> Result<(), String> {
            Ok(())
        }
        async fn send_interactive_buttons(&self, _to: &str, _body_text: &str, _buttons: Vec<(&str, &str)>) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_whatsapp_cloud_provider_new() {
        let provider = WhatsAppCloudProvider::new("phone_id".to_string(), "test_token".to_string());
        assert_eq!(provider.metadata.id, "whatsapp_cloud_api");
        assert_eq!(provider.metadata.category, "social");
    }

    #[test]
    fn test_whatsapp_cloud_provider_with_client() {
        let mock_client = Arc::new(MockWhatsAppCloudClient);
        let provider = WhatsAppCloudProvider::with_client(mock_client);
        assert_eq!(provider.metadata.id, "whatsapp_cloud_api");
        assert_eq!(provider.metadata.category, "social");
    }

    #[test]
    fn test_whatsapp_cloud_provider_to_integration_provider() {
        let provider = WhatsAppCloudProvider::new("phone_id".to_string(), "test_token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "whatsapp_cloud_api");
    }

    #[tokio::test]
    async fn test_whatsapp_cloud_provider_send_message() {
        let mock_client = Arc::new(MockWhatsAppCloudClient);
        let provider = WhatsAppCloudProvider::with_client(mock_client);
        let result = provider.send_message("user", "hello").await;
        assert!(result.is_ok());
    }
}
