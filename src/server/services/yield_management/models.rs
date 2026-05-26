use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct YieldProfile {
    pub id: String,
    pub tenant_id: String,
    pub target_id: String,
    pub target_type: String,
    pub enabled: bool,
    pub min_price_cents: i64,
    pub max_price_cents: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct PriceAdjustmentEvent {
    pub id: String,
    pub tenant_id: String,
    pub yield_profile_id: String,
    pub old_price_cents: i64,
    pub new_price_cents: i64,
    pub reason: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct CapacityState {
    pub id: String,
    pub tenant_id: String,
    pub yield_profile_id: String,
    pub available: i64,
    pub total: i64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct DemandSignal {
    pub id: String,
    pub tenant_id: String,
    pub yield_profile_id: String,
    pub signal_type: String,
    pub score: f64,
    pub created_at: DateTime<Utc>,
}
