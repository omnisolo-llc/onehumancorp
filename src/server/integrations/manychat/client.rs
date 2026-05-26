use reqwest::Client;

pub struct ManychatClient {
    pub api_key: String,
    http_client: Client,
}

impl ManychatClient {
    pub fn new(api_key: String) -> Self {
        ManychatClient {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn fetch_conversations(&self) -> Result<Vec<String>, String> {
        let url = "https://api.manychat.com/fb/page/getConversations";

        let res = self.http_client.get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(vec!["Test Conversation 1".to_string()])
                } else {
                    Err(format!("Manychat API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
