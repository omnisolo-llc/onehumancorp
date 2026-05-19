use super::client::{MetaClientWrapper, RealMetaClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct MetaProvider {
    client: Arc<dyn MetaClientWrapper>,
    metadata: ProviderMetadata,
}

impl MetaProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealMetaClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "meta".to_string(),
                name: "Meta Graph API (Facebook, Instagram, WhatsApp)".to_string(),
                category: "social".to_string(),
                base_url: "https://graph.facebook.com/v19.0".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn MetaClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "meta".to_string(),
                name: "Meta Graph API (Facebook, Instagram, WhatsApp)".to_string(),
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

    pub async fn send_message(&self, platform: &str, to: &str, body: &str) -> Result<(), String> {
        self.client.send_message(platform, to, body).await
    }
}
