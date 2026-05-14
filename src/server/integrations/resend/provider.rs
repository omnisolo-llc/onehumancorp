use std::sync::Arc;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::{ResendClientWrapper, RealResendClient};

pub struct ResendProvider {
    pub client: Arc<dyn ResendClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl ResendProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealResendClient::new(api_key);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "resend".to_string(),
                name: "Resend Integration".to_string(),
                category: "email_marketing".to_string(),
                base_url: "https://api.resend.com".to_string(),
            },
        }
    }
}
