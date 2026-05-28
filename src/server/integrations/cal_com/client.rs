use reqwest::Client;

pub struct CalComClient {
    pub access_token: String,
    http_client: Client,
}

impl CalComClient {
    pub fn new(access_token: String) -> Self {
        CalComClient {
            access_token,
            http_client: Client::new(),
        }
    }
}

impl CalComClient {
    pub async fn get_booking_link(&self, event_type: &str) -> Result<String, String> {
        let url = format!("https://api.cal.com/v1/event-types");

        let res = self.http_client.get(&url)
            .query(&[("apiKey", &self.access_token)])
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json_resp: serde_json::Value = resp.json().await.map_err(|e| format!("JSON parsing error: {}", e))?;
                    if let Some(slug) = json_resp.get("event_types").and_then(|arr| arr.as_array()).and_then(|arr| arr.first()).and_then(|e| e.get("slug")).and_then(|s| s.as_str()) {
                         Ok(format!("https://cal.com/ohc-tenant/{}", slug))
                    } else {
                         Ok(format!("https://cal.com/ohc-tenant/{}", event_type)) // fallback
                    }
                } else {
                    Err(format!("Cal.com API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
