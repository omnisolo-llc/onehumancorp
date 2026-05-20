use super::client::{OutlookCalendarClientWrapper, RealOutlookCalendarClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct OutlookCalendarProvider {
    client: Arc<dyn OutlookCalendarClientWrapper>,
    metadata: ProviderMetadata,
}

impl OutlookCalendarProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealOutlookCalendarClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "outlook".to_string(),
                name: "Microsoft Outlook Calendar".to_string(),
                category: "calendar".to_string(),
                base_url: "https://graph.microsoft.com/v1.0".to_string(),
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
