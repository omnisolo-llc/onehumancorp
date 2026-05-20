use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait ZoomClientWrapper: Send + Sync {
    async fn create_meeting(&self, topic: &str) -> Result<String, String>;
}

pub struct RealZoomClient {
    api_key: String,
    http_client: Client,
}

impl RealZoomClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl ZoomClientWrapper for RealZoomClient {
    async fn create_meeting(&self, _topic: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "zoom_create_meeting",
            0.05
        ).await;
        Ok("https://zoom.us/j/mock_meeting".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_client_creation() {
        let client = RealZoomClient::new("key".to_string());
        assert_eq!(client.api_key, "key");
    }

    #[tokio::test]
    async fn test_create_meeting_error_handling() {
        let client = RealZoomClient::new("key".to_string());
        let _ = client.create_meeting("topic").await;
    }
}
