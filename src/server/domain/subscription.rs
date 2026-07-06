use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionStatus {
    Active,
    Canceled,
    PastDue,
    Unpaid,
    Incomplete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlan {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub description: String,
    pub amount: i64,
    pub currency: String,
    pub interval: String, // "day", "week", "month", "year"
    pub active: bool,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscriber {
    pub id: String,
    pub tenant_id: String,
    pub plan_id: String,
    pub customer_id: String,
    pub stripe_subscription_id: String,
    pub status: SubscriptionStatus,
    pub current_period_end: i64,
    pub created_at: i64,
    pub predicted_restock_date: Option<i64>,
    pub health_score: i32,
    pub last_engagement_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FulfillmentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FulfillmentSchedule {
    pub id: String,
    pub tenant_id: String,
    pub plan_id: String,
    pub fulfillment_date: String,
    pub subscriber_count: i64,
    pub status: FulfillmentStatus,
    pub label_url: Option<String>,
    pub created_at: i64,
}
