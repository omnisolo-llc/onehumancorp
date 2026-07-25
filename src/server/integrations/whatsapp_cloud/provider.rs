use super::client::{RealWhatsAppCloudClient, WhatsAppCloudClientWrapper, Template, Interactive, Media};
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

    pub async fn send_template(&self, to: &str, template: Template) -> Result<(), String> {
        self.client.send_template(to, template).await
    }

    pub async fn send_interactive(&self, to: &str, interactive: Interactive) -> Result<(), String> {
        self.client.send_interactive(to, interactive).await
    }

    pub async fn send_media(&self, to: &str, media_type: &str, media: Media) -> Result<(), String> {
        self.client.send_media(to, media_type, media).await
    }

    pub async fn setup_webhook(&self, app_id: &str, app_secret: &str, webhook_url: &str, verify_token: &str) -> Result<(), String> {
        let access_token_url = format!(
            "https://graph.facebook.com/v19.0/oauth/access_token?client_id={}&client_secret={}&grant_type=client_credentials",
            app_id, app_secret
        );
        let client = reqwest::Client::new();

        let token_res = client.get(&access_token_url).send().await;
        let app_access_token = match token_res {
            Ok(res) if res.status().is_success() => {
                let json: serde_json::Value = res.json().await.unwrap_or_default();
                json["access_token"].as_str().unwrap_or_default().to_string()
            }
            Ok(res) => return Err(format!("Failed to get app access token: {}", res.text().await.unwrap_or_default())),
            Err(e) => return Err(format!("Reqwest error: {}", e)),
        };

        if app_access_token.is_empty() {
            return Err("Empty app access token received".to_string());
        }

        let subscribe_url = format!(
            "https://graph.facebook.com/v19.0/{}/subscriptions",
            app_id
        );

        let payload = serde_json::json!({
            "object": "whatsapp_business_account",
            "callback_url": webhook_url,
            "verify_token": verify_token,
            "fields": ["messages", "message_template_status_update"]
        });

        let res = client
            .post(&subscribe_url)
            .bearer_auth(&app_access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    let err_text = response.text().await.unwrap_or_default();
                    Err(format!("WhatsApp Webhook setup error: {}", err_text))
                }
            }
            Err(e) => Err(format!("Reqwest error: {}", e)),
        }
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
        async fn send_template(&self, _to: &str, _template: Template) -> Result<(), String> {
            Ok(())
        }
        async fn send_interactive(&self, _to: &str, _interactive: Interactive) -> Result<(), String> {
            Ok(())
        }
        async fn send_media(&self, _to: &str, _media_type: &str, _media: Media) -> Result<(), String> {
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
