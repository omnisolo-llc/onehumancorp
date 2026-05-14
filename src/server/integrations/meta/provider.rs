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
                name: "Meta (Instagram & Facebook)".to_string(),
                category: "social_media".to_string(),
                base_url: "https://graph.facebook.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn MetaClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "meta".to_string(),
                name: "Meta (Instagram & Facebook)".to_string(),
                category: "social_media".to_string(),
                base_url: "https://graph.facebook.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn send_message(&self, recipient_id: &str, text: &str) -> Result<(), String> {
        self.client.send_message(recipient_id, text).await
    }
}
