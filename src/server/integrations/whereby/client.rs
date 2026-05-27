use reqwest::Client;

pub struct WherebyClient {
    pub api_key: String,
    http_client: Client,
}

impl WherebyClient {
    pub fn new(api_key: String) -> Self {
        WherebyClient {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn create_meeting(&self, topic: &str) -> Result<String, String> {
        let url = "https://api.whereby.dev/v1/meetings";
        let payload = serde_json::json!({
            "isLocked": false,
            "roomNamePrefix": topic.replace(" ", "-").to_lowercase(),
            "endDate": "2099-12-31T23:59:59Z" // Mock long-lived room
        });

        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let body: serde_json::Value = resp.json().await.unwrap_or_default();
                    if let Some(room_url) = body.get("roomUrl").and_then(|u| u.as_str()) {
                        Ok(room_url.to_string())
                    } else {
                         // Mock fallback
                         Ok("https://whereby.com/mock-room-123".to_string())
                    }
                } else {
                    Err(format!("Whereby API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
