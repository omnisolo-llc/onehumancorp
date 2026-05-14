use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait CalComClientWrapper: Send + Sync {
    async fn sync_calendar(&self, user_id: &str, calendar_data: &str) -> Result<(), String>;
}

pub struct RealCalComClient {
    pub api_key: String,
}

impl RealCalComClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl CalComClientWrapper for RealCalComClient {
    async fn sync_calendar(&self, user_id: &str, calendar_data: &str) -> Result<(), String> {
        // Mock Cal.com sync
        tracing::info!("Syncing calendar for user {} to Cal.com: {}", user_id, calendar_data);
        Ok(())
    }
}
