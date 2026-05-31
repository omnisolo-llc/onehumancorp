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
        let url = "https://api.manychat.com/fb/subscriber/search"; // simplified for fetch

        let res = self.http_client.get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(vec!["Test Conversation 1".to_string()]) // simplify response parsing
                } else {
                    Err(format!("Manychat API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn send_message(&self, to: &str, body: &str) -> Result<(), String> {
        let url = "https://api.manychat.com/fb/sending/sendContent";

        let payload = serde_json::json!({
            "subscriber_id": to,
            "data": {
                "version": "v2",
                "content": {
                    "messages": [
                        {
                            "type": "text",
                            "text": body
                        }
                    ]
                }
            },
            "message_tag": "NON_PROMOTIONAL_SUBSCRIPTION"
        });

        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Manychat API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
