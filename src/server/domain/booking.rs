use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub id: String,
    pub organization_id: String,
    pub customer_id: String,
    pub amount_cents: i64,
    pub description: String,
    pub status: String, // DRAFT, PENDING_APPROVAL, APPROVED, REJECTED
    pub created_at_unix: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Booking {
    pub id: String,
    pub organization_id: String,
    pub customer_id: String,
    pub quote_id: Option<String>,
    pub start_time_unix: i64,
    pub end_time_unix: i64,
    pub status: String, // PENDING, CONFIRMED, CANCELLED
    pub payment_link: Option<String>,
}
