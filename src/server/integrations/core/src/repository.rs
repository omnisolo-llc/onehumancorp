use uuid::Uuid;
use async_trait::async_trait;

use crate::models::{Contact, Inbox, Conversation, Message, OutboxMessage};

#[async_trait]
pub trait OmnichannelRepository: Send + Sync {
    async fn create_contact(&self, tenant_id: Uuid, contact: Contact) -> Result<Contact, String>;
    async fn get_contact(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Contact>, String>;

    async fn create_inbox(&self, tenant_id: Uuid, inbox: Inbox) -> Result<Inbox, String>;
    async fn get_inbox(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Inbox>, String>;

    async fn create_conversation(&self, tenant_id: Uuid, conversation: Conversation) -> Result<Conversation, String>;
    async fn get_conversation(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Conversation>, String>;

    async fn create_message(&self, tenant_id: Uuid, message: Message) -> Result<Message, String>;
    async fn get_message(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Message>, String>;
    async fn get_messages_for_conversation(&self, tenant_id: Uuid, conversation_id: Uuid) -> Result<Vec<Message>, String>;

    // Transactional Outbox operations
    async fn enqueue_outbox_message(&self, tenant_id: Uuid, outbox_msg: OutboxMessage) -> Result<OutboxMessage, String>;
    async fn fetch_pending_outbox_messages(&self, limit: i64) -> Result<Vec<OutboxMessage>, String>;
    async fn mark_outbox_message_completed(&self, tenant_id: Uuid, id: Uuid) -> Result<(), String>;
    async fn mark_outbox_message_failed(&self, tenant_id: Uuid, id: Uuid, attempt_increment: bool) -> Result<(), String>;
}
