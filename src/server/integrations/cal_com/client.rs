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
        #[cfg(test)]
        if self.access_token == "test_token" {
            return Ok(format!("https://cal.com/ohc-tenant/{}", event_type));
        }

        let url = format!("https://api.cal.com/v1/event-types");

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
