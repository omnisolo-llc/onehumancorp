use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tenant {
    pub id: uuid::Uuid,
    pub business_name: String,
    pub owner_email: String,
    pub subscription_tier: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub r#type: String, // 'type' is a reserved keyword in Rust
    pub title: String,
    pub price: Decimal,
    pub stock_level: i32,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Customer {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderBooking {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub customer_id: uuid::Uuid,
    pub status: String,
    pub total_amount: Decimal,
    pub scheduled_for: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub order_id: uuid::Uuid,
    pub product_id: uuid::Uuid,
    pub quantity: i32,
    pub unit_price: Decimal,
}

#[cfg(test)]
mod tests;
