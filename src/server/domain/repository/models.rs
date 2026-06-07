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
pub struct ConversationalCheckoutSession {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    #[sqlx(rename = "type")]
    pub r#type: String,
    pub amount: i64,
    pub status: String,
    pub inventory_lock_id: Option<String>,
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
    pub payment_source: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RawMaterial {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub current_quantity: Option<i32>,
    pub reorder_threshold: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BOMItem {
    pub id: String,
    pub tenant_id: String,
    pub finished_good_id: String,
    pub raw_material_id: String,
    pub quantity_required: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Vendor {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub contact_info: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PurchaseOrder {
    pub id: String,
    pub tenant_id: String,
    pub vendor_id: String,
    pub status: String,
    pub total_cost: Option<f64>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct POLineItem {
    pub id: String,
    pub tenant_id: String,
    pub purchase_order_id: String,
    pub raw_material_id: String,
    pub quantity: Option<i32>,
    pub unit_price: Option<f64>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DepletionLog {
    pub id: String,
    pub tenant_id: String,
    pub raw_material_id: String,
    pub sales_event_id: String,
    pub quantity_deducted: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InventoryPrediction {
    pub id: String,
    pub tenant_id: String,
    pub product_id: String,
    pub predicted_stockout_date: Option<DateTime<Utc>>,
    pub confidence_score: Option<f64>,
    pub suggested_reorder_quantity: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Campaign {
    pub id: String,
    pub tenant_id: String,
    pub goal: String,
    pub status: String,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CampaignAsset {
    pub id: String,
    pub tenant_id: String,
    pub campaign_id: String,
    pub r#type: String,
    pub content_url: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChannelExecution {
    pub id: String,
    pub tenant_id: String,
    pub campaign_id: String,
    pub channel: String,
    pub metrics_sent: Option<i32>,
    pub metrics_clicks: Option<i32>,
    pub metrics_conversions: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PromotionCode {
    pub code: String,
    pub tenant_id: String,
    pub campaign_id: String,
    pub discount_value: Option<f64>,
    pub discount_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LeadGenCampaign {
    pub id: String,
    pub tenant_id: String,
    pub budget: f64,
    pub radius_miles: i32,
    pub zip_code: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SmartPricingPolicy {
    pub id: String,
    pub tenant_id: String,
    pub product_id: String,
    pub min_margin_percent: f64,
    pub auto_discount_trigger_days_stagnant: i32,
    pub max_discount_percent: f64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ActiveDiscount {
    pub id: String,
    pub tenant_id: String,
    pub policy_id: String,
    pub product_id: String,
    pub discount_amount: f64,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}
