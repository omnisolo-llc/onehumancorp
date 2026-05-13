use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait CalComClientWrapper: Send + Sync {
    async fn generate_booking_link(&self, user_email: &str, event_type: &str) -> Result<String, String>;
}

pub struct RealCalComClient {
    api_key: String,
    base_url: String,
    http_client: Client,
}

impl RealCalComClient {
    pub fn new(api_key: String, base_url: String) -> Self {
        Self {
            api_key,
            base_url,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl CalComClientWrapper for RealCalComClient {
    async fn generate_booking_link(&self, user_email: &str, event_type: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "calcom_generate_link",
            0.05
        ).await;

        Ok(format!("{}/booking/mock_link?email={}&type={}", self.base_url, user_email, event_type))
    }
}
