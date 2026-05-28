use reqwest::Client;

pub struct AyrshareClient {
    pub api_key: String,
    http_client: Client,
}

impl AyrshareClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

impl AyrshareClient {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ayrshare_client_new() {
        let client = AyrshareClient::new("test_token".to_string());
        assert_eq!(client.api_key, "test_token");
    }

    // Usually HTTP calls are mocked in unit tests, or skipped unless e2e.
    // For unit testing here, we can test initialization and methods signature.
}
