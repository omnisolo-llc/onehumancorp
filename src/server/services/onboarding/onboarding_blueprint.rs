use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessBlueprint {
    pub catalog_schema: String,
    pub dummy_inventory: Vec<String>,
    pub booking_availability: Option<String>,
    pub default_policies: Vec<String>,
}
