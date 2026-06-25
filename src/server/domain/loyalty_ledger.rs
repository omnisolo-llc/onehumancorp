use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct LoyaltyProgram {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub program_type: String,
    pub config: sqlx::types::Json<serde_json::Value>,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct CustomerLoyaltyAccount {
    pub id: String,
    pub tenant_id: String,
    pub program_id: String,
    pub customer_id: String,
    pub points_balance: i32,
    pub punches: i32,
    pub tier_name: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct LoyaltyTransaction {
    pub id: String,
    pub tenant_id: String,
    pub account_id: String,
    pub transaction_type: String,
    pub amount: i32,
    pub reason: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct LoyaltyReward {
    pub id: String,
    pub tenant_id: String,
    pub program_id: String,
    pub name: String,
    pub description: Option<String>,
    pub cost_in_points: i32,
    pub reward_type: String,
    pub reward_value: sqlx::types::Json<serde_json::Value>,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
