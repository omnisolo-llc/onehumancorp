use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer360 {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub mood: Option<String>,
    pub preferences: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
