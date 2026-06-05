use reqwest::Client;

pub struct ConvertKitClient {
    pub api_secret: String,
    http_client: Client,
}

impl ConvertKitClient {
    pub fn new(api_secret: String) -> Self {
        ConvertKitClient {
            api_secret,
            http_client: Client::new(),
        }
    }

    pub async fn add_subscriber(&self, form_id: &str, email: &str, first_name: &str) -> Result<String, String> {
        let url = format!("https://api.convertkit.com/v3/forms/{}/subscribe", form_id);
        let payload = serde_json::json!({
            "api_secret": self.api_secret,
            "email": email,
            "first_name": first_name
        });

        let res = self.http_client.post(&url)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(resp.text().await.unwrap_or_default())
                } else {
                    Err(format!("ConvertKit API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
