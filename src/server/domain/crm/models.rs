use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Customer360 {
    pub id: String,
    pub tenant_id: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub mood: String,
    pub preferences: sqlx::types::Json<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InteractionTimeline {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub source: String,
    pub sentiment: String,
    pub occurred_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LoyaltyLedger {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub points_balance: i32,
    pub tier_name: String,
    pub last_updated: Option<DateTime<Utc>>,
}
