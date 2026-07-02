use reqwest::Client;

pub struct MessageBirdClient {
    pub api_key: String,
    http_client: Client,
}

impl MessageBirdClient {
    pub fn new(api_key: String) -> Self {
        MessageBirdClient {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn send_sms(&self, originator: &str, recipients: &str, body: &str) -> Result<String, String> {
        let url = "https://rest.messagebird.com/messages".to_string();
        let payload = serde_json::json!({
            "originator": originator,
            "recipients": recipients,
            "body": body
        });

        let res = self.http_client.post(&url)
            .header("Authorization", format!("AccessKey {}", self.api_key))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(resp.text().await.unwrap_or_default())
                } else {
                    Err(format!("MessageBird API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
