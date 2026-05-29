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
}
