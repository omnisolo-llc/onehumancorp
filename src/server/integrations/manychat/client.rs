pub struct ManychatClient {
    api_key: String,
}

impl ManychatClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn get_subscriber_info(&self, subscriber_id: &str) -> Result<String, String> {
        Ok(format!("Subscriber info for {} (mocked)", subscriber_id))
    }
}
