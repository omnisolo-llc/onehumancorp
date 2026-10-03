pub mod models;
pub mod repository;
pub mod router;
pub mod state_machine;
pub mod traits;

#[cfg(test)]
mod tests {
    use super::models::*;
    use super::repository::*;
    use super::router::*;
    use super::state_machine::*;
    use super::traits::*;
    use async_trait::async_trait;
    use sea_orm::entity::prelude::*;
    use serde_json::Value;

    struct MockChannelAdapter;

    #[async_trait]
    impl ChannelAdapter for MockChannelAdapter {
        async fn send_message(&self, _recipient_id: &str, _content: &str) -> Result<(), String> {
            Ok(())
        }
        async fn receive_webhook(&self, _payload: Value) -> Result<(), String> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_mock_channel_adapter() {
        let adapter = MockChannelAdapter;
        assert!(adapter.send_message("user_1", "hello").await.is_ok());
        assert!(adapter.receive_webhook(serde_json::json!({})).await.is_ok());
    }

    #[test]
    fn test_inbox_model_creation() {
        let id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let now = chrono::Utc::now().into();
        let model = inbox::Model {
            id,
            tenant_id,
            name: "WhatsApp".to_string(),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(model.id, id);
        assert_eq!(model.tenant_id, tenant_id);
        assert_eq!(model.name, "WhatsApp");
    }

    #[test]
    fn test_conversation_model_creation() {
        let id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();
        let now = chrono::Utc::now().into();
        let model = conversation::Model {
            id,
            tenant_id,
            inbox_id,
            contact_id,
            assignee_id: None,
            status: "open".to_string(),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(model.id, id);
        assert_eq!(model.tenant_id, tenant_id);
        assert_eq!(model.inbox_id, inbox_id);
        assert_eq!(model.status, "open");
    }

    #[test]
    fn test_message_model_creation() {
        let id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let conversation_id = Uuid::new_v4();
        let now = chrono::Utc::now().into();
        let model = message::Model {
            id,
            tenant_id,
            conversation_id,
            sender_type: "contact".to_string(),
            sender_id: None,
            content_type: "text".to_string(),
            content: "Hello there".to_string(),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(model.id, id);
        assert_eq!(model.tenant_id, tenant_id);
        assert_eq!(model.conversation_id, conversation_id);
        assert_eq!(model.content, "Hello there");
        assert_eq!(model.content_type, "text");
    }

    #[test]
    fn test_contact_model_creation() {
        let id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let now = chrono::Utc::now().into();
        let model = contact::Model {
            id,
            tenant_id,
            name: "Test User".to_string(),
            email: None,
            phone: None,
            channel_identity: "ig_123".to_string(),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(model.id, id);
        assert_eq!(model.tenant_id, tenant_id);
        assert_eq!(model.name, "Test User");
        assert_eq!(model.channel_identity, "ig_123");
    }

    #[tokio::test]
    async fn test_in_memory_repository() {
        let repo = InMemoryChatRepository::new();
        let tenant_id = Uuid::new_v4();
        let now = chrono::Utc::now().into();

        // Test contact creation and retrieval
        let contact = contact::Model {
            id: Uuid::new_v4(),
            tenant_id,
            name: "Test User".to_string(),
            email: None,
            phone: None,
            channel_identity: "ig_123".to_string(),
            created_at: now,
            updated_at: now,
        };
        repo.create_contact(contact.clone()).await.unwrap();

        let found = repo
            .find_contact_by_identity(tenant_id, "ig_123")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(found.id, contact.id);

        let not_found = repo
            .find_contact_by_identity(tenant_id, "unknown")
            .await
            .unwrap();
        assert!(not_found.is_none());

        // Test conversation creation and retrieval
        let conv = conversation::Model {
            id: Uuid::new_v4(),
            tenant_id,
            inbox_id: Uuid::new_v4(),
            contact_id: contact.id,
            assignee_id: None,
            status: "open".to_string(),
            created_at: now,
            updated_at: now,
        };
        repo.create_conversation(conv.clone()).await.unwrap();

        let found_conv = repo
            .find_active_conversation(tenant_id, contact.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(found_conv.id, conv.id);

        let not_found_conv = repo
            .find_active_conversation(tenant_id, Uuid::new_v4())
            .await
            .unwrap();
        assert!(not_found_conv.is_none());

        // Test message creation
        let msg = message::Model {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id: conv.id,
            sender_type: "contact".to_string(),
            sender_id: Some(contact.id),
            content_type: "text".to_string(),
            content: "Hello".to_string(),
            created_at: now,
            updated_at: now,
        };
        let created_msg = repo.create_message(msg.clone()).await.unwrap();
        assert_eq!(created_msg.id, msg.id);
    }

    #[tokio::test]
    async fn test_message_router_creates_contact_and_conversation() {
        let repo = InMemoryChatRepository::new();
        let router = MessageRouter::new(repo.clone());

        let tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();

        let msg = router
            .route_incoming_message(tenant_id, inbox_id, "ig_user_123", "Hello world")
            .await
            .unwrap();

        assert_eq!(msg.tenant_id, tenant_id);
        assert_eq!(msg.content, "Hello world");

        // Verify contact was created
        let contact = repo
            .find_contact_by_identity(tenant_id, "ig_user_123")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(contact.channel_identity, "ig_user_123");

        // Verify conversation was created
        let conv = repo
            .find_active_conversation(tenant_id, contact.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(conv.status, "open");
        assert_eq!(msg.conversation_id, conv.id);
    }

    #[tokio::test]
    async fn test_message_router_reuses_existing_conversation() {
        let repo = InMemoryChatRepository::new();
        let router = MessageRouter::new(repo.clone());

        let tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();

        // First message creates contact and conversation
        let msg1 = router
            .route_incoming_message(tenant_id, inbox_id, "ig_user_123", "Hello")
            .await
            .unwrap();

        // Second message from same identity should reuse them
        let msg2 = router
            .route_incoming_message(tenant_id, inbox_id, "ig_user_123", "How are you?")
            .await
            .unwrap();

        assert_eq!(msg1.conversation_id, msg2.conversation_id);
        assert_eq!(msg1.sender_id, msg2.sender_id);
    }

    #[test]
    fn test_conversation_state_machine_transitions() {
        let mut state = ConversationState::Open;

        // Open -> BotHandling
        state =
            ConversationStateMachine::transition(&state, &ConversationEvent::AssignToBot).unwrap();
        assert_eq!(state, ConversationState::BotHandling);

        // BotHandling -> HumanAssigned
        state = ConversationStateMachine::transition(&state, &ConversationEvent::AssignToHuman)
            .unwrap();
        assert_eq!(state, ConversationState::HumanAssigned);

        // HumanAssigned -> Resolved
        state = ConversationStateMachine::transition(&state, &ConversationEvent::Resolve).unwrap();
        assert_eq!(state, ConversationState::Resolved);

        // Resolved -> Open (incoming message)
        state = ConversationStateMachine::transition(&state, &ConversationEvent::IncomingMessage)
            .unwrap();
        assert_eq!(state, ConversationState::Open);

        // Open -> Snoozed
        state = ConversationStateMachine::transition(&state, &ConversationEvent::Snooze).unwrap();
        assert_eq!(state, ConversationState::Snoozed);

        // Snoozed -> Open (reopen)
        state = ConversationStateMachine::transition(&state, &ConversationEvent::Reopen).unwrap();
        assert_eq!(state, ConversationState::Open);

        // Snoozed -> Open (incoming message)
        state = ConversationStateMachine::transition(
            &ConversationState::Snoozed,
            &ConversationEvent::Snooze,
        )
        .unwrap();
        state = ConversationStateMachine::transition(&state, &ConversationEvent::IncomingMessage)
            .unwrap();
        assert_eq!(state, ConversationState::Open);

        // Invalid transitions
        assert!(ConversationStateMachine::transition(
            &ConversationState::Resolved,
            &ConversationEvent::AssignToBot
        )
        .is_err());
        assert!(ConversationStateMachine::transition(
            &ConversationState::Snoozed,
            &ConversationEvent::Resolve
        )
        .is_err());
    }

    #[test]
    fn test_conversation_state_parsing() {
        use std::str::FromStr;
        assert_eq!(
            ConversationState::from_str("open").unwrap(),
            ConversationState::Open
        );
        assert_eq!(
            ConversationState::from_str("Open").unwrap(),
            ConversationState::Open
        );
        assert_eq!(
            ConversationState::from_str("bot_handling").unwrap(),
            ConversationState::BotHandling
        );
        assert_eq!(
            ConversationState::from_str("bothandling").unwrap(),
            ConversationState::BotHandling
        );
        assert!(ConversationState::from_str("invalid").is_err());
    }

    #[test]
    fn test_conversation_state_display() {
        assert_eq!(ConversationState::Open.to_string(), "open");
        assert_eq!(ConversationState::BotHandling.to_string(), "bot_handling");
        assert_eq!(
            ConversationState::HumanAssigned.to_string(),
            "human_assigned"
        );
    }
}
pub mod web_widget_adapter;
pub mod whatsapp_adapter;
