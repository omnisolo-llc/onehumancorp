pub struct DailycoClient {
    pub api_key: String,
}

impl DailycoClient {
    pub fn new(api_key: String) -> Self {
        DailycoClient { api_key }
    }

    pub async fn create_meeting(&self, _topic: &str) -> Result<String, String> {
        Ok("https://mock.daily.co/mock_meeting_123".to_string())
    }
}
