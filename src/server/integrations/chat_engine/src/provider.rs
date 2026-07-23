use server_integrations_core::ProviderMetadata;
use crate::core::{Conversation, Message, MessageType, Contact};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use server_integrations_core::IntegrationProvider;

pub fn get_provider() -> IntegrationProvider {
    IntegrationProvider {
        metadata: chat_engine_metadata(),
    }
}

pub struct ChatEngineProvider {
    conversations: Arc<Mutex<HashMap<String, Conversation>>>,
    contacts: Arc<Mutex<HashMap<String, Contact>>>,
}

impl ChatEngineProvider {
    pub fn new() -> Self {
        Self {
            conversations: Arc::new(Mutex::new(HashMap::new())),
            contacts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn create_contact(&self, contact: Contact) -> Result<(), String> {
        let mut contacts = self.contacts.lock().await;
        contacts.insert(contact.id.clone(), contact);
        Ok(())
    }

    pub async fn create_conversation(&self, contact_id: &str) -> Result<String, String> {
        let mut convs = self.conversations.lock().await;
        let id = uuid::Uuid::new_v4().to_string();
        convs.insert(id.clone(), Conversation::new(id.clone(), contact_id.to_string()));
        Ok(id)
    }

    pub async fn send_message(&self, conversation_id: &str, content: &str, is_private: bool) -> Result<(), String> {
        let mut convs = self.conversations.lock().await;
        if let Some(conv) = convs.get_mut(conversation_id) {
            let msg = Message {
                id: uuid::Uuid::new_v4().to_string(),
                content: content.to_string(),
                message_type: MessageType::Outgoing,
                created_at: chrono::Utc::now(),
                sender_id: None, // system/agent
                is_private,
            };
            conv.add_message(msg);
            Ok(())
        } else {
            Err("Conversation not found".to_string())
        }
    }

    pub async fn receive_message(&self, conversation_id: &str, content: &str, sender_id: &str) -> Result<(), String> {
        let mut convs = self.conversations.lock().await;
        if let Some(conv) = convs.get_mut(conversation_id) {
            let msg = Message {
                id: uuid::Uuid::new_v4().to_string(),
                content: content.to_string(),
                message_type: MessageType::Incoming,
                created_at: chrono::Utc::now(),
                sender_id: Some(sender_id.to_string()),
                is_private: false,
            };
            conv.add_message(msg);
            Ok(())
        } else {
            Err("Conversation not found".to_string())
        }
    }

    pub async fn get_conversation(&self, id: &str) -> Option<Conversation> {
        let convs = self.conversations.lock().await;
        convs.get(id).cloned()
    }
}

pub fn chat_engine_metadata() -> ProviderMetadata {
    ProviderMetadata {
        id: "chat_engine".to_string(),
        name: "Native Omnichannel Chat Engine".to_string(),
        category: "customer_support".to_string(),
        base_url: "internal://chat_engine".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chat_engine_flow() {
        let engine = ChatEngineProvider::new();

        let contact = Contact {
            id: "c1".to_string(),
            name: "Alice".to_string(),
            email: None,
            phone_number: None,
            custom_attributes: HashMap::new(),
        };

        engine.create_contact(contact).await.unwrap();
        let conv_id = engine.create_conversation("c1").await.unwrap();

        engine.receive_message(&conv_id, "Hello, I need help.", "c1").await.unwrap();
        engine.send_message(&conv_id, "Hi Alice, how can I help you?", false).await.unwrap();

        let conv = engine.get_conversation(&conv_id).await.unwrap();
        assert_eq!(conv.messages.len(), 2);
        assert_eq!(conv.messages[0].message_type, MessageType::Incoming);
        assert_eq!(conv.messages[1].message_type, MessageType::Outgoing);
    }
}
