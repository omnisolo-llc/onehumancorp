use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyState {
    pub customer_id: String,
    pub tenant_id: String,
    pub tier: String,
    pub points: i32,
    pub purchase_frequency: i32,
    pub days_since_last_purchase: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralLinkage {
    pub customer_id: String,
    pub tenant_id: String,
    pub referral_code: String,
    pub referred_count: i32,
}
