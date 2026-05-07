use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait EasyPostClientWrapper: Send + Sync {
    async fn create_shipment(&self, to_address: &str, from_address: &str, parcel: &str) -> Result<String, String>;
}

pub struct RealEasyPostClient {
    api_key: String,
    http_client: Client,
}

impl RealEasyPostClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl EasyPostClientWrapper for RealEasyPostClient {
    async fn create_shipment(&self, _to_address: &str, _from_address: &str, _parcel: &str) -> Result<String, String> {
        if self.api_key.is_empty() {
            return Err("API key is required".to_string());
        }
        // Mock EasyPost shipment creation
        Ok("shp_123456789".to_string())
    }
}
