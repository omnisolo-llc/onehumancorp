pub struct ZoomClient {
    pub client_id: String,
    pub client_secret: String,
}

impl ZoomClient {
    pub fn new(client_id: String, client_secret: String) -> Self {
        ZoomClient { client_id, client_secret }
    }

    pub async fn create_meeting(&self, tenant_id: &str) -> Result<String, String> {
        let _ = crate::telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "zoom_create_meeting",
            0.15
        ).await;
        Ok("https://zoom.us/j/mock_meeting_123".to_string())
    }
}
