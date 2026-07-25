use super::chat::{Contact, ContactInbox, Conversation, Inbox, Message};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Debug)]
pub enum ChatRepositoryError {
    NotFound,
    Other(String),
}

#[async_trait]
pub trait ChatRepository: Send + Sync {
    async fn get_inbox(&self, tenant_id: Uuid, id: Uuid) -> Result<Inbox, ChatRepositoryError>;
    async fn save_inbox(&self, inbox: Inbox) -> Result<(), ChatRepositoryError>;

    async fn get_contact(&self, tenant_id: Uuid, id: Uuid) -> Result<Contact, ChatRepositoryError>;
    async fn save_contact(&self, contact: Contact) -> Result<(), ChatRepositoryError>;

    async fn get_contact_inbox(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> Result<ContactInbox, ChatRepositoryError>;
    async fn save_contact_inbox(
        &self,
        contact_inbox: ContactInbox,
    ) -> Result<(), ChatRepositoryError>;

    async fn get_conversation(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> Result<Conversation, ChatRepositoryError>;
    async fn save_conversation(
        &self,
        conversation: Conversation,
    ) -> Result<(), ChatRepositoryError>;

    async fn get_message(&self, tenant_id: Uuid, id: Uuid) -> Result<Message, ChatRepositoryError>;
    async fn save_message(&self, message: Message) -> Result<(), ChatRepositoryError>;
}

pub struct InMemoryChatRepository {
    inboxes: Arc<RwLock<HashMap<(Uuid, Uuid), Inbox>>>,
    contacts: Arc<RwLock<HashMap<(Uuid, Uuid), Contact>>>,
    contact_inboxes: Arc<RwLock<HashMap<(Uuid, Uuid), ContactInbox>>>,
    conversations: Arc<RwLock<HashMap<(Uuid, Uuid), Conversation>>>,
    messages: Arc<RwLock<HashMap<(Uuid, Uuid), Message>>>,
}

impl InMemoryChatRepository {
    pub fn new() -> Self {
        Self {
            inboxes: Arc::new(RwLock::new(HashMap::new())),
            contacts: Arc::new(RwLock::new(HashMap::new())),
            contact_inboxes: Arc::new(RwLock::new(HashMap::new())),
            conversations: Arc::new(RwLock::new(HashMap::new())),
            messages: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl ChatRepository for InMemoryChatRepository {
    async fn get_inbox(&self, tenant_id: Uuid, id: Uuid) -> Result<Inbox, ChatRepositoryError> {
        let lock = self.inboxes.read().unwrap();
        lock.get(&(tenant_id, id))
            .cloned()
            .ok_or(ChatRepositoryError::NotFound)
    }

    async fn save_inbox(&self, inbox: Inbox) -> Result<(), ChatRepositoryError> {
        let mut lock = self.inboxes.write().unwrap();
        lock.insert((inbox.tenant_id, inbox.id), inbox);
        Ok(())
    }

    async fn get_contact(&self, tenant_id: Uuid, id: Uuid) -> Result<Contact, ChatRepositoryError> {
        let lock = self.contacts.read().unwrap();
        lock.get(&(tenant_id, id))
            .cloned()
            .ok_or(ChatRepositoryError::NotFound)
    }

    async fn save_contact(&self, contact: Contact) -> Result<(), ChatRepositoryError> {
        let mut lock = self.contacts.write().unwrap();
        lock.insert((contact.tenant_id, contact.id), contact);
        Ok(())
    }

    async fn get_contact_inbox(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> Result<ContactInbox, ChatRepositoryError> {
        let lock = self.contact_inboxes.read().unwrap();
        lock.get(&(tenant_id, id))
            .cloned()
            .ok_or(ChatRepositoryError::NotFound)
    }

    async fn save_contact_inbox(
        &self,
        contact_inbox: ContactInbox,
    ) -> Result<(), ChatRepositoryError> {
        let mut lock = self.contact_inboxes.write().unwrap();
        lock.insert((contact_inbox.tenant_id, contact_inbox.id), contact_inbox);
        Ok(())
    }

    async fn get_conversation(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> Result<Conversation, ChatRepositoryError> {
        let lock = self.conversations.read().unwrap();
        lock.get(&(tenant_id, id))
            .cloned()
            .ok_or(ChatRepositoryError::NotFound)
    }

    async fn save_conversation(
        &self,
        conversation: Conversation,
    ) -> Result<(), ChatRepositoryError> {
        let mut lock = self.conversations.write().unwrap();
        lock.insert((conversation.tenant_id, conversation.id), conversation);
        Ok(())
    }

    async fn get_message(&self, tenant_id: Uuid, id: Uuid) -> Result<Message, ChatRepositoryError> {
        let lock = self.messages.read().unwrap();
        lock.get(&(tenant_id, id))
            .cloned()
            .ok_or(ChatRepositoryError::NotFound)
    }

    async fn save_message(&self, message: Message) -> Result<(), ChatRepositoryError> {
        let mut lock = self.messages.write().unwrap();
        lock.insert((message.tenant_id, message.id), message);
        Ok(())
    }
}
