use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait JitsiClientWrapper: Send + Sync {
    async fn generate_meeting_link(&self, room_name: &str) -> Result<String, String>;
}

pub struct RealJitsiClient {
    base_url: String,
    _http_client: Client,
}

impl RealJitsiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            _http_client: Client::new(),
        }
    }
}

#[async_trait]
impl JitsiClientWrapper for RealJitsiClient {
    async fn generate_meeting_link(&self, room_name: &str) -> Result<String, String> {
        Ok(format!("{}/{}", self.base_url, room_name))
    }
}
