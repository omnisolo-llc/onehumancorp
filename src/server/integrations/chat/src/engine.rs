use crate::service::ChatEngineService;
use crate::models::{MessageType};
use std::sync::Arc;
use uuid::Uuid;

pub struct OmnichannelEngine {
    service: Arc<dyn ChatEngineService>,
}

impl OmnichannelEngine {
    pub fn new(service: Arc<dyn ChatEngineService>) -> Self {
        Self { service }
    }

    pub async fn handle_incoming_message(&self, tenant_id: &str, contact_id: Uuid, content: &str) -> Result<(), String> {
        let conv = self.service.create_conversation(tenant_id, contact_id).await?;
        self.service.send_message(tenant_id, conv.id, content, MessageType::Incoming, Some(contact_id)).await?;
        let auto_response = format!("Auto-reply: We received your message '{}'. A human agent will be with you shortly.", content);
        self.service.auto_respond(tenant_id, conv.id, &auto_response).await?;
        Ok(())
    }

    pub async fn handoff_to_human(&self, tenant_id: &str, conv_id: Uuid, agent_id: Uuid) -> Result<(), String> {
        self.service.assign_human_agent(tenant_id, conv_id, agent_id).await?;
        self.service.send_message(tenant_id, conv_id, "You are now connected with a human agent.", MessageType::Outgoing, Some(agent_id)).await?;
        Ok(())
    }

    pub async fn resolve(&self, tenant_id: &str, conv_id: Uuid) -> Result<(), String> {
        self.service.resolve_conversation(tenant_id, conv_id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::{ChatEngineService};


    // We can use a mock service for unit tests rather than real postgres
    struct MockService;

    #[async_trait::async_trait]
    impl ChatEngineService for MockService {
        async fn create_conversation(&self, tenant_id: &str, contact_id: Uuid) -> Result<crate::models::Conversation, String> {
            Ok(crate::models::Conversation {
                id: Uuid::new_v4(),
                tenant_id: tenant_id.to_string(),
                contact_id,
                assignee_id: None,
                status: crate::models::ConversationStatus::Open,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        }
        async fn send_message(&self, tenant_id: &str, conv_id: Uuid, content: &str, msg_type: MessageType, sender: Option<Uuid>) -> Result<crate::models::Message, String> {
            Ok(crate::models::Message {
                id: Uuid::new_v4(),
                tenant_id: tenant_id.to_string(),
                conversation_id: conv_id,
                content: content.to_string(),
                message_type: msg_type,
                sender_id: sender,
                is_private: false,
                created_at: chrono::Utc::now(),
            })
        }
        async fn auto_respond(&self, tenant_id: &str, conv_id: Uuid, content: &str) -> Result<crate::models::Message, String> {
            self.send_message(tenant_id, conv_id, content, MessageType::Outgoing, None).await
        }
        async fn assign_human_agent(&self, tenant_id: &str, conv_id: Uuid, agent_id: Uuid) -> Result<crate::models::Conversation, String> {
            Ok(crate::models::Conversation {
                id: conv_id,
                tenant_id: tenant_id.to_string(),
                contact_id: Uuid::new_v4(),
                assignee_id: Some(agent_id),
                status: crate::models::ConversationStatus::Open,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        }
        async fn resolve_conversation(&self, tenant_id: &str, conv_id: Uuid) -> Result<crate::models::Conversation, String> {
            Ok(crate::models::Conversation {
                id: conv_id,
                tenant_id: tenant_id.to_string(),
                contact_id: Uuid::new_v4(),
                assignee_id: None,
                status: crate::models::ConversationStatus::Resolved,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
        }
    }

    #[tokio::test]
    async fn test_engine_flow() {
        let service = Arc::new(MockService);
        let engine = OmnichannelEngine::new(service.clone());

        let tenant = "tenant1";
        let contact = Uuid::new_v4();

        // Simulate incoming message
        let res = engine.handle_incoming_message(tenant, contact, "I need help with my order").await;
        assert!(res.is_ok());

        // Human agent handoff
        let conv_id = Uuid::new_v4();
        let agent = Uuid::new_v4();
        let handoff_res = engine.handoff_to_human(tenant, conv_id, agent).await;
        assert!(handoff_res.is_ok());

        let resolve_res = engine.resolve(tenant, conv_id).await;
        assert!(resolve_res.is_ok());
    }
}
