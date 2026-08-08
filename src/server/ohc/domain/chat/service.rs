use super::models::{Conversation, Message};
use super::repository::ChatRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct ChatService {
    repository: Arc<dyn ChatRepository>,
}

impl ChatService {
    pub fn new(repository: Arc<dyn ChatRepository>) -> Self {
        Self { repository }
    }

    pub async fn start_conversation(
        &self,
        tenant_id: Uuid,
        conversation: Conversation,
    ) -> Result<Conversation, String> {
        // Enforce tenant isolation in the service layer as an extra check, although repository should also enforce it
        if tenant_id != conversation.tenant_id {
            return Err("Tenant ID mismatch".to_string());
        }
        self.repository.create_conversation(tenant_id, conversation).await
    }

    pub async fn add_message_to_conversation(
        &self,
        tenant_id: Uuid,
        message: Message,
    ) -> Result<Message, String> {
        if tenant_id != message.tenant_id {
            return Err("Tenant ID mismatch".to_string());
        }

        // Verify conversation exists and belongs to tenant
        let conversation = self.repository.get_conversation(tenant_id, message.conversation_id).await?;
        if conversation.is_none() {
            return Err("Conversation not found".to_string());
        }

        self.repository.add_message(tenant_id, message).await
    }

    pub async fn get_conversation_history(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<Vec<Message>, String> {
        self.repository.get_messages_for_conversation(tenant_id, conversation_id).await
    }
}
