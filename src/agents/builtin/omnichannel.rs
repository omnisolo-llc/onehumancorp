use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;

/// Master Catalog: Chatwoot Retirement & Native Rust Omnichannel Chat Integration
/// External Chatwoot dependencies are 100% RETIRED. The builtin AI agent microservice
/// connects directly via high-performance Rust IPC/gRPC to OHC's native Rust Chat Engine.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: i64,
    pub additional_attributes: Option<Value>,
    pub blocked: bool,
    pub contact_type: i32,
    pub country_code: String,
    pub custom_attributes: Option<Value>,
    pub email: String,
    pub identifier: String,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub last_name: String,
    pub location: String,
    pub middle_name: String,
    pub name: String,
    pub phone_number: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub id: i64,
    pub allow_messages_after_resolved: bool,
    pub auto_assignment_config: Option<Value>,
    pub business_name: Option<String>,
    pub channel_type: String,
    pub csat_config: Option<Value>,
    pub csat_survey_enabled: bool,
    pub email_address: Option<String>,
    pub enable_auto_assignment: bool,
    pub enable_email_collect: bool,
    pub greeting_enabled: bool,
    pub greeting_message: Option<String>,
    pub lock_to_single_conversation: bool,
    pub name: String,
    pub out_of_office_message: Option<String>,
    pub sender_name_type: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: i64,
    pub additional_attributes: Option<Value>,
    pub agent_last_seen_at: Option<DateTime<Utc>>,
    pub assignee_last_seen_at: Option<DateTime<Utc>>,
    pub cached_label_list: Option<String>,
    pub contact_last_seen_at: Option<DateTime<Utc>>,
    pub custom_attributes: Option<Value>,
    pub first_reply_created_at: Option<DateTime<Utc>>,
    pub identifier: Option<String>,
    pub last_activity_at: DateTime<Utc>,
    pub priority: Option<i32>,
    pub snoozed_until: Option<DateTime<Utc>>,
    pub status: i32,
    pub status_changed_at: Option<DateTime<Utc>>,
    pub uuid: Uuid,
    pub waiting_since: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub inbox_id: i64,
    pub contact_id: i64,
    pub assignee_id: Option<i64>,
    pub assignee_agent_bot_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub additional_attributes: Option<Value>,
    pub content: Option<String>,
    pub content_attributes: Option<Value>,
    pub content_type: i32,
    pub external_source_ids: Option<Value>,
    pub message_type: i32,
    pub private: bool,
    pub processed_message_content: Option<String>,
    pub sender_type: Option<String>,
    pub sentiment: Option<Value>,
    pub status: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub conversation_id: i64,
    pub inbox_id: i64,
}

pub struct NativeChatEngine {
    contacts: RwLock<HashMap<i64, Contact>>,
    inboxes: RwLock<HashMap<i64, Inbox>>,
    conversations: RwLock<HashMap<i64, Conversation>>,
    messages: RwLock<HashMap<i64, Message>>,
    next_contact_id: std::sync::atomic::AtomicI64,
    next_inbox_id: std::sync::atomic::AtomicI64,
    next_conversation_id: std::sync::atomic::AtomicI64,
    next_message_id: std::sync::atomic::AtomicI64,
}

impl NativeChatEngine {
    pub fn new() -> Self {
        Self {
            contacts: RwLock::new(HashMap::new()),
            inboxes: RwLock::new(HashMap::new()),
            conversations: RwLock::new(HashMap::new()),
            messages: RwLock::new(HashMap::new()),
            next_contact_id: std::sync::atomic::AtomicI64::new(1),
            next_inbox_id: std::sync::atomic::AtomicI64::new(1),
            next_conversation_id: std::sync::atomic::AtomicI64::new(1),
            next_message_id: std::sync::atomic::AtomicI64::new(1),
        }
    }

    pub fn create_contact(&self, name: String, email: String) -> Contact {
        let id = self.next_contact_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let contact = Contact {
            id,
            additional_attributes: None,
            blocked: false,
            contact_type: 0,
            country_code: "".to_string(),
            custom_attributes: None,
            email,
            identifier: format!("visitor-{}", id),
            last_activity_at: None,
            last_name: "".to_string(),
            location: "".to_string(),
            middle_name: "".to_string(),
            name,
            phone_number: "".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.contacts.write().unwrap().insert(id, contact.clone());
        contact
    }

    pub fn create_inbox(&self, name: String, channel_type: String) -> Inbox {
        let id = self.next_inbox_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let inbox = Inbox {
            id,
            allow_messages_after_resolved: true,
            auto_assignment_config: None,
            business_name: None,
            channel_type,
            csat_config: None,
            csat_survey_enabled: false,
            email_address: None,
            enable_auto_assignment: true,
            enable_email_collect: false,
            greeting_enabled: false,
            greeting_message: None,
            lock_to_single_conversation: false,
            name,
            out_of_office_message: None,
            sender_name_type: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.inboxes.write().unwrap().insert(id, inbox.clone());
        inbox
    }

    pub fn create_conversation(&self, inbox_id: i64, contact_id: i64) -> Conversation {
        let id = self.next_conversation_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let conversation = Conversation {
            id,
            additional_attributes: None,
            agent_last_seen_at: None,
            assignee_last_seen_at: None,
            cached_label_list: None,
            contact_last_seen_at: None,
            custom_attributes: None,
            first_reply_created_at: None,
            identifier: None,
            last_activity_at: Utc::now(),
            priority: None,
            snoozed_until: None,
            status: 0,
            status_changed_at: None,
            uuid: Uuid::new_v4(),
            waiting_since: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            inbox_id,
            contact_id,
            assignee_id: None,
            assignee_agent_bot_id: None,
        };

        self.conversations.write().unwrap().insert(id, conversation.clone());
        self.auto_assign(id);

        self.conversations.read().unwrap().get(&id).unwrap().clone()
    }

    pub fn create_message(&self, conversation_id: i64, content: String, sender_type: String) -> Message {
        let id = self.next_message_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        let inbox_id = {
            let convs = self.conversations.read().unwrap();
            let conv = convs.get(&conversation_id).expect("Conversation must exist");
            conv.inbox_id
        };

        let message = Message {
            id,
            additional_attributes: None,
            content: Some(content),
            content_attributes: None,
            content_type: 0,
            external_source_ids: None,
            message_type: 0, // 0 = incoming, 1 = outgoing
            private: false,
            processed_message_content: None,
            sender_type: Some(sender_type),
            sentiment: None,
            status: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            conversation_id,
            inbox_id,
        };

        self.messages.write().unwrap().insert(id, message.clone());
        message
    }

    pub fn auto_assign(&self, conversation_id: i64) {
        let mut convs = self.conversations.write().unwrap();
        if let Some(conv) = convs.get_mut(&conversation_id) {
            let inboxes = self.inboxes.read().unwrap();
            if let Some(inbox) = inboxes.get(&conv.inbox_id) {
                if inbox.enable_auto_assignment {
                    // Route to bot/LLM
                    conv.assignee_agent_bot_id = Some(999);
                } else {
                    // Route to human
                    conv.assignee_id = Some(101);
                }
                conv.updated_at = Utc::now();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_contact_and_inbox() {
        let engine = NativeChatEngine::new();
        let contact = engine.create_contact("John Doe".to_string(), "john@example.com".to_string());
        assert_eq!(contact.name, "John Doe");
        assert_eq!(contact.email, "john@example.com");

        let inbox = engine.create_inbox("Support Inbox".to_string(), "Email".to_string());
        assert_eq!(inbox.name, "Support Inbox");
        assert_eq!(inbox.channel_type, "Email");
    }

    #[test]
    fn test_auto_assign_bot() {
        let engine = NativeChatEngine::new();
        let contact = engine.create_contact("John".to_string(), "john@example.com".to_string());
        let inbox = engine.create_inbox("Support".to_string(), "Web".to_string());

        let conv = engine.create_conversation(inbox.id, contact.id);

        // Since enable_auto_assignment is true by default, it should assign a bot.
        assert_eq!(conv.assignee_agent_bot_id, Some(999));
        assert_eq!(conv.assignee_id, None);
    }

    #[test]
    fn test_auto_assign_human() {
        let engine = NativeChatEngine::new();
        let contact = engine.create_contact("Jane".to_string(), "jane@example.com".to_string());

        let mut inbox = engine.create_inbox("Sales".to_string(), "Phone".to_string());
        inbox.enable_auto_assignment = false;
        engine.inboxes.write().unwrap().insert(inbox.id, inbox.clone());

        let conv = engine.create_conversation(inbox.id, contact.id);

        assert_eq!(conv.assignee_id, Some(101));
        assert_eq!(conv.assignee_agent_bot_id, None);
    }
}
