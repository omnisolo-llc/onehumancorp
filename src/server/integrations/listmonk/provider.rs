use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::ListmonkClient;
use std::sync::Arc;

pub struct ListmonkProvider {
    metadata: ProviderMetadata,
    client: Option<Arc<ListmonkClient>>,
}

impl ListmonkProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "listmonk".to_string(),
                name: "Listmonk Email Marketing".to_string(),
                category: "email_marketing".to_string(),
                base_url: "http://localhost:9000/api".to_string(), // Default local listmonk
            },
            client: None,
        }
    }

    pub fn with_credentials(base_url: String, username: String, password: Option<String>) -> Self {
        let mut provider = Self::new();
        provider.client = Some(Arc::new(ListmonkClient::new(base_url, username, password)));
        provider
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn send_email_campaign(&self, list_ids: Vec<i32>, name: &str, subject: &str, body: &str) -> Result<i32, String> {
        if let Some(client) = &self.client {
            let campaign_id = client.create_campaign(list_ids, name, subject, body).await?;
            client.send_campaign(campaign_id).await?;
            Ok(campaign_id)
        } else {
            Err("Listmonk client not initialized".to_string())
        }
    }
}
