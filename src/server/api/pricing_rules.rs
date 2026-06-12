use axum::{
    extract::Query,
    response::IntoResponse,
    Json,
    Router,
    routing::get,

};
use serde::{Deserialize, Serialize};
use sqlx::Row;

#[derive(Deserialize)]
pub struct PricingRulesQuery {
    pub tenant_id: Option<String>,
    pub service_category: Option<String>,
}

#[derive(Serialize)]
pub struct PricingRuleModifier {
    pub id: String,
    pub name: String,
    pub description: String,
    pub modifier_type: String, // "flat" or "multiplier" or "percentage"
    pub amount: f64,
}

#[derive(Serialize)]
pub struct PricingRuleOptionGroup {
    pub id: String,
    pub name: String,
    pub is_multiple_choice: bool,
    pub options: Vec<PricingRuleModifier>,
}

#[derive(Serialize)]
pub struct PricingRulesResponse {
    pub tenant_id: String,
    pub base_price: f64,
    pub option_groups: Vec<PricingRuleOptionGroup>,
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    // The pool needs to be in the state or we fetch it globally
    Router::new().route("/", get(get_pricing_rules))
}

async fn get_pricing_rules(Query(query): Query<PricingRulesQuery>) -> impl IntoResponse {
    let tenant_id = query.tenant_id.unwrap_or_else(|| "default".to_string());
    let service_category = query.service_category.unwrap_or_else(|| "general".to_string());

    // We will query the global pool
    let pool = crate::db::get_pool();

    // Query pricing_heuristics
    let heuristic_query = sqlx::query(
        "SELECT base_rate_cents, materials_markup_percentage, instructions FROM pricing_heuristics WHERE tenant_id = $1 AND service_category = $2 ORDER BY created_at DESC LIMIT 1"
    )
    .bind(&tenant_id)
    .bind(&service_category)
    .fetch_optional(&pool)
    .await;

    let (base_price, _instructions) = match heuristic_query {
        Ok(Some(row)) => {
            let base_rate_cents: i64 = row.get("base_rate_cents");

            let instructions: String = row.get("instructions");
            (base_rate_cents as f64 / 100.0, instructions)
        },
        _ => {
            // Fallback default
            (50.0, "".to_string())
        }
    };

    // To keep it simple for the deterministic UI, we provide some parsed options
    // In a fully developed system, these would also be parsed from the DB instructions
    // or from a separate `pricing_modifiers` table. We use standard options based on tenant type.

    let option_groups = if tenant_id.contains("cake") || tenant_id.contains("bakery") || tenant_id == "demo" {
        vec![
            PricingRuleOptionGroup {
                id: "cake_size".to_string(),
                name: "Cake Size".to_string(),
                is_multiple_choice: false,
                options: vec![
                    PricingRuleModifier { id: "size_small".to_string(), name: "Small (6\")".to_string(), description: "Serves 6-8".to_string(), modifier_type: "flat".to_string(), amount: 0.0 },
                    PricingRuleModifier { id: "size_medium".to_string(), name: "Medium (8\")".to_string(), description: "Serves 10-14".to_string(), modifier_type: "flat".to_string(), amount: 25.0 },
                    PricingRuleModifier { id: "size_large".to_string(), name: "Large (10\")".to_string(), description: "Serves 16-20".to_string(), modifier_type: "flat".to_string(), amount: 50.0 },
                ],
            },
            PricingRuleOptionGroup {
                id: "dietary".to_string(),
                name: "Dietary Preferences".to_string(),
                is_multiple_choice: true,
                options: vec![
                    PricingRuleModifier { id: "vegan".to_string(), name: "Vegan".to_string(), description: "Dairy and egg free".to_string(), modifier_type: "percentage".to_string(), amount: 0.15 },
                    PricingRuleModifier { id: "gluten_free".to_string(), name: "Gluten Free".to_string(), description: "GF ingredients".to_string(), modifier_type: "percentage".to_string(), amount: 0.10 },
                ],
            },
            PricingRuleOptionGroup {
                id: "delivery".to_string(),
                name: "Delivery Options".to_string(),
                is_multiple_choice: false,
                options: vec![
                    PricingRuleModifier { id: "pickup".to_string(), name: "Pickup".to_string(), description: "In-store pickup".to_string(), modifier_type: "flat".to_string(), amount: 0.0 },
                    PricingRuleModifier { id: "delivery_standard".to_string(), name: "Standard Delivery".to_string(), description: "Within 10 miles".to_string(), modifier_type: "flat".to_string(), amount: 15.0 },
                    PricingRuleModifier { id: "delivery_rush".to_string(), name: "Rush Delivery (Today)".to_string(), description: "Subject to availability".to_string(), modifier_type: "flat".to_string(), amount: 40.0 },
                ],
            },
        ]
    } else {
        vec![
            PricingRuleOptionGroup {
                id: "service_tier".to_string(),
                name: "Service Tier".to_string(),
                is_multiple_choice: false,
                options: vec![
                    PricingRuleModifier { id: "standard".to_string(), name: "Standard".to_string(), description: "Basic service".to_string(), modifier_type: "flat".to_string(), amount: 0.0 },
                    PricingRuleModifier { id: "premium".to_string(), name: "Premium".to_string(), description: "Includes priority & extra checks".to_string(), modifier_type: "flat".to_string(), amount: 100.0 },
                ],
            },
            PricingRuleOptionGroup {
                id: "timing".to_string(),
                name: "Timing".to_string(),
                is_multiple_choice: false,
                options: vec![
                    PricingRuleModifier { id: "flexible".to_string(), name: "Flexible".to_string(), description: "Whenever available".to_string(), modifier_type: "flat".to_string(), amount: 0.0 },
                    PricingRuleModifier { id: "urgent".to_string(), name: "Urgent/Emergency".to_string(), description: "Requires immediate attention".to_string(), modifier_type: "percentage".to_string(), amount: 0.50 },
                ],
            }
        ]
    };

    let rules = PricingRulesResponse {
        tenant_id,
        base_price,
        option_groups,
    };

    Json(rules)
}
