use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use crate::hub::Hub;
use sqlx::Row;

pub fn pos_routes<S>(hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/orders", get(get_orders_handler))
        .route("/inventory", get(get_inventory_handler))
        .with_state(hub)
}

#[derive(serde::Deserialize)]
pub struct PosQuery {
    pub tenant_id: Option<String>,
}

async fn get_orders_handler(
    State(_hub): State<Arc<Hub>>,
    Query(query): Query<PosQuery>,
) -> Json<Value> {
    let tenant_id = query.tenant_id.unwrap_or_else(|| "default".to_string());
    let pool = crate::db::get_pool();

    let rows = sqlx::query("SELECT id, total_amount, status, created_at FROM orders WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 20")
        .bind(&tenant_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    let orders: Vec<Value> = rows.into_iter().map(|row| {
        json!({
            "id": row.get::<String, _>("id"),
            "total_amount": row.get::<f64, _>("total_amount"),
            "status": row.get::<String, _>("status"),
            "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
        })
    }).collect();

    Json(json!({ "orders": orders }))
}

async fn get_inventory_handler(
    State(_hub): State<Arc<Hub>>,
    Query(query): Query<PosQuery>,
) -> Json<Value> {
    let tenant_id = query.tenant_id.unwrap_or_else(|| "default".to_string());
    let pool = crate::db::get_pool();

    let rows = sqlx::query("SELECT id, title, description, price_cents, currency, inventory_count FROM products WHERE tenant_id = $1")
        .bind(&tenant_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    let rules_rows = sqlx::query("SELECT rules_json FROM pricing_rules WHERE tenant_id = $1")
        .bind(&tenant_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    let mut inventory: Vec<Value> = Vec::new();
    for row in rows {
        let mut price_cents: i64 = row.get("price_cents");
        let stock: i32 = row.get("inventory_count");
        let product_id: String = row.get("id");

        // Simple application of the first matching rule for this product
        for rule_row in &rules_rows {
            if let Ok(rules_json) = rule_row.try_get::<sqlx::types::Json<serde_json::Value>, _>("rules_json") {
                if let Some(rule_product_id) = rules_json.get("product_id").and_then(|v| v.as_str()) {
                    if rule_product_id == product_id {
                        if let Some(min_price) = rules_json.get("min_price_cents").and_then(|v| v.as_i64()) {
                            let bounds = crate::pricing::dynamic::PricingBounds {
                                base_price: price_cents as f64 / 100.0,
                                min_price: min_price as f64 / 100.0,
                                max_price: (price_cents * 2) as f64 / 100.0,
                            };
                            let context = crate::pricing::dynamic::ContextSignals {
                                time_of_day: "afternoon".to_string(), // In reality we'd parse current time
                                weather: "sunny".to_string(),
                                inventory_velocity: if stock > 50 { "slow" } else if stock < 5 { "fast" } else { "normal" }.to_string(),
                                demand_level: "normal".to_string(),
                            };
                            let result = crate::pricing::dynamic::DynamicPricingEngine::calculate_price(&bounds, &context);
                            price_cents = (result.price * 100.0).round() as i64;
                            break;
                        }
                    }
                }
            }
        }

        inventory.push(json!({
            "id": product_id,
            "name": row.get::<String, _>("title"),
            "description": row.get::<Option<String>, _>("description"),
            "price_cents": price_cents,
            "currency": row.get::<String, _>("currency"),
            "stock": stock,
        }));
    }

    Json(json!({ "inventory": inventory }))
}
