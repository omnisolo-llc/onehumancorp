use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ChatConnector: Send + Sync {
    /// Returns the type of channel this connector handles (e.g. "whatsapp", "web_widget")
    fn channel_type(&self) -> &str;

    /// Sends a message to the specified contact using the channel configuration
    async fn send_message(&self, config: &Option<Value>, to: &str, content: &str) -> Result<(), String>;
}

pub struct WebWidgetConnector;

#[async_trait]
impl ChatConnector for WebWidgetConnector {
    fn channel_type(&self) -> &str {
        "web_widget"
    }

    async fn send_message(&self, _config: &Option<Value>, _to: &str, _content: &str) -> Result<(), String> {
        // In a real web widget scenario, this might push to a redis pub/sub that is consumed
        // by the websocket server connected to the user.
        // For now, we simulate success.
        Ok(())
    }
}

pub struct WhatsAppConnector;

#[async_trait]
impl ChatConnector for WhatsAppConnector {
    fn channel_type(&self) -> &str {
        "whatsapp"
    }

    async fn send_message(&self, _config: &Option<Value>, _to: &str, _content: &str) -> Result<(), String> {
        // Implementation would use the whatsapp cloud api connector to send the message.
        // For example, reading token/phone_id from config.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_web_widget_connector() {
        let connector = WebWidgetConnector;
        assert_eq!(connector.channel_type(), "web_widget");
        let result = connector.send_message(&None, "user1", "hello").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_whatsapp_connector() {
        let connector = WhatsAppConnector;
        assert_eq!(connector.channel_type(), "whatsapp");
        let result = connector.send_message(&None, "user1", "hello").await;
        assert!(result.is_ok());
    }
}
