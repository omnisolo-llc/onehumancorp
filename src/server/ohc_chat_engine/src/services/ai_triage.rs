use crate::models::{Message, SuggestedReply};
use uuid::Uuid;
use chrono::Utc;
use serde_json::json;

pub struct AiTriageAgent;

impl AiTriageAgent {
    pub async fn process_message(message: &Message) -> SuggestedReply {
        let draft_content = if message.content.to_lowercase().contains("vegan") {
            "Yes, we have a variety of vegan options. Would you like a quote?"
        } else {
            "Thanks for reaching out! How can I help you today?"
        };

        SuggestedReply {
            id: Uuid::new_v4(),
            tenant_id: message.tenant_id,
            message_id: message.id,
            content: draft_content.to_string(),
            action_payload: Some(json!({"action": "create_quote"})),
            created_at: Utc::now(),
        }
    }
}
