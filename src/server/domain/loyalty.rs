use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyLedger {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub points_balance: i32,
    pub tier_name: Option<String>,
    pub last_updated: DateTime<Utc>,
}
