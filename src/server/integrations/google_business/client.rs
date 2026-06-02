use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait GoogleBusinessClientWrapper: Send + Sync {
    async fn sync_menu(&self, _menu_data: &str) -> Result<String, String>;
    async fn sync_hours(&self, _hours_data: &str) -> Result<String, String>;
}

pub struct RealGoogleBusinessClient {
    _access_token: String,
    _http_client: Client,
}

impl RealGoogleBusinessClient {
    pub fn new(access_token: String) -> Self {
        Self {
            _access_token: access_token,
            _http_client: Client::new(),
        }
    }
}

#[async_trait]
impl GoogleBusinessClientWrapper for RealGoogleBusinessClient {
    async fn sync_menu(&self, _menu_data: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "google_business_sync_menu",
            0.01
        ).await;
        Ok("ok".to_string())
    }

    async fn sync_hours(&self, _hours_data: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "google_business_sync_hours",
            0.01
        ).await;
        Ok("ok".to_string())
    }
}
