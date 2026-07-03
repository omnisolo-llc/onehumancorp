use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    pub code: String,
    pub exchange_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPrice {
    pub id: Uuid,
    pub product_id: Uuid,
    pub currency_code: String,
    pub localized_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheInvalidationEvent {
    pub id: Uuid,
    pub path: String,
    pub triggered_at: i64,
}
