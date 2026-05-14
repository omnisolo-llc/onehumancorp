use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait ShippoClientWrapper: Send + Sync {
    async fn create_label(&self, order_id: &str, address_to: &str) -> Result<String, String>;
}

pub struct RealShippoClient {
    pub api_key: String,
}

impl RealShippoClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl ShippoClientWrapper for RealShippoClient {
    async fn create_label(&self, order_id: &str, address_to: &str) -> Result<String, String> {
        // Mock Shippo label generation
        tracing::info!("Generating Shippo label for order {} to {}", order_id, address_to);
        Ok(format!("shippo_mock_tracking_{}", order_id))
    }
}
