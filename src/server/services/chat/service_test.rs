#[cfg(test)]
mod tests {
    use super::super::service::ChatService;
    use super::super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};
    use uuid::Uuid;
    use chrono::Utc;

    // We can't connect to postgres locally in Bazel without a lot of setup, but we can verify
    // that the structures serialize and deserialize correctly to emulate database row mapping behavior
    // and verify that the traits are correctly implemented.

    #[test]
    fn test_models_serialization() {
        let tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();

        let inbox = ChatInbox {
            id: inbox_id,
            tenant_id,
            name: "Main Inbox".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&inbox).unwrap();
        assert!(json.contains("Main Inbox"));

        let deserialized: ChatInbox = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, inbox_id);

        let channel = ChatChannel {
            id: Uuid::new_v4(),
            tenant_id,
            inbox_id,
            channel_type: "whatsapp".to_string(),
            config: serde_json::json!({"api_key": "test"}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json_channel = serde_json::to_string(&channel).unwrap();
        assert!(json_channel.contains("whatsapp"));
    }
}
