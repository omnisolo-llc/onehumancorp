use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub r#type: String, // "physical, digital, service"
    pub title: String,
    pub price: f64,
    pub stock_level: i32,
    pub is_active: bool,
}
