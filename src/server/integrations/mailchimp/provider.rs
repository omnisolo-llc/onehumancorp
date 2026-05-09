use std::sync::Arc;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::MailchimpClient;

pub struct MailchimpProvider {
    client: Arc<MailchimpClient>,
}

impl MailchimpProvider {
    pub fn new(api_key: String, server_prefix: String) -> Self {
        Self {
            client: Arc::new(MailchimpClient::new(api_key, server_prefix)),
        }
    }

    pub fn get_metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            id: "mailchimp".to_string(),
            name: "Mailchimp".to_string(),
            category: "email_marketing".to_string(),
            base_url: "https://{server_prefix}.api.mailchimp.com".to_string(),
        }
    }
}
