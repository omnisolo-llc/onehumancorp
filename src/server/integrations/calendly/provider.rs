use std::sync::Arc;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::CalendlyClient;

pub struct CalendlyProvider {
    client: Arc<CalendlyClient>,
}

impl CalendlyProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Arc::new(CalendlyClient::new(api_key)),
        }
    }

    pub fn get_metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            id: "calendly".to_string(),
            name: "Calendly".to_string(),
            category: "calendar".to_string(),
            base_url: "https://api.calendly.com".to_string(),
        }
    }
}
