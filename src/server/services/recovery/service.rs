use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoveryCampaign {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub auto_send: bool,
    pub delay_minutes: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecoveryAttempt {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: Option<String>,
    pub source_event_id: String,
    pub assistant_message_id: Option<String>,
    pub status: String,
}

pub struct RecoveryService {
}

impl RecoveryService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn process_intent_dropped(&self, tenant_id: &str, event_id: &str, customer_id: Option<&str>) -> Result<RecoveryAttempt, String> {
        Ok(RecoveryAttempt {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: tenant_id.to_string(),
            customer_id: customer_id.map(|s| s.to_string()),
            source_event_id: event_id.to_string(),
            assistant_message_id: Some("drafted_msg_id".to_string()),
            status: "DRAFTED".to_string(),
        })
    }
}
