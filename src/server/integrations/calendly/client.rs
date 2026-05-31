use reqwest::Client;

pub struct CalendlyClient {
    pub api_key: String,
    http_client: Client,
}

impl CalendlyClient {
    pub fn new(api_key: String) -> Self {
        CalendlyClient {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn fetch_event_types(&self) -> Result<Vec<String>, String> {
        let url = "https://api.calendly.com/users/me/event_types"; // Replace with user's URI in a real implementation

        let res = self.http_client.get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(vec!["30-min Consultation".to_string()]) // simplify response parsing
                } else {
                    Err(format!("Calendly API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
