use super::data::Message;
use async_trait::async_trait;

#[async_trait]
pub trait ChannelAdapter: Send + Sync {
    async fn send_message(&self, message: &Message) -> Result<(), String>;
    fn translates_channel(&self) -> &str;
}
