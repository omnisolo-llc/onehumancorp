use super::client::WhatsAppClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct WhatsAppProvider {
    client: Arc<WhatsAppClient>,
    metadata: ProviderMetadata,
}

impl WhatsAppProvider {
    pub fn new(phone_number_id: String, access_token: String) -> Self {
        let client = WhatsAppClient::new(access_token, phone_number_id);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "whatsapp".to_string(),
                name: "WhatsApp".to_string(),
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

    pub async fn send_message(&self, to: &str, body: &str) -> Result<String, String> {
        self.client.send_message(to, body).await
    }
}
