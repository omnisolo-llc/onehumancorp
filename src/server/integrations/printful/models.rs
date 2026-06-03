use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrintOnDemandProduct {
    pub id: String,
    pub name: String,
    pub base_cost: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mockup {
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FulfillmentOrder {
    pub id: String,
    pub status: String,
}
