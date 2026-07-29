use super::engine::ChatEngine;
use super::models::ChatMessage;

#[test]
fn test_chat_engine_intent_classification() {
    let engine = ChatEngine::new(None);
    let msg = ChatMessage {
        id: "1".to_string(),
        tenant_id: "t1".to_string(),
        conversation_id: "c1".to_string(),
        sender_id: "cust1".to_string(),
        sender_type: "customer".to_string(),
        content: "What is the price of this item?".to_string(),
        created_at: 1000,
    };

    let res = engine.handle_incoming_message(msg).unwrap();
    assert!(res.is_some());
    // Auto-responder responds with generic message since there's no price match
    assert_eq!(res.unwrap().content, "Thank you for your message. I'm an AI assistant. How can I help?");
}

#[test]
fn test_chat_engine_human_handoff() {
    let engine = ChatEngine::new(None);
    let msg = ChatMessage {
        id: "2".to_string(),
        tenant_id: "t1".to_string(),
        conversation_id: "c2".to_string(),
        sender_id: "cust2".to_string(),
        sender_type: "customer".to_string(),
        content: "I want to talk to a human manager right now".to_string(),
        created_at: 1000,
    };

    let res = engine.handle_incoming_message(msg).unwrap();
    assert!(res.is_some());
    assert_eq!(res.unwrap().content, "Transferring you to a human agent...");
}

#[test]
fn test_chat_engine_copilot_drafting() {
    let engine = ChatEngine::new(None);
    let msg = ChatMessage {
        id: "3".to_string(),
        tenant_id: "t1".to_string(),
        conversation_id: "c3".to_string(),
        sender_id: "cust3".to_string(),
        sender_type: "customer".to_string(),
        content: "My order is delayed.".to_string(),
        created_at: 1000,
    };

    engine.handle_incoming_message(msg).unwrap();

    let draft = engine.draft_copilot_response("c3").unwrap();
    assert!(draft.contains("Drafting response for: 'My order is delayed.'"));
}
