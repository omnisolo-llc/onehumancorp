use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriageFeedItem {
    pub conversation_id: String,
    pub tenant_id: String,
    pub customer_name: String,
    pub channel: String,
    pub last_message: String,
    pub urgency_score: i32,
    pub action_draft: Option<ActionDraft>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionDraft {
    pub draft_id: String,
    pub intent: String,
    pub proposed_reply: String,
    pub amount: Option<f64>,
    pub booking_slot: Option<String>,
}

pub fn get_triage_feed(tenant_id: &str) -> Vec<TriageFeedItem> {
    // Placeholder implementation for retrieving the triage feed
    vec![]
}

pub fn approve_action_draft(draft_id: &str) -> Result<(), String> {
    // Placeholder for approving a draft
    tracing::info!("Approved action draft: {}", draft_id);
    Ok(())
}
