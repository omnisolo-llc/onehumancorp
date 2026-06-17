use super::client::{WhatsAppClientWrapper, RealWhatsAppClient};
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct WhatsAppProvider {
    client: Arc<dyn WhatsAppClientWrapper>,
    metadata: ProviderMetadata,
    pub phone_number_id: String,
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
                category: "social".to_string(),
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
        self.client.send_message(&self.phone_number_id, to, body).await
    }
}
