use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait AyrshareClientWrapper: Send + Sync {
    async fn post_message(&self, post: &str, platforms: Vec<String>, organization_id: &str) -> Result<String, String>;
    async fn get_history(&self, organization_id: &str) -> Result<Vec<SocialMessage>, String>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SocialMessage {
    pub id: String,
    pub platform: String,
    pub text: String,
    pub user: String,
    pub created_at: String,
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
    async fn post_message(&self, post: &str, platforms: Vec<String>, organization_id: &str) -> Result<String, String> {
        let url = "https://api.ayrshare.com/api/post";
        let res = self.http_client.post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({
                "post": post,
                "platforms": platforms,
            }))
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                let _ = ::server_telemetry::record_api_call_cost(
                    &crate::db::get_pool(),
                    organization_id,
                    "ayrshare_post",
                    0.05
                ).await;
                Ok("Success".to_string())
            }
            Ok(resp) => Err(format!("Ayrshare API error: {}", resp.status())),
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    async fn get_history(&self, organization_id: &str) -> Result<Vec<SocialMessage>, String> {
        let url = "https://api.ayrshare.com/api/history";
        let res = self.http_client.get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                let _ = ::server_telemetry::record_api_call_cost(
                    &crate::db::get_pool(),
                    organization_id,
                    "ayrshare_get_history",
                    0.02
                ).await;
                let messages: Vec<SocialMessage> = resp.json().await.map_err(|e| e.to_string())?;
                Ok(messages)
            }
            Ok(resp) => Err(format!("Ayrshare API error: {}", resp.status())),
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ayrshare_creation() {
        let _client = RealAyrshareClient::new("test_key".to_string());
    }
}
