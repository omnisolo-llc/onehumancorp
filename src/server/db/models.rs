use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tenant {
    pub id: String,
    pub business_name: String,
    pub owner_email: String,
    pub subscription_tier: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: String,
    pub tenant_id: String,
    pub r#type: String, // "physical", "digital", "service"
    pub title: String,
    pub price_cents: i64,
    pub stock_level: i32,
    pub is_active: bool,
    pub updated_at: Option<DateTime<Utc>>,
    pub version: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Customer {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub version: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderBooking {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub status: String, // "pending", "paid", "completed", "cancelled"
    pub total_amount_cents: i64,
    pub scheduled_for: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub version: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderItem {
    pub id: String,
    pub tenant_id: String,
    pub order_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub unit_price_cents: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AgentMemory {
    pub id: String,
    pub tenant_id: String,
    pub department: String, // "operations", "marketing", "finance"...
    pub context_summary: String,
    // embedding is handled separately depending on the DB (pgvector vs blob)
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub version: i32,
}
