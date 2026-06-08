use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum QuoteStatus {
    Draft,
    Sent,
    Approved,
    Rejected,
    Expired,
}

impl Default for QuoteStatus {
    fn default() -> Self {
        QuoteStatus::Draft
    }
}

impl ToString for QuoteStatus {
    fn to_string(&self) -> String {
        match self {
            QuoteStatus::Draft => "DRAFT".to_string(),
            QuoteStatus::Sent => "SENT".to_string(),
            QuoteStatus::Approved => "APPROVED".to_string(),
            QuoteStatus::Rejected => "REJECTED".to_string(),
            QuoteStatus::Expired => "EXPIRED".to_string(),
        }
    }
}

impl std::str::FromStr for QuoteStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DRAFT" => Ok(QuoteStatus::Draft),
            "SENT" => Ok(QuoteStatus::Sent),
            "APPROVED" => Ok(QuoteStatus::Approved),
            "REJECTED" => Ok(QuoteStatus::Rejected),
            "EXPIRED" => Ok(QuoteStatus::Expired),
            _ => Err(format!("Unknown status: {}", s)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LineItem {
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub total: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomQuote {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub customer_id: Option<String>,
    pub status: QuoteStatus,
    pub total_amount: f64,
    pub proposed_completion_date: Option<DateTime<Utc>>,
    pub line_items: Vec<LineItem>,
    pub original_request: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
