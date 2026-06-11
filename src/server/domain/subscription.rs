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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FulfillmentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FulfillmentBatch {
    pub id: String,
    pub tenant_id: String,
    pub plan_id: String,
    pub fulfillment_date: String,
    pub subscriber_count: i64,
    pub status: FulfillmentStatus,
    pub label_url: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Subscription {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub product_id: String,
    pub stripe_subscription_id: Option<String>,
    pub status: String,
    pub current_period_end: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct SubscriptionEvent {
    pub id: String,
    pub tenant_id: String,
    pub subscription_id: String,
    pub event_type: String,
    pub agent_id: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
