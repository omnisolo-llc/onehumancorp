pub struct ZoomClient {
    api_key: String,
    api_secret: String,
}

impl ZoomClient {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self { api_key, api_secret }
    }

    pub async fn create_meeting(&self) -> Result<String, String> {
        Ok("Mock meeting url".to_string())
    }
}
