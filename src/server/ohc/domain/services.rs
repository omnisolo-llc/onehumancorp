use async_trait::async_trait;

use crate::domain::contact::Contact;
use crate::domain::conversation::Conversation;
use crate::domain::inbox::Inbox;
use crate::domain::message::Message;

#[async_trait]
pub trait InboxRepository: Send + Sync {
    async fn find_by_id(&self, tenant_id: &str, id: &str) -> Result<Option<Inbox>, String>;
    async fn save(&self, inbox: &Inbox) -> Result<(), String>;
    async fn list_by_tenant(&self, tenant_id: &str) -> Result<Vec<Inbox>, String>;
}

#[async_trait]
pub trait ConversationRepository: Send + Sync {
    async fn find_by_id(&self, tenant_id: &str, id: &str) -> Result<Option<Conversation>, String>;
    async fn save(&self, conversation: &Conversation) -> Result<(), String>;
    async fn list_by_inbox(&self, tenant_id: &str, inbox_id: &str) -> Result<Vec<Conversation>, String>;
}

#[async_trait]
pub trait MessageRepository: Send + Sync {
    async fn find_by_id(&self, tenant_id: &str, id: &str) -> Result<Option<Message>, String>;
    async fn save(&self, message: &Message) -> Result<(), String>;
    async fn list_by_conversation(&self, tenant_id: &str, conversation_id: &str) -> Result<Vec<Message>, String>;
}

#[async_trait]
pub trait ContactRepository: Send + Sync {
    async fn find_by_id(&self, tenant_id: &str, id: &str) -> Result<Option<Contact>, String>;
    async fn save(&self, contact: &Contact) -> Result<(), String>;
}
