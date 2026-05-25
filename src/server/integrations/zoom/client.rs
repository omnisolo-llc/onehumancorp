use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait ZoomClientWrapper: Send + Sync {
    async fn create_meeting(&self, topic: &str) -> Result<String, String>;
    async fn get_oauth_url(&self, redirect_uri: &str) -> String;
    async fn exchange_token(&self, code: &str, redirect_uri: &str) -> Result<String, String>;
}

pub struct ZoomClient {
    pub api_key: String,
    http_client: Client,
}

impl ZoomClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl ZoomClientWrapper for ZoomClient {
    async fn create_meeting(&self, topic: &str) -> Result<String, String> {
        let payload = serde_json::json!({
            "topic": topic,
            "type": 2, // Scheduled meeting
            "duration": 60
        });

        let res = self.http_client.post("https://api.zoom.us/v2/users/me/meetings")
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                let json: serde_json::Value = resp.json().await.map_err(|_| "Invalid JSON")?;
                Ok(json["join_url"].as_str().unwrap_or("").to_string())
            },
            _ => Err("Failed to create meeting".to_string())
        }
    }

    async fn get_oauth_url(&self, redirect_uri: &str) -> String {
        let client_id = std::env::var("ZOOM_CLIENT_ID").unwrap_or_else(|_| "".to_string());
        format!("https://zoom.us/oauth/authorize?response_type=code&client_id={}&redirect_uri={}", client_id, redirect_uri)
    }

    async fn exchange_token(&self, _code: &str, _redirect_uri: &str) -> Result<String, String> {
        Ok("mock_zoom_token".to_string())
    }
}
