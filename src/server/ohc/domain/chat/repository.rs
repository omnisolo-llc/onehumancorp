use super::models::{Contact, Conversation, Inbox, Message};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ChatRepository: Send + Sync {
    async fn create_conversation(
        &self,
        tenant_id: Uuid,
        conversation: Conversation,
    ) -> Result<Conversation, String>;

    async fn get_conversation(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<Option<Conversation>, String>;

    async fn add_message(
        &self,
        tenant_id: Uuid,
        message: Message,
    ) -> Result<Message, String>;

    async fn get_messages_for_conversation(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<Vec<Message>, String>;

    async fn create_inbox(
        &self,
        tenant_id: Uuid,
        inbox: Inbox,
    ) -> Result<Inbox, String>;

    async fn create_contact(
        &self,
        tenant_id: Uuid,
        contact: Contact,
    ) -> Result<Contact, String>;
}
