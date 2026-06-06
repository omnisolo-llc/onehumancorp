use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Collective {
    pub id: Uuid,
    pub name: String,
    pub location_center: Option<String>,
    pub radius_meters: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollectiveMember {
    pub id: Uuid,
    pub collective_id: Uuid,
    pub tenant_id: Uuid,
    pub status: String,
    pub joined_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SharedOffer {
    pub id: Uuid,
    pub collective_id: Uuid,
    pub originating_tenant_id: Uuid,
    pub target_tenant_id: Uuid,
    pub discount_type: String,
    pub value: f64,
    pub auto_apply: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollectiveLoyaltyBalance {
    pub id: Uuid,
    pub collective_id: Uuid,
    pub customer_id: Uuid,
    pub points_balance: i32,
    pub last_updated: DateTime<Utc>,
}
