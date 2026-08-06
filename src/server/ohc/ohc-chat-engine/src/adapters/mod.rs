pub mod instagram;
pub mod whatsapp;
pub mod web_widget;

use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ChannelAdapter {
    async fn handle_webhook(&self, payload: Value) -> Result<(), String>;
    async fn send_message(&self, to: &str, content: &str) -> Result<(), String>;
}
