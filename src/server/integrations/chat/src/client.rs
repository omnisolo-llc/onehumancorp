use async_trait::async_trait;

#[async_trait]
pub trait ChatClientWrapper: Send + Sync {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String>;
}

pub struct RealChatClient {
    pub endpoint: String,
    pub auth_token: String,
}

impl RealChatClient {
    pub fn new(endpoint: String, auth_token: String) -> Self {
        Self {
            endpoint,
            auth_token,
        }
    }
}

#[async_trait]
impl ChatClientWrapper for RealChatClient {
    async fn send_message(&self, _to: &str, _body: &str) -> Result<(), String> {
        Ok(())
    }
}
