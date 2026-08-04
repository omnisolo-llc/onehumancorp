#[cfg(test)]
mod tests {
    use crate::domain::omnichannel::models::{Inbox, Contact, Conversation, Message, Channel};
    use crate::domain::omnichannel::api::IngestWebhookRequest;
    use uuid::Uuid;

    #[test]
    fn test_ingest_webhook_request_struct() {
        let req = IngestWebhookRequest {
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            contact_name: "John Doe".to_string(),
            contact_identifier: "john@example.com".to_string(),
            content: "Hello".to_string(),
            sender_type: "user".to_string(),
        };
        assert_eq!(req.contact_name, "John Doe");
    }

    #[test]
    fn test_models_structs() {
        let id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let inbox = Inbox {
            id,
            tenant_id,
            name: "Test".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        assert_eq!(inbox.name, "Test");

        let contact = Contact {
            id,
            tenant_id,
            name: "Jane Doe".to_string(),
            identifier: "jane@example.com".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        assert_eq!(contact.identifier, "jane@example.com");

        let conv = Conversation {
            id,
            tenant_id,
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            status: "open".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        assert_eq!(conv.status, "open");

        let msg = Message {
            id,
            tenant_id,
            conversation_id: Uuid::new_v4(),
            content: "Hello".to_string(),
            sender_type: "user".to_string(),
            status: "delivered".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        assert_eq!(msg.content, "Hello");

        let channel = Channel {
            id,
            tenant_id,
            inbox_id: Uuid::new_v4(),
            provider_type: "web".to_string(),
            credentials: sqlx::types::Json(serde_json::json!({})),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        assert_eq!(channel.provider_type, "web");
    }
}
