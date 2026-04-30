use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBooking {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub customer_id: Uuid,
    pub status: String, // "pending, paid, completed, cancelled"
    pub total_amount: f64,
    pub scheduled_for: Option<DateTime<Utc>>, // Null for instant orders
}
