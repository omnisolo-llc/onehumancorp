use reqwest::Client;

pub struct DailyClient {
    pub api_key: String,
    http_client: Client,
}

impl DailyClient {
    pub fn new(api_key: String) -> Self {
        DailyClient {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn create_meeting(&self, topic: &str) -> Result<String, String> {
        let url = "https://api.daily.co/v1/rooms";
        // Daily.co requires alphanumeric names and hyphens only
        let sanitized_topic = topic
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>();

        let payload = serde_json::json!({
            "name": sanitized_topic,
            "privacy": "public"
        });

        let res = self
            .http_client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    let json: serde_json::Value =
                        serde_json::from_str(&text).unwrap_or(serde_json::Value::Null);
                    let join_url = json["url"]
                        .as_str()
                        .unwrap_or("https://domain.daily.co/mock_meeting_123")
                        .to_string();
                    Ok(join_url)
                } else {
                    Err(format!("Daily API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
