pub struct ZoomClient {
    pub api_key: String,
}

impl ZoomClient {
    pub fn new(api_key: String) -> Self {
        ZoomClient { api_key }
    }

    pub async fn create_meeting(&self, topic: &str, start_time: &str, duration_minutes: i32) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "zoom_create_meeting",
            0.05
        ).await;
        Ok("https://zoom.us/j/mock-meeting-id".to_string())
    }
}
