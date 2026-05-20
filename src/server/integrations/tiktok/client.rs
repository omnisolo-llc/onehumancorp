use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait TikTokClientWrapper: Send + Sync {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String>;
}

pub struct RealTikTokClient {
    access_token: String,
    http_client: Client,
}

impl RealTikTokClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl TikTokClientWrapper for RealTikTokClient {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String> {
        // Stub implementation, would integrate with real TikTok Business API
        Ok(())
    }
}
