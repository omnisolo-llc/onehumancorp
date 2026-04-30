use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: Uuid,
    pub business_name: String,
    pub owner_email: String,
    pub subscription_tier: String,
    pub created_at: DateTime<Utc>,
}
