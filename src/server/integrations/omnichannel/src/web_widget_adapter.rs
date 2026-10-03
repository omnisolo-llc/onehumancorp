use crate::traits::ChannelAdapter;
use async_trait::async_trait;
use serde_json::Value;

pub struct WebWidgetAdapter;

#[async_trait]
impl ChannelAdapter for WebWidgetAdapter {
    async fn send_message(&self, _recipient_id: &str, _content: &str) -> Result<(), String> {
        // Web widget messages are delivered via the unified WebSocket manager,
        // so no provider-specific API call is needed here.
        Ok(())
    }

    async fn receive_webhook(&self, _payload: Value) -> Result<(), String> {
        // Web widgets typically use REST/WebSocket to ingest directly,
        // but can optionally support webhook normalization if required.
        Ok(())
    }
}
