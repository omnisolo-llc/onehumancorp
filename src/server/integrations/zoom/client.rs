pub struct ZoomClient {
    pub api_key: String,
}

impl ZoomClient {
    pub fn new(api_key: String) -> Self {
        ZoomClient { api_key }
    }

    pub async fn create_meeting(&self, _topic: &str) -> Result<String, String> {
        Ok("https://zoom.us/j/mock_meeting_123".to_string())
    }
}
