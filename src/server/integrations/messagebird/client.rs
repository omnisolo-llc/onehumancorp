pub struct MessageBirdClient {
    _api_key: String,
}

impl MessageBirdClient {
    pub fn new(api_key: String) -> Self {
        Self { _api_key: api_key }
    }
}

impl MessageBirdClient {
    pub async fn mock_method(&self) -> Result<(), String> {
        Ok(())
    }
}
