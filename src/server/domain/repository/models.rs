use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: String,
    pub organization_id: String,
    pub parent_task_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub assigned_agent_role: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TaskDependency {
    pub task_id: String,
    pub depends_on_task_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub industry: Option<String>,
    pub tier: Option<String>,
    pub owner_email: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Business {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub r#type: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AgentMemory {
    pub id: String,
    pub tenant_id: String,
    pub business_id: Option<String>,
    pub department: Option<String>,
    pub content: String,
    pub embedding: Option<Vec<f32>>,
    pub interaction_data: Option<sqlx::types::Json<serde_json::Value>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Customer {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub preferences: Option<sqlx::types::Json<serde_json::Value>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: String,
    pub tenant_id: String,
    pub r#type: String,
    pub title: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub price_cents: Option<i64>,
    pub currency: Option<String>,
    pub in_stock: Option<bool>,
    pub inventory_count: Option<i32>,
    pub is_sold_out: Option<bool>,
    pub is_perishable: Option<bool>,
    pub default_ttl_minutes: Option<i32>,
    pub metadata: Option<sqlx::types::Json<serde_json::Value>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub status: Option<String>,
    pub total_amount: Option<f64>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Booking {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub product_id: Option<String>,
    pub service_id: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AIAgent {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub role: String,
    pub department: Option<String>,
    pub status: Option<String>,
    pub provider_type: Option<String>,
    pub region: Option<String>,
    pub registered_at: Option<DateTime<Utc>>,
}
