use std::sync::Arc;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::ZoomClient;

pub struct ZoomProvider {
    client: Arc<ZoomClient>,
}

impl ZoomProvider {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            client: Arc::new(ZoomClient::new(api_key, api_secret)),
        }
    }

    pub fn get_metadata(&self) -> ProviderMetadata {
        ProviderMetadata {
            id: "zoom".to_string(),
            name: "Zoom".to_string(),
            category: "video".to_string(),
            base_url: "https://api.zoom.us".to_string(),
        }
    }
}
