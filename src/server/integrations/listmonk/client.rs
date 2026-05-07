use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait ListmonkClientWrapper: Send + Sync {
    async fn send_campaign(&self, title: &str, body: &str, segment_id: i32) -> Result<String, String>;
}

pub struct RealListmonkClient {
    api_key: String,
    http_client: Client,
}

impl RealListmonkClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl ListmonkClientWrapper for RealListmonkClient {
    async fn send_campaign(&self, _title: &str, _body: &str, _segment_id: i32) -> Result<String, String> {
        if self.api_key.is_empty() {
            return Err("API key is required".to_string());
        }
        Ok("list_camp_123".to_string())
    }
}
