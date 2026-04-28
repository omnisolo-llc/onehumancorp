// Billing module - provides Tracker struct used by hub.rs
// The actual billing auditor implementation is in services::billing::auditor
pub use crate::services::billing::auditor::CostAuditor;

pub struct Tracker;

impl Tracker {
    pub fn new() -> Self {
        Tracker
    }

    pub fn summary(&self, _scope: &str) -> TokenSummary {
        TokenSummary::default()
    }
}

#[derive(Default)]
pub struct TokenSummary {
    pub total_tokens: i64,
}

impl Default for Tracker {
    fn default() -> Self {
        Tracker::new()
    }
}
