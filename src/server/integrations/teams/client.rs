use reqwest::Client;

pub struct TeamsClient {
    pub access_token: String,
    http_client: Client,
}

impl TeamsClient {
    pub fn new(access_token: String) -> Self {
        TeamsClient {
            access_token,
            http_client: Client::new(),
        }
    }

    pub async fn create_meeting(&self, subject: &str) -> Result<String, String> {
        let url = "https://graph.microsoft.com/v1.0/me/onlineMeetings".to_string();
        let payload = serde_json::json!({
            "startDateTime": "2025-01-01T10:00:00Z",
            "endDateTime": "2025-01-01T11:00:00Z",
            "subject": subject
        });

        let res = self.http_client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(resp.text().await.unwrap_or_default())
                } else {
                    Err(format!("Teams API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
