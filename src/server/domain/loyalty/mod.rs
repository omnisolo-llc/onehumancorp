use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgram {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub program_type: String, // 'points', 'punch_card', 'tiers'
    pub config: serde_json::Value,
    pub is_active: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerLoyaltyAccount {
    pub id: String,
    pub tenant_id: String,
    pub program_id: String,
    pub customer_id: String,
    pub points_balance: i32,
    pub punches: i32,
    pub current_tier: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyTransaction {
    pub id: String,
    pub tenant_id: String,
    pub account_id: String,
    pub transaction_type: String, // 'earn', 'redeem', 'adjust'
    pub amount: i32,
    pub reason: Option<String>,
    pub order_id: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reward {
    pub id: String,
    pub tenant_id: String,
    pub program_id: String,
    pub name: String,
    pub description: Option<String>,
    pub cost_in_points: Option<i32>,
    pub cost_in_punches: Option<i32>,
    pub reward_type: String, // 'discount', 'free_item', 'tier_upgrade'
    pub config: serde_json::Value,
    pub is_active: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
