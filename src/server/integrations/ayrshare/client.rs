use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait AyrshareClientWrapper: Send + Sync {
    async fn post_message(&self, post: &str, platforms: Vec<&str>) -> Result<(), String>;
    async fn get_messages(&self) -> Result<Vec<Message>, String>;
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

#[derive(Serialize)]
struct PostRequest<'a> {
    post: &'a str,
    platforms: Vec<&'a str>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    pub id: String,
    pub post: String,
    pub platform: String,
}

#[async_trait]
impl AyrshareClientWrapper for RealAyrshareClient {
    async fn post_message(&self, post: &str, platforms: Vec<&str>) -> Result<(), String> {
        let url = "https://app.ayrshare.com/api/post";
        let req = PostRequest { post, platforms };

        let res = self.http_client.post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&req)
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

    async fn get_messages(&self) -> Result<Vec<Message>, String> {
        let url = "https://app.ayrshare.com/api/history";

        let res = self.http_client.get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let text = resp.text().await.map_err(|e| format!("Failed to read response text: {}", e))?;
                    if text.is_empty() { return Ok(vec![]); }
                    let parsed: Vec<Message> = serde_json::from_str(&text).map_err(|e| format!("Failed to parse response: {}", e))?;
                    Ok(parsed)
                } else {
                    Err(format!("Ayrshare API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
