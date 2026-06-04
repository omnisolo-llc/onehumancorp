use reqwest::Client;

pub struct ChatwootClient {
    pub api_key: String,
    pub base_url: String,
    http_client: Client,
}

impl ChatwootClient {
    pub fn new(api_key: String, base_url: String) -> Self {
        ChatwootClient {
            api_key,
            base_url,
            http_client: Client::new(),
        }
    }

    pub async fn send_message(&self, account_id: &str, conversation_id: &str, content: &str) -> Result<(), String> {
        let url = format!("{}/api/v1/accounts/{}/conversations/{}/messages", self.base_url, account_id, conversation_id);
        let payload = serde_json::json!({
            "content": content,
            "message_type": "outgoing"
        });

        let res = self.http_client.post(&url)
            .header("api_access_token", &self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Chatwoot API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
