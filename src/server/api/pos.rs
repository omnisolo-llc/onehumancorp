use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use crate::hub::Hub;
use sqlx::Row;
use crate::utils::cache::HybridCache;
use std::sync::OnceLock;

pub static POS_ORDERS_CACHE: OnceLock<HybridCache<Value>> = OnceLock::new();

pub fn pos_routes<S>(hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/orders", get(get_orders_handler).post(post_orders_handler))
        .route("/inventory", get(get_inventory_handler).post(post_inventory_handler))
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
    let cache_key = format!("pos_orders:{}", tenant_id);
    let cache = POS_ORDERS_CACHE.get_or_init(|| HybridCache::new(crate::get_redis_client()));

    if let Some((cached, is_stale)) = cache.get_with_swr(&cache_key).await {
        if !is_stale {
            return Json(cached);
        }
        let tenant_id_bg = tenant_id.clone();
        let cache_key_bg = cache_key.clone();
        tokio::spawn(async move {
            let pool = crate::db::get_pool();
            let rows = sqlx::query("SELECT id, total_amount, status, created_at FROM orders WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 20")
                .bind(&tenant_id_bg)
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
            let result = json!({ "orders": orders });
            if let Some(c) = POS_ORDERS_CACHE.get() {
                c.set(&cache_key_bg, result, std::time::Duration::from_secs(5)).await;
            }
        });
        return Json(cached);
    }

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

    let result = json!({ "orders": orders });
    cache.set(&cache_key, result.clone(), std::time::Duration::from_secs(5)).await;

    Json(result)
}


#[derive(serde::Deserialize)]
pub struct MutationPayload {
    pub item_id: Option<String>,
    pub is_sold_out: Option<bool>,
    pub order_id: Option<String>,
    pub status: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct OfflineMutation {
    pub r#type: String,
    pub payload: MutationPayload,
    pub timestamp: String,
}


async fn post_inventory_handler(
    State(hub): State<Arc<Hub>>,
    headers: axum::http::HeaderMap,
    axum::Json(mutations): axum::Json<Vec<OfflineMutation>>,
) -> Json<Value> {
    let pool = crate::db::get_pool();
    let req_tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default");

    let mut failed = 0;

    for mutation in mutations {
        if mutation.r#type == "TOGGLE_SOLD_OUT" {
            if let (Some(item_id), Some(is_sold_out)) = (mutation.payload.item_id, mutation.payload.is_sold_out) {
                // Determine tenant_id
                let row = sqlx::query("SELECT tenant_id FROM products WHERE id = $1")
                    .bind(&item_id)
                    .fetch_optional(&pool)
                    .await
                    .unwrap_or(None);

                if let Some(row) = row {
                    let tenant_id: String = row.get("tenant_id");
                    if tenant_id != req_tenant_id && req_tenant_id != "default" { failed += 1; continue; }
                    if tenant_id != req_tenant_id && req_tenant_id != "default" { failed += 1; continue; }

                    let res = sqlx::query("UPDATE products SET is_sold_out = $1 WHERE id = $2")
                        .bind(is_sold_out)
                        .bind(&item_id)
                        .execute(&pool)
                        .await;

                    if res.is_err() {
                        failed += 1;
                    } else {
                        // Notify operations agent
                        // let event = crate::orchestration::TeammateMeshEvent {
let event = crate::orchestration::departments::types::DepartmentEvent {
                                 id: uuid::Uuid::new_v4().to_string(),
                                 tenant_id: tenant_id.clone(),
                                 event_type: "tenant.inventory.updated".to_string(),
                                 payload: json!({
                                     "product_id": item_id,
                                     "is_sold_out": is_sold_out
                                 })
                             };
                             // Cannot easily dispatch here without refactoring pos_routes to take orchestrator.

                    }
                } else {
                    failed += 1;
                }
            } else {
                failed += 1;
            }
        }
    }

    if failed > 0 {
        Json(json!({ "success": false, "failed_count": failed }))
    } else {
        Json(json!({ "success": true, "failed_count": 0 }))
    }
}


async fn post_orders_handler(
    State(hub): State<Arc<Hub>>,
    headers: axum::http::HeaderMap,
    axum::Json(mutations): axum::Json<Vec<OfflineMutation>>,
) -> Json<Value> {
    let pool = crate::db::get_pool();
    let req_tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default");

    let mut failed = 0;

    for mutation in mutations {
        if mutation.r#type == "UPDATE_ORDER_STATUS" {
            if let (Some(order_id), Some(status)) = (mutation.payload.order_id, mutation.payload.status) {
                let row = sqlx::query("SELECT tenant_id, total_amount FROM orders WHERE id = $1")
                    .bind(&order_id)
                    .fetch_optional(&pool)
                    .await
                    .unwrap_or(None);

                if let Some(row) = row {
                    let tenant_id: String = row.get("tenant_id");
                    if tenant_id != req_tenant_id && req_tenant_id != "default" { failed += 1; continue; }
                    if tenant_id != req_tenant_id && req_tenant_id != "default" { failed += 1; continue; }

                    let res = sqlx::query("UPDATE orders SET status = $1 WHERE id = $2")
                        .bind(&status)
                        .bind(&order_id)
                        .execute(&pool)
                        .await;

                    if res.is_err() {
                        failed += 1;
                    } else {
                        if status == "Ready" {
                             let event = crate::orchestration::departments::types::DepartmentEvent {
                                 id: uuid::Uuid::new_v4().to_string(),
                                 tenant_id: tenant_id.clone(),
                                 event_type: "tenant.order.fulfillment_ready".to_string(),
                                 payload: json!({
                                     "order_id": order_id,
                                     "status": "Ready"
                                 })
                             };
                             // Cannot easily dispatch here without refactoring pos_routes to take orchestrator.
                        }
                    }
                } else {
                    failed += 1;
                }
            } else {
                failed += 1;
            }
        }
    }

    if failed > 0 {
        Json(json!({ "success": false, "failed_count": failed }))
    } else {
        Json(json!({ "success": true, "failed_count": 0 }))
    }
}


async fn get_inventory_handler(
    State(_hub): State<Arc<Hub>>,
    Query(query): Query<PosQuery>,
) -> Json<Value> {
    let tenant_id = query.tenant_id.unwrap_or_else(|| "default".to_string());
    let pool = crate::db::get_pool();

    let rows = sqlx::query("SELECT id, title, description, price_cents, currency, inventory_count, is_sold_out FROM products WHERE tenant_id = $1")
        .bind(&tenant_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    let inventory: Vec<Value> = rows.into_iter().map(|row| {
        json!({
            "id": row.get::<String, _>("id"),
            "name": row.get::<String, _>("title"),
            "description": row.get::<Option<String>, _>("description"),
            "price_cents": row.get::<i64, _>("price_cents"),
            "currency": row.get::<String, _>("currency"),
            "stock": row.get::<i32, _>("inventory_count"),
            "is_sold_out": row.get::<Option<bool>, _>("is_sold_out").unwrap_or(false),
        })
    }).collect();

    Json(json!({ "inventory": inventory }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pos_orders_cache_initialization() {
        let tenant_id = "test_tenant";
        let cache_key = format!("pos_orders:{}", tenant_id);
        let cache = POS_ORDERS_CACHE.get_or_init(|| HybridCache::new(None));

        let initial_val = cache.get(&cache_key).await;
        assert!(initial_val.is_none(), "Cache should be empty initially");

        cache.set(&cache_key, json!({"orders": []}), std::time::Duration::from_secs(60)).await;

        let cached_val = cache.get(&cache_key).await;
        assert!(cached_val.is_some(), "Cache should hit after set");
    }
}
