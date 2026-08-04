use super::*;
use crate::models::{WebhookPayload, ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};

#[test]
fn test_models() {
    let payload = WebhookPayload {
        channel: "whatsapp".to_string(),
        sender_id: "+123456789".to_string(),
        content: "hello world".to_string(),
    };
    assert_eq!(payload.channel, "whatsapp");
    assert_eq!(payload.content, "hello world");
}
