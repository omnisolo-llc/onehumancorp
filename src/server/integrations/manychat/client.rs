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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manychat_client_new() {
        let client = ManychatClient::new("dummy_token".to_string());
        assert_eq!(client.api_key, "dummy_token");
    }

    #[tokio::test]
    async fn test_manychat_fetch_conversations() {
        let client = ManychatClient::new("dummy_token".to_string());
        let res = client.fetch_conversations().await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec!["Test Conversation 1".to_string()]);
    }
}
