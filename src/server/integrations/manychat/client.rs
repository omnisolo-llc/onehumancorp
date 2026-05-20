pub struct ManychatClient {
    pub access_token: String,
}

impl ManychatClient {
    pub fn new(access_token: String) -> Self {
        ManychatClient { access_token }
    }

    pub async fn send_message(&self, _platform: &str, _to: &str, _body: &str) -> Result<(), String> {
        // Mock implementation for Manychat send message
        Ok(())
    }
}
