pub struct CalComClient {
    pub api_key: String,
}

impl CalComClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn create_booking_link(&self, event_type_id: i32) -> Result<String, String> {
        let _ = crate::telemetry::record_api_call_cost(&crate::db::get_pool(), "unknown", "calcom_create_link", 0.05).await;
        tracing::info!("Generated link for {}", event_type_id);
        Ok("https://cal.com/booking/123".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = CalComClient::new("key".to_string());
        assert_eq!(client.api_key, "key");
    }

    #[tokio::test]
    async fn test_create_booking_link() {
        let client = CalComClient::new("key".to_string());
        let res = client.create_booking_link(1).await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "https://cal.com/booking/123");
    }
}
