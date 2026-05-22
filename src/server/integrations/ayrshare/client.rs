use reqwest::Client;

pub struct AyrshareClient {
    api_key: String,
    #[allow(dead_code)]
    http_client: Client,
}

impl AyrshareClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn post_message(&self, message: &str, platforms: Vec<&str>) -> Result<(), String> {
        let url = "https://app.ayrshare.com/api/post";

        let payload = serde_json::json!({
            "post": message,
            "platforms": platforms
        });

        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Ayrshare API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
