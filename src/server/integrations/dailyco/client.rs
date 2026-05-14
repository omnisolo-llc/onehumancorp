use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait DailyCoClientWrapper: Send + Sync {
    async fn create_room(&self, booking_id: &str) -> Result<String, String>;
}

pub struct RealDailyCoClient {
    pub api_key: String,
}

impl RealDailyCoClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl DailyCoClientWrapper for RealDailyCoClient {
    async fn create_room(&self, booking_id: &str) -> Result<String, String> {
        // Mock Daily.co room creation
        tracing::info!("Creating Daily.co room for booking {}", booking_id);
        Ok(format!("https://ohc-mock.daily.co/room_{}", booking_id))
    }
}
