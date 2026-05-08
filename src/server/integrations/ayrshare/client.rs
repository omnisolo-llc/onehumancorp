use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

#[async_trait]
pub trait AyrshareClientWrapper: Send + Sync {
    async fn post_message(&self, post: &str, platforms: Vec<&str>) -> Result<(), String>;
}

pub struct RealAyrshareClient {
    api_key: String,
    http_client: Client,
}

impl RealAyrshareClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl AyrshareClientWrapper for RealAyrshareClient {
    async fn post_message(&self, post: &str, platforms: Vec<&str>) -> Result<(), String> {
        let url = "https://app.ayrshare.com/api/post";
        let res = self.http_client.post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "post": post,
                "platforms": platforms,
            }))
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
    fn test_real_client_creation() {
        let client = RealAyrshareClient::new("api_key".to_string());
        assert_eq!(client.api_key, "api_key");
    }

    #[tokio::test]
    async fn test_post_message_error_handling() {
        let client = RealAyrshareClient::new("api_key".to_string());
        let _ = client.post_message("test", vec!["twitter"]).await;
    }
}
