use crate::models::{ChannelProvider, Contact, Conversation, ConversationStatus, Message};
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

pub struct OmnichannelGateway {
    // In-memory mock for DB layer
    contacts: Arc<Mutex<HashMap<Uuid, Contact>>>,
    conversations: Arc<Mutex<HashMap<Uuid, Conversation>>>,
    messages: Arc<Mutex<HashMap<Uuid, Message>>>,
}

impl OmnichannelGateway {
    pub fn new() -> Self {
        Self {
            contacts: Arc::new(Mutex::new(HashMap::new())),
            conversations: Arc::new(Mutex::new(HashMap::new())),
            messages: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn ingest_webhook(&self, tenant_id: Uuid, _channel: ChannelProvider, sender_identifier: String, content: String) -> Result<Message, String> {
        let mut contacts = self.contacts.lock().await;

        let contact = contacts.values().find(|c| c.tenant_id == tenant_id && (c.email == Some(sender_identifier.clone()) || c.phone == Some(sender_identifier.clone()) || c.name == sender_identifier));

        let contact_id = if let Some(c) = contact {
            c.id
        } else {
            let id = Uuid::new_v4();
            contacts.insert(id, Contact {
                id,
                tenant_id,
                name: sender_identifier.clone(),
                email: None,
                phone: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });
            id
        };

        let mut conversations = self.conversations.lock().await;
        let conversation = conversations.values().find(|c| c.tenant_id == tenant_id && c.contact_id == contact_id && c.status == ConversationStatus::Open);

        let conversation_id = if let Some(c) = conversation {
            c.id
        } else {
            let id = Uuid::new_v4();
            conversations.insert(id, Conversation {
                id,
                tenant_id,
                inbox_id: Uuid::new_v4(), // Mock inbox
                contact_id,
                status: ConversationStatus::Open,
                snoozed_until: None,
                assignee_agent_bot_id: None, // Will be set by rules engine in a real app
                contact_last_seen_at: Some(Utc::now()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            });
            id
        };

        let mut messages = self.messages.lock().await;
        let message_id = Uuid::new_v4();
        let message = Message {
            id: message_id,
            tenant_id,
            conversation_id,
            content: content.clone(),
            sender_id: Some(contact_id),
            is_draft: false,
            created_at: Utc::now(),
        };
        messages.insert(message_id, message.clone());

        // Trigger Ambassador Agent drafting async

        let messages_clone = Arc::clone(&self.messages);

        tokio::spawn(async move {
            Self::trigger_ambassador_agent(tenant_id, conversation_id, content, messages_clone).await;
        });

        Ok(message)
    }

    async fn trigger_ambassador_agent(tenant_id: Uuid, conversation_id: Uuid, content: String, messages: Arc<Mutex<HashMap<Uuid, Message>>>) {
        // Mock Ambassador Agent RAG logic
        let draft_content = format!("Draft reply to: {}", content);
        let mut msgs = messages.lock().await;
        let draft_msg = Message {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id,
            content: draft_content,
            sender_id: None, // System/Agent
            is_draft: true, // Requires owner approval
            created_at: Utc::now(),
        };
        msgs.insert(draft_msg.id, draft_msg);
    }

    pub async fn approve_draft(&self, tenant_id: Uuid, message_id: Uuid) -> Result<Message, String> {
        let mut msgs = self.messages.lock().await;
        if let Some(msg) = msgs.get_mut(&message_id) {
            if msg.tenant_id == tenant_id && msg.is_draft {
                msg.is_draft = false;
                // In a real app, this would dispatch via Rust Omnichannel Dispatcher to external channel
                return Ok(msg.clone());
            }
        }
        Err("Draft not found or already approved".to_string())
    }

    pub async fn get_drafts(&self, tenant_id: Uuid) -> Vec<Message> {
        let msgs = self.messages.lock().await;
        msgs.values().filter(|m| m.tenant_id == tenant_id && m.is_draft).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ingest_webhook_and_auto_draft() {
        let gateway = OmnichannelGateway::new();
        let tenant_id = Uuid::new_v4();

        let msg = gateway.ingest_webhook(tenant_id, ChannelProvider::Instagram, "maya_baker".to_string(), "Do you make vegan cakes?".to_string()).await.unwrap();

        assert_eq!(msg.content, "Do you make vegan cakes?");
        assert_eq!(msg.is_draft, false);

        // Yield to allow async agent draft to process
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let drafts = gateway.get_drafts(tenant_id).await;
        assert_eq!(drafts.len(), 1);
        assert_eq!(drafts[0].content, "Draft reply to: Do you make vegan cakes?");
        assert_eq!(drafts[0].is_draft, true);
    }

    #[tokio::test]
    async fn test_approve_draft() {
        let gateway = OmnichannelGateway::new();
        let tenant_id = Uuid::new_v4();

        gateway.ingest_webhook(tenant_id, ChannelProvider::WhatsApp, "1234567890".to_string(), "Hi".to_string()).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let drafts = gateway.get_drafts(tenant_id).await;
        let draft_id = drafts[0].id;

        let approved = gateway.approve_draft(tenant_id, draft_id).await.unwrap();
        assert_eq!(approved.is_draft, false);

        let updated_drafts = gateway.get_drafts(tenant_id).await;
        assert_eq!(updated_drafts.len(), 0);
    }
}
