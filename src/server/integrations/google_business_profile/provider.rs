use super::client::{GoogleBusinessProfileClientWrapper, RealGoogleBusinessProfileClient};
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct GoogleBusinessProfileProvider {
    client: Arc<dyn GoogleBusinessProfileClientWrapper>,
    metadata: ProviderMetadata,
}

impl GoogleBusinessProfileProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealGoogleBusinessProfileClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "google_business_profile".to_string(),
                name: "Google Business Profile".to_string(),
                category: "messaging_and_reviews".to_string(),
                base_url: "https://businessmessages.googleapis.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn GoogleBusinessProfileClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "google_business_profile".to_string(),
                name: "Google Business Profile".to_string(),
                category: "messaging_and_reviews".to_string(),
                base_url: "https://businessmessages.googleapis.com".to_string(),
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

    pub async fn send_message(&self, conversation_id: &str, text: &str) -> Result<String, String> {
        self.client.send_message(conversation_id, text).await
    }

    pub async fn reply_to_review(&self, account_id: &str, location_id: &str, review_id: &str, reply: &str) -> Result<String, String> {
        self.client.reply_to_review(account_id, location_id, review_id, reply).await
    }
}
