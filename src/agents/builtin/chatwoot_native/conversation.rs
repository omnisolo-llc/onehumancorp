use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConversationStatus {
    Open,
    Resolved,
    Pending,
    Snoozed,
    Bot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: i64,
    pub account_id: i64,
    pub status: ConversationStatus,
    pub assignee_id: Option<i64>,
    pub assignee_agent_bot_id: Option<i64>,
}

impl Conversation {
    pub fn new(id: i64, account_id: i64) -> Self {
        Self {
            id,
            account_id,
            status: ConversationStatus::Open,
            assignee_id: None,
            assignee_agent_bot_id: None,
        }
    }

    /// Simulates Chatwoot's bot_handoff! logic
    pub fn bot_handoff(&mut self) {
        if self.status == ConversationStatus::Bot {
            self.status = ConversationStatus::Open;
        }
        self.assignee_agent_bot_id = None;
    }
}
