use uuid::Uuid;
use super::models::ChatMessage;

pub trait ChannelAdapter {
    fn ingest_webhook(&self, payload: &serde_json::Value) -> Result<ChatMessage, String>;
}

pub struct MockChannelAdapter;

impl ChannelAdapter for MockChannelAdapter {
    fn ingest_webhook(&self, payload: &serde_json::Value) -> Result<ChatMessage, String> {
        let tenant_id = payload.get("tenant_id").and_then(|v| v.as_str()).and_then(|s| Uuid::parse_str(s).ok())
            .ok_or("Missing or invalid tenant_id")?;
        let content = payload.get("message").and_then(|v| v.as_str())
            .ok_or("Missing message content")?.to_string();

        Ok(ChatMessage {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id: Uuid::new_v4(), // Mock conversation id for now
            sender_type: "contact".to_string(),
            sender_id: None,
            content,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use uuid::Uuid;

    #[test]
    fn test_mock_channel_adapter_success() {
        let adapter = MockChannelAdapter;
        let tenant_id = Uuid::new_v4();
        let payload = json!({
            "tenant_id": tenant_id.to_string(),
            "source": "whatsapp",
            "sender_id": "15551234567",
            "message": "Hello world"
        });

        let result = adapter.ingest_webhook(&payload);
        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message.tenant_id, tenant_id);
        assert_eq!(message.content, "Hello world");
        assert_eq!(message.sender_type, "contact");
    }

    #[test]
    fn test_mock_channel_adapter_missing_tenant() {
        let adapter = MockChannelAdapter;
        let payload = json!({
            "message": "Hello world"
        });

        let result = adapter.ingest_webhook(&payload);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Missing or invalid tenant_id");
    }
}
