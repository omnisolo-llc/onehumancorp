use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

#[async_trait]
pub trait AyrshareClientWrapper: Send + Sync {
    async fn post_message(&self, post: &str, platforms: Vec<&str>) -> Result<Value, String>;
    async fn get_messages(&self) -> Result<Value, String>;
    async fn reply_message(&self, message_id: &str, reply: &str) -> Result<Value, String>;
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
    async fn post_message(&self, post: &str, platforms: Vec<&str>) -> Result<Value, String> {
        let url = "https://app.ayrshare.com/api/post";
        let payload = serde_json::json!({
            "post": post,
            "platforms": platforms,
        });

        let res = self.http_client.post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json = resp.json().await.map_err(|e| format!("Failed to parse JSON: {}", e))?;
                    Ok(json)
                } else {
                    Err(format!("Ayrshare API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    async fn get_messages(&self) -> Result<Value, String> {
        let url = "https://app.ayrshare.com/api/comments"; // simplified endpoint
        let res = self.http_client.get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json = resp.json().await.map_err(|e| format!("Failed to parse JSON: {}", e))?;
                    Ok(json)
                } else {
                    Err(format!("Ayrshare API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    async fn reply_message(&self, message_id: &str, reply: &str) -> Result<Value, String> {
        let url = "https://app.ayrshare.com/api/comments/reply"; // simplified endpoint
        let payload = serde_json::json!({
            "messageId": message_id,
            "reply": reply,
        });

        let res = self.http_client.post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json = resp.json().await.map_err(|e| format!("Failed to parse JSON: {}", e))?;
                    Ok(json)
                } else {
                    Err(format!("Ayrshare API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
