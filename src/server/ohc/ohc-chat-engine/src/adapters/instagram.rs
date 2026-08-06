use async_trait::async_trait;
use serde_json::Value;

use super::ChannelAdapter;

pub struct InstagramAdapter {
    pub access_token: String,
    pub page_id: String,
}

#[async_trait]
impl ChannelAdapter for InstagramAdapter {
    async fn handle_webhook(&self, _payload: Value) -> Result<(), String> {
        Ok(())
    }

    async fn send_message(&self, _to: &str, _content: &str) -> Result<(), String> {
        Ok(())
    }
}
