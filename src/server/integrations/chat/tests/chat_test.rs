use server_integrations_chat::{ChatService, WsEvent};
use tokio::sync::broadcast;
use uuid::Uuid;

#[tokio::test]
async fn test_create_message_broadcasts_event() {
    let (tx, mut rx) = broadcast::channel(100);
    let service = ChatService::new(tx);

    let tenant_id = Uuid::new_v4();
    let conversation_id = Uuid::new_v4();
    let content = "Hello, I need a cake".to_string();

    let result = service
        .create_message(tenant_id, conversation_id, content.clone(), "contact".to_string())
        .await;

    assert!(result.is_ok());

    let msg = result.unwrap();
    assert_eq!(msg.tenant_id, tenant_id);
    assert_eq!(msg.content, content);

    // Verify event was broadcast
    let event = rx.recv().await.expect("Failed to receive broadcast event");
    match event {
        WsEvent::MessageCreated { message } => {
            assert_eq!(message.id, msg.id);
            assert_eq!(message.content, content);
        }
        _ => panic!("Expected MessageCreated event"),
    }
}
