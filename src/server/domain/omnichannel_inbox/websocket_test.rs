use super::websocket::{InboxEventBroadcaster, InboxEvent};
use super::data::Message;
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_inbox_event_broadcaster() {
    let broadcaster = InboxEventBroadcaster::new();
    let mut rx = broadcaster.subscribe();

    let msg = Message {
        id: Uuid::new_v4(),
        conversation_id: Uuid::new_v4(),
        sender_id: None,
        content: "test message".to_string(),
        direction: "inbound".to_string(),
        created_at: Utc::now(),
    };

    broadcaster.broadcast(InboxEvent::MessageReceived(msg.clone())).unwrap();

    let event = rx.recv().await.unwrap();
    match event {
        InboxEvent::MessageReceived(received_msg) => {
            assert_eq!(received_msg.id, msg.id);
        },
        _ => panic!("Expected MessageReceived event"),
    }
}
