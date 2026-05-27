pub struct ManychatClient {
    pub api_key: String,
}

impl ManychatClient {
    pub fn new(api_key: String) -> Self {
        ManychatClient { api_key }
    }

    pub async fn fetch_conversations(&self) -> Result<Vec<String>, String> {
        Ok(vec!["Test Conversation 1".to_string()])
    }

    pub async fn send_message(&self, _platform: &str, _to: &str, _body: &str) -> Result<(), String> {
        Ok(())
    }

    pub async fn handle_webhook(&self, _payload: &str) -> Result<(), String> {
        Ok(())
    }
}
