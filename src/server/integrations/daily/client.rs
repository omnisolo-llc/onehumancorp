use reqwest::Client;

pub struct DailyClient {
    pub api_key: String,
    http_client: Client,
}

impl DailyClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn create_meeting(&self, _topic: &str) -> Result<String, String> {
        let url = "https://api.daily.co/v1/rooms";
        let payload = serde_json::json!({
            "properties": {
                "exp": chrono::Utc::now().timestamp() + 3600 // 1 hour from now
            }
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
                    if let Some(url) = body.get("url").and_then(|u| u.as_str()) {
                        Ok(url.to_string())
                    } else {
                        Err("Failed to parse Daily.co response".to_string())
                    }
                } else {
                    Err(format!("Daily.co API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
