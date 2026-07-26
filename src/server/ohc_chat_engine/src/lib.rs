pub mod models;
pub mod services;

#[cfg(test)]
mod tests {
    use super::models::{Message};
    use super::services::chat_service::{ChatEngine, MockChatEngine};
    use super::services::ai_triage::AiTriageAgent;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_conversation() {
        let engine = MockChatEngine;
        let tenant_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let convo = engine.create_conversation(tenant_id, channel_id).await.unwrap();
        assert_eq!(convo.tenant_id, tenant_id);
    }

    #[tokio::test]
    async fn test_ai_triage() {
        let engine = MockChatEngine;
        let tenant_id = Uuid::new_v4();
        let convo_id = Uuid::new_v4();
        let msg = engine.receive_message(tenant_id, convo_id, "Do you have vegan cupcakes?".to_string()).await.unwrap();
        let suggested = AiTriageAgent::process_message(&msg).await;

        assert!(suggested.content.contains("vegan options"));
        assert!(suggested.action_payload.is_some());
    }
}
