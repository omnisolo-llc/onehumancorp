use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionTimeline {
    pub id: Uuid,
    pub tenant_id: String,
    pub customer_id: String,
    pub source: String,
    pub sentiment: String,
    pub occurred_at: DateTime<Utc>,
}
