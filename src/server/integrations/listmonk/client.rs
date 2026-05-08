use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

#[async_trait]
pub trait ListmonkClientWrapper: Send + Sync {
    async fn send_campaign(&self, campaign_id: i32) -> Result<(), String>;
}

pub struct RealListmonkClient {
    base_url: String,
    username: String,
    password: Option<String>,
    http_client: Client,
}

impl RealListmonkClient {
    pub fn new(base_url: String, username: String, password: Option<String>) -> Self {
        Self {
            base_url,
            username,
            password,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl ListmonkClientWrapper for RealListmonkClient {
    async fn send_campaign(&self, campaign_id: i32) -> Result<(), String> {
        let url = format!("{}/api/campaigns/{}/status", self.base_url, campaign_id);
        let mut req = self.http_client.put(&url).json(&json!({"status": "running"}));

        if let Some(pwd) = &self.password {
            req = req.basic_auth(&self.username, Some(pwd));
        } else {
            req = req.basic_auth(&self.username, Some("")); // handle empty password
        }

        let res = req.send().await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Listmonk API error: {}", resp.status()))
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
        let client = RealListmonkClient::new("http://localhost:9000".to_string(), "admin".to_string(), Some("pass".to_string()));
        assert_eq!(client.username, "admin");
    }

    #[tokio::test]
    async fn test_send_campaign_error_handling() {
        let client = RealListmonkClient::new("http://localhost:9000".to_string(), "admin".to_string(), Some("pass".to_string()));
        let _ = client.send_campaign(1).await;
    }
}
