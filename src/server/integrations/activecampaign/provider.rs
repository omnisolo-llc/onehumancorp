use super::client::ActiveCampaignClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ActiveCampaignProvider {
    _client: Arc<ActiveCampaignClient>,
    metadata: ProviderMetadata,
}

impl ActiveCampaignProvider {
    pub fn new(api_key: String) -> Self {
        let client = ActiveCampaignClient::new(api_key);

        Self {
            _client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "activecampaign".to_string(),
                name: "ActiveCampaign".to_string(),
                category: "email_marketing".to_string(),
                base_url: "https://youraccount.api-us1.com/api/3".to_string(),
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
