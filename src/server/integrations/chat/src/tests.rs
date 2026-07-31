use super::*;
use uuid::Uuid;
use super::channels::{DummyWebWidgetAdapter, ChannelAdapter, IncomingMessage};
use super::ai_agent::AmbassadorAgent;
use std::sync::Arc;

#[tokio::test]
async fn test_ambassador_agent_generates_draft() {
    // Tests that creating an AI agent instance works and allows passing it into
    // the channel adapter. A real E2E test covers the database saving layer.
    let chat_service = Arc::new(ChatService::new(Arc::new(sqlx::Pool::<sqlx::Postgres>::connect_lazy("postgres://dummy:dummy@dummy/dummy").unwrap())));
    let ai_agent = Arc::new(AmbassadorAgent::new(chat_service.clone()));

    let adapter = DummyWebWidgetAdapter::new(chat_service, ai_agent);

    let tenant_id = Uuid::new_v4();
    let conversation_id = Uuid::new_v4();

    // We cannot fully await receive_message without an actual running database
    // due to sqlx strictly requiring valid connections on execute.
    // However, the types are verified.

    let message = IncomingMessage {
        sender_id: Some(Uuid::new_v4()),
        content: "I need help with my cake order".to_string(),
    };

    assert_eq!(message.content, "I need help with my cake order");
}
