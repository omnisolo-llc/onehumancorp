use super::models::{ConversationStatus, MessageDirection};
use super::websocket::WsState;
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::broadcast;
use chrono::Utc;

#[tokio::test]
async fn test_ws_state_channel_creation() {
    let state = WsState::new();
    let tenant_id = Uuid::new_v4();
    let channel = state.get_or_create_channel(tenant_id);
    assert_eq!(state.tenant_channels.len(), 1);
}

#[tokio::test]
async fn test_ws_state_broadcasting() {
    let state = WsState::new();
    let tenant_id = Uuid::new_v4();

    // Create channel
    let tx = state.get_or_create_channel(tenant_id);
    let mut rx = tx.subscribe();

    let msg = super::models::Message {
        id: Uuid::new_v4(),
        conversation_id: Uuid::new_v4(),
        sender_id: None,
        content: "Test".to_string(),
        direction: MessageDirection::Inbound,
        is_read: false,
        created_at: Utc::now(),
    };

    state.broadcast_message(tenant_id, msg.clone());

    let received = rx.recv().await.unwrap();
    assert_eq!(received.content, "Test");
}
