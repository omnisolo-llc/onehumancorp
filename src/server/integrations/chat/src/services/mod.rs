use crate::models::{ChannelType, Contact, Conversation, ConversationStatus, Inbox, Message, MessageType};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

// In-memory mock DB for initial implementation, later switch to PostgreSQL/SeaORM
pub struct ChatService {
    inboxes: Arc<RwLock<HashMap<Uuid, Inbox>>>,
    contacts: Arc<RwLock<HashMap<Uuid, Contact>>>,
    conversations: Arc<RwLock<HashMap<Uuid, Conversation>>>,
    messages: Arc<RwLock<HashMap<Uuid, Message>>>,
}

impl Default for ChatService {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatService {
    pub fn new() -> Self {
        Self {
            inboxes: Arc::new(RwLock::new(HashMap::new())),
            contacts: Arc::new(RwLock::new(HashMap::new())),
            conversations: Arc::new(RwLock::new(HashMap::new())),
            messages: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: String, channel_type: ChannelType) -> Inbox {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let inbox = Inbox {
            id,
            tenant_id,
            name,
            channel_type,
            created_at: now,
            updated_at: now,
        };
        self.inboxes.write().await.insert(id, inbox.clone());
        inbox
    }

    pub async fn get_inbox(&self, tenant_id: Uuid, inbox_id: Uuid) -> Option<Inbox> {
        let inboxes = self.inboxes.read().await;
        inboxes.get(&inbox_id).filter(|i| i.tenant_id == tenant_id).cloned()
    }

    pub async fn create_contact(&self, tenant_id: Uuid, name: Option<String>, email: Option<String>, phone: Option<String>) -> Contact {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let contact = Contact {
            id,
            tenant_id,
            name,
            email,
            phone,
            created_at: now,
            updated_at: now,
        };
        self.contacts.write().await.insert(id, contact.clone());
        contact
    }

    pub async fn create_conversation(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid) -> Conversation {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let conversation = Conversation {
            id,
            tenant_id,
            inbox_id,
            contact_id,
            status: ConversationStatus::Open,
            created_at: now,
            updated_at: now,
        };
        self.conversations.write().await.insert(id, conversation.clone());
        conversation
    }

    pub async fn create_message(&self, tenant_id: Uuid, conversation_id: Uuid, sender_id: Option<Uuid>, content: String, message_type: MessageType) -> Message {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let message = Message {
            id,
            tenant_id,
            conversation_id,
            sender_id,
            content,
            message_type,
            created_at: now,
        };
        self.messages.write().await.insert(id, message.clone());
        message
    }

    pub async fn get_conversations_for_inbox(&self, tenant_id: Uuid, inbox_id: Uuid) -> Vec<Conversation> {
        let conversations = self.conversations.read().await;
        conversations.values()
            .filter(|c| c.tenant_id == tenant_id && c.inbox_id == inbox_id)
            .cloned()
            .collect()
    }

    pub async fn get_messages_for_conversation(&self, tenant_id: Uuid, conversation_id: Uuid) -> Vec<Message> {
        let messages = self.messages.read().await;
        let mut result: Vec<Message> = messages.values()
            .filter(|m| m.tenant_id == tenant_id && m.conversation_id == conversation_id)
            .cloned()
            .collect();
        result.sort_by_key(|m| m.created_at);
        result
    }
}
