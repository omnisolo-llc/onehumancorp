use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedStorefrontItem {
    pub id: String,
    pub org_id: String,
    pub title: String,
    pub description: String,
    pub item_type: String, // "physical", "digital", "booking"
    pub price_cents: i64,
    pub currency: String,
    pub duration_minutes: Option<i32>,
    pub inventory_count: Option<i32>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct UnifiedModelsManager;

impl UnifiedModelsManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_item(
        &self,
        org_id: &str,
        title: &str,
        description: &str,
        item_type: &str,
        price_cents: i64,
        duration_minutes: Option<i32>,
    ) -> UnifiedStorefrontItem {
        UnifiedStorefrontItem {
            id: format!("item-{}", Uuid::new_v4()),
            org_id: org_id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            item_type: item_type.to_string(),
            price_cents,
            currency: "USD".to_string(),
            duration_minutes,
            inventory_count: if item_type == "physical" { Some(10) } else { None },
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
