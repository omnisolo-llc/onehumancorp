use crate::models::{contact, conversation, message};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[async_trait]
pub trait ChatRepository: Send + Sync {
    async fn find_contact_by_identity(
        &self,
        tenant_id: Uuid,
        identity: &str,
    ) -> Result<Option<contact::Model>, String>;
    async fn create_contact(&self, contact: contact::Model) -> Result<contact::Model, String>;

    async fn find_active_conversation(
        &self,
        tenant_id: Uuid,
        contact_id: Uuid,
    ) -> Result<Option<conversation::Model>, String>;
    async fn create_conversation(
        &self,
        conversation: conversation::Model,
    ) -> Result<conversation::Model, String>;
    async fn update_conversation(
        &self,
        conversation: conversation::Model,
    ) -> Result<conversation::Model, String>;

    async fn create_message(&self, message: message::Model) -> Result<message::Model, String>;
}

#[derive(Default, Clone)]
pub struct InMemoryChatRepository {
    contacts: Arc<Mutex<Vec<contact::Model>>>,
    conversations: Arc<Mutex<Vec<conversation::Model>>>,
    messages: Arc<Mutex<Vec<message::Model>>>,
}

impl InMemoryChatRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl ChatRepository for InMemoryChatRepository {
    async fn find_contact_by_identity(
        &self,
        tenant_id: Uuid,
        identity: &str,
    ) -> Result<Option<contact::Model>, String> {
        let contacts = self.contacts.lock().await;
        Ok(contacts
            .iter()
            .find(|c| c.tenant_id == tenant_id && c.channel_identity == identity)
            .cloned())
    }

    async fn create_contact(&self, contact: contact::Model) -> Result<contact::Model, String> {
        let mut contacts = self.contacts.lock().await;
        contacts.push(contact.clone());
        Ok(contact)
    }

    async fn find_active_conversation(
        &self,
        tenant_id: Uuid,
        contact_id: Uuid,
    ) -> Result<Option<conversation::Model>, String> {
        let conversations = self.conversations.lock().await;
        Ok(conversations
            .iter()
            .find(|c| c.tenant_id == tenant_id && c.contact_id == contact_id && c.status == "open")
            .cloned())
    }

    async fn create_conversation(
        &self,
        conversation: conversation::Model,
    ) -> Result<conversation::Model, String> {
        let mut conversations = self.conversations.lock().await;
        conversations.push(conversation.clone());
        Ok(conversation)
    }

    async fn update_conversation(
        &self,
        conversation: conversation::Model,
    ) -> Result<conversation::Model, String> {
        let mut conversations = self.conversations.lock().await;
        if let Some(existing) = conversations.iter_mut().find(|c| c.id == conversation.id) {
            *existing = conversation.clone();
        }
        Ok(conversation)
    }

    async fn create_message(&self, message: message::Model) -> Result<message::Model, String> {
        let mut messages = self.messages.lock().await;
        messages.push(message.clone());
        Ok(message)
    }
}
