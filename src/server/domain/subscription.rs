use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionStatus {
    Active,
    Canceled,
    PastDue,
    Paused,
}

impl Default for SubscriptionStatus {
    fn default() -> Self {
        Self::Active
    }
}

impl AsRef<str> for SubscriptionStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Canceled => "canceled",
            Self::PastDue => "past_due",
            Self::Paused => "paused",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlan {
    pub id: String,
    pub tenant_id: String,
    pub product_id: String,
    pub interval: String, // "weekly", "monthly", "yearly"
    pub interval_count: i32,
    pub status: String,
    pub discount_percentage: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub plan_id: String,
    pub status: String,
    pub current_period_start: i64,
    pub current_period_end: i64,
    pub cancel_at_period_end: bool,
    pub canceled_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FulfillmentStatus {
    Pending,
    LabelsPrinted,
    Fulfilled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FulfillmentBatch {
    pub id: String,
    pub tenant_id: String,
    pub subscription_plan_id: String,
    pub fulfillment_date: String,
    pub subscriber_count: i32,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}
