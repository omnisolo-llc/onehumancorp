use async_trait::async_trait;

#[async_trait]
pub trait ShipEngineClientWrapper: Send + Sync {
    async fn generate_label(&self, address: &str) -> Result<String, String>;
}

pub struct RealShipEngineClient {
    pub api_key: String,
}

impl RealShipEngineClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl ShipEngineClientWrapper for RealShipEngineClient {
    async fn generate_label(&self, _address: &str) -> Result<String, String> {
        // Mock generate label
        Ok("mock_label_url".to_string())
    }
}
