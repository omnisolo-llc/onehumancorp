#[cfg(test)]
mod tests {
    use crate::models::IncomingMessage;

    #[test]
    fn test_models_serialize_deserialize() {
        let message = IncomingMessage {
            tenant_id: "tenant-1".to_string(),
            source_channel: "instagram".to_string(),
            sender_id: "user123".to_string(),
            message_content: "Hello".to_string(),
            timestamp: 1234567890,
        };

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: IncomingMessage = serde_json::from_str(&serialized).unwrap();

        assert_eq!(message.tenant_id, deserialized.tenant_id);
        assert_eq!(message.message_content, deserialized.message_content);
    }
}
