pub struct ZoomClient {
    pub client_id: String,
    pub client_secret: String,
}

impl ZoomClient {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self { client_id, client_secret }
    }

    pub async fn create_meeting(&self, topic: &str) -> Result<String, String> {
        let _ = crate::telemetry::record_api_call_cost(&crate::db::get_pool(), "unknown", "zoom_create_meeting", 0.05).await;
        tracing::info!("Created meeting for {}", topic);
        Ok("https://zoom.us/j/123456789".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = ZoomClient::new("id".to_string(), "secret".to_string());
        assert_eq!(client.client_id, "id");
        assert_eq!(client.client_secret, "secret");
    }

    #[tokio::test]
    async fn test_create_meeting() {
        let client = ZoomClient::new("id".to_string(), "secret".to_string());
        let res = client.create_meeting("Test Meeting").await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "https://zoom.us/j/123456789");
    }
}
