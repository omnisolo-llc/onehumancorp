use super::client::PaytmClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct PaytmProvider {
    _client: Arc<PaytmClient>,
    metadata: ProviderMetadata,
}

impl PaytmProvider {
    pub fn new(access_token: String) -> Self {
        let client = PaytmClient::new(access_token);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "paytm".to_string(),
                name: "Paytm".to_string(),
                category: "payment".to_string(),
                base_url: "https://securegw.paytm.in".to_string(),
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
}
