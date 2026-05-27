use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CapacityLedger {
    pub entry_id: String,
    pub item_id: String,
    pub tenant_id: String,
    pub available_quantity: i32,
    pub expiration_time: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FlashSaleEvent {
    pub event_id: String,
    pub ledger_entry_id: String,
    pub tenant_id: String,
    pub target_audience: Option<String>,
    pub discount_amount: Option<f64>,
    pub broadcast_time: Option<DateTime<Utc>>,
    pub status: Option<String>,
}
