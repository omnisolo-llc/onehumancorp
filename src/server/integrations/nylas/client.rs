use reqwest::Client;

pub struct NylasClient {
    pub access_token: String,
    http_client: Client,
}

impl NylasClient {
    pub fn new(access_token: String) -> Self {
        NylasClient {
            access_token,
            http_client: Client::new(),
        }
    }

    pub async fn get_calendars(&self) -> Result<String, String> {
        let url = "https://api.nylas.com/calendars".to_string();
        let res = self.http_client.get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(resp.text().await.unwrap_or_default())
                } else {
                    Err(format!("Nylas API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
