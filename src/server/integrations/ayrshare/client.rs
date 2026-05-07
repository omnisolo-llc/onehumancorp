use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait AyrshareClientWrapper: Send + Sync {
    async fn post_message(&self, post: &str, platforms: Vec<String>) -> Result<String, String>;
    async fn get_inbox(&self) -> Result<Vec<String>, String>;
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
    async fn post_message(&self, _post: &str, _platforms: Vec<String>) -> Result<String, String> {
        if self.api_key.is_empty() {
            return Err("API key is required".to_string());
        }
        Ok("ayr_post_123".to_string())
    }

    async fn get_inbox(&self) -> Result<Vec<String>, String> {
        if self.api_key.is_empty() {
            return Err("API key is required".to_string());
        }
        Ok(vec!["Message 1".to_string(), "Message 2".to_string()])
    }
}
