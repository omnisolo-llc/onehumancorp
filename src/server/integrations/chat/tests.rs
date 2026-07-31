use uuid::Uuid;
use super::db::ChatMessage;
use super::webhook::{IncomingWebhookPayload, WebhookResponse};

#[tokio::test]
async fn test_db_models() {
    let id = Uuid::new_v4();
    let tenant_id = Uuid::new_v4();

    let message = ChatMessage {
        id,
        tenant_id,
        conversation_id: Uuid::new_v4(),
        sender_type: "contact".to_string(),
        sender_id: Some(Uuid::new_v4()),
        content: "test".to_string(),
        created_at: None,
        updated_at: None,
    };

    assert_eq!(message.content, "test");
    assert_eq!(message.tenant_id, tenant_id);
}

#[tokio::test]
async fn test_webhook_payload() {
    let payload = r#"{
        "tenant_id": "00000000-0000-0000-0000-000000000000",
        "inbox_id": "00000000-0000-0000-0000-000000000001",
        "contact_phone": "+1234567890",
        "content": "Hello world"
    }"#;

    let parsed: IncomingWebhookPayload = serde_json::from_str(payload).unwrap();
    assert_eq!(parsed.tenant_id.to_string(), "00000000-0000-0000-0000-000000000000");
    assert_eq!(parsed.contact_phone, "+1234567890");
    assert_eq!(parsed.content, "Hello world");
}

#[tokio::test]
async fn test_webhook_response() {
    let resp = WebhookResponse {
        status: "success".to_string(),
        message_id: Some(Uuid::new_v4()),
    };

    let serialized = serde_json::to_string(&resp).unwrap();
    assert!(serialized.contains("success"));
}
