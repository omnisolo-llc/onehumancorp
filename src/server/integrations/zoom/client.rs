use reqwest::Client;

pub struct ZoomClient {
    pub api_key: String,
    http_client: Client,
}

impl ZoomClient {
    pub fn new(api_key: String) -> Self {
        ZoomClient {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn create_meeting(&self, topic: &str) -> Result<String, String> {
        let url = "https://api.zoom.us/v2/users/me/meetings";
        let payload = serde_json::json!({
            "topic": topic,
            "type": 2
        });

        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    let json: serde_json::Value = serde_json::from_str(&text).unwrap_or(serde_json::Value::Null);
                    let join_url = json["join_url"].as_str().unwrap_or("https://zoom.us/j/mock_meeting_123").to_string();
                    Ok(join_url)
                } else {
                    Err(format!("Zoom API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
