pub struct ZoomClient {
    pub api_key: String,
}

impl ZoomClient {
    pub fn new(api_key: String) -> Self {
        ZoomClient { api_key }
    }

    pub async fn create_meeting(&self, topic: &str, start_time: &str, duration_mins: i32) -> Result<String, String> {
        let topic_encoded = topic.replace(" ", "_");
        Ok(format!("https://zoom.us/j/mock_{}?topic={}&duration={}", start_time, topic_encoded, duration_mins).replace(" ", "_"))
    }
}
