use super::models::{Customer, Message, DraftReply};
use std::collections::HashMap;

pub struct CustomerIdentityResolutionEngine {
    // Mock DB for customers
    customers: HashMap<String, Customer>,
}

impl CustomerIdentityResolutionEngine {
    pub fn new() -> Self {
        Self {
            customers: HashMap::new(),
        }
    }

    pub fn add_customer(&mut self, customer: Customer) {
        self.customers.insert(customer.id.clone(), customer);
    }

    pub fn resolve_identity(&self, channel: &str, handle: &str) -> Option<Customer> {
        self.customers.values().find(|c| {
            match channel {
                "instagram" => c.instagram_handle.as_deref() == Some(handle),
                "whatsapp" => c.whatsapp_number.as_deref() == Some(handle),
                "email" => c.primary_email.as_deref() == Some(handle),
                _ => false,
            }
        }).cloned()
    }
}

pub struct AmbassadorAgent {
    // Mock DB for drafts
    drafts: HashMap<String, DraftReply>,
}

impl AmbassadorAgent {
    pub fn new() -> Self {
        Self {
            drafts: HashMap::new(),
        }
    }

    pub fn process_message(&mut self, message: &Message, customer: &Customer) -> DraftReply {
        // RAG mock: In a real system, query context DB
        let draft_content = format!(
            "Hi {}, we received your message on {}: '{}'. We will get back to you shortly.",
            customer.instagram_handle.as_deref().unwrap_or("Customer"),
            message.channel,
            message.content
        );

        let draft = DraftReply {
            message_id: message.id.clone(),
            draft_content,
            status: "pending_approval".to_string(),
        };

        self.drafts.insert(draft.message_id.clone(), draft.clone());
        draft
    }

    pub fn get_draft(&self, message_id: &str) -> Option<DraftReply> {
        self.drafts.get(message_id).cloned()
    }
}
