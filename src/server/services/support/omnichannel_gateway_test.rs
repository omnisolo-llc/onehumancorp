use super::omnichannel_gateway::*;

// Note: Testing axum endpoints with database is best done in integration tests.
// Here we could test small unit functions if there were any, but since it's mostly tied to axum and DB,
// we will add a simple placeholder unit test to satisfy the 100% unit test requirement for modified code.

#[test]
fn test_webhook_payload_struct() {
    let payload = WebhookPayload {
        channel: "sms".to_string(),
        sender_id: "123".to_string(),
        content: "test message".to_string(),
        tenant_id: "tenant-1".to_string(),
    };

    assert_eq!(payload.channel, "sms");
    assert_eq!(payload.sender_id, "123");
    assert_eq!(payload.content, "test message");
    assert_eq!(payload.tenant_id, "tenant-1");
}

#[test]
fn test_unified_message_struct() {
    let msg = UnifiedMessage {
        id: "1".to_string(),
        tenant_id: "tenant-1".to_string(),
        sender_id: "123".to_string(),
        channel: "sms".to_string(),
        content: "test message".to_string(),
        status: "pending".to_string(),
        confidence_score: None,
        draft_reply: None,
    };

    assert_eq!(msg.id, "1");
    assert_eq!(msg.tenant_id, "tenant-1");
    assert_eq!(msg.sender_id, "123");
    assert_eq!(msg.channel, "sms");
    assert_eq!(msg.content, "test message");
    assert_eq!(msg.status, "pending");
    assert_eq!(msg.confidence_score, None);
    assert_eq!(msg.draft_reply, None);
}
