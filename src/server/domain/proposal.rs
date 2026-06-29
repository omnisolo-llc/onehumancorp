use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Proposal {
    pub id: Uuid,
    pub tenant_id: String,
    pub customer_id: Option<Uuid>,
    pub status: String,
    pub total_amount_cents: i64,
    pub required_deposit_cents: i64,
    pub valid_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[sqlx(skip)]
    pub line_items: Vec<ProposalLineItem>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProposalLineItem {
    pub id: Uuid,
    pub proposal_id: Uuid,
    pub description: String,
    pub unit_price_cents: i64,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
