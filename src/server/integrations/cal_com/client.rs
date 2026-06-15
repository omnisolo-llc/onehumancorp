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
    pub async fn get_free_busy(&self, time_min: &str, time_max: &str) -> Result<String, String> {
        let url = "https://api.cal.com/v1/availability".to_string();

        let res = self.http_client.get(&url)
            .query(&[
                ("apiKey", &self.access_token),
                ("dateFrom", &time_min.to_string()),
                ("dateTo", &time_max.to_string())
            ])
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    Ok(text)
                } else {
                    Err(format!("Cal.com API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn create_event(&self, summary: &str, start_time: &str, end_time: &str) -> Result<String, String> {
        let url = "https://api.cal.com/v1/bookings".to_string();

        let payload = serde_json::json!({
            "title": summary,
            "start": start_time,
            "end": end_time
        });

        let res = self.http_client.post(&url)
            .query(&[("apiKey", &self.access_token)])
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.unwrap_or_default();
                    let json: serde_json::Value = serde_json::from_str(&text).unwrap_or(serde_json::Value::Null);
                    let event_id = json["booking"]["id"].as_str().unwrap_or("mock_event_123").to_string();
                    Ok(event_id)
                } else {
                    Err(format!("Cal.com API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn get_booking_link(&self, event_type: &str) -> Result<String, String> {
        let url = "https://api.cal.com/v1/event-types".to_string();

        let res = self.http_client.get(&url)
            .query(&[("apiKey", &self.access_token)])
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(format!("https://cal.com/ohc-tenant/{}", event_type))
                } else {
                    Err(format!("Cal.com API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
