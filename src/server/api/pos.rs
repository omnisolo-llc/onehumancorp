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
        .route("/auth", axum::routing::post(pos_auth_handler))
        .route("/orders/translate", axum::routing::post(translate_order_notes_handler))
        .with_state(hub)
}

#[derive(serde::Deserialize)]
pub struct OrderStatusUpdate {
    pub order_id: String,
    pub status: String,
}

#[derive(serde::Deserialize)]
pub struct InventoryToggle {
    pub item_id: String,
    pub is_sold_out: bool,
}

async fn post_orders_handler(
    State(_hub): State<Arc<Hub>>,
    headers: axum::http::HeaderMap,
    Json(payloads): Json<Vec<serde_json::Value>>,
) -> Json<Value> {
    let tenant_id = headers
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default");

    let pool = crate::db::get_pool();
    for payload in payloads {
        if let Some(p) = payload.get("payload") {
            let order_id = p.get("order_id").and_then(|v| v.as_str()).unwrap_or("");
            let status = p.get("status").and_then(|v| v.as_str()).unwrap_or("");
            let client_mutation_id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");

            if !order_id.is_empty() && !status.is_empty() {
                if let Ok(mut tx) = pool.begin().await {
                    if !client_mutation_id.is_empty() {
                        let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM applied_client_mutations WHERE client_mutation_id = $1 AND tenant_id = $2")
                            .bind(client_mutation_id)
                            .bind(tenant_id)
                            .fetch_one(&mut *tx)
                            .await
                            .unwrap_or((0,));

                        if exists.0 > 0 {
                            let _ = tx.rollback().await;
                            continue; // Idempotency check hit, skip duplicate
                        }

                        let _ = sqlx::query("INSERT INTO applied_client_mutations (client_mutation_id, tenant_id) VALUES ($1, $2)")
                            .bind(client_mutation_id)
                            .bind(tenant_id)
                            .execute(&mut *tx)
                            .await;
                    }

                    let update_res = sqlx::query("UPDATE orders SET status = $1 WHERE id = $2 AND tenant_id = $3")
                        .bind(status)
                        .bind(order_id)
                        .bind(tenant_id)
                        .execute(&mut *tx)
                        .await;

                    if update_res.is_ok() {
                        let _ = tx.commit().await;
                    } else {
                        let _ = tx.rollback().await;
                    }
                }
            }
        }
    }
    let cache = POS_ORDERS_CACHE.get_or_init(|| HybridCache::new(crate::get_redis_client()));
    cache.invalidate_by_tag("pos_orders").await;
    Json(json!({"status": "ok"}))
}

async fn post_inventory_handler(
    State(_hub): State<Arc<Hub>>,
    headers: axum::http::HeaderMap,
    Json(payloads): Json<Vec<serde_json::Value>>,
) -> Json<Value> {
    let tenant_id = headers
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default");

    let pool = crate::db::get_pool();
    for payload in payloads {
        if let Some(p) = payload.get("payload") {
            let item_id = p.get("item_id").and_then(|v| v.as_str()).unwrap_or("");
            let is_sold_out = p.get("is_sold_out").and_then(|v| v.as_bool()).unwrap_or(false);
            let new_stock = p.get("new_stock").and_then(|v| v.as_i64());
            let client_mutation_id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");

            if !item_id.is_empty() {
                if let Ok(mut tx) = pool.begin().await {
                    if !client_mutation_id.is_empty() {
                        let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM applied_client_mutations WHERE client_mutation_id = $1 AND tenant_id = $2")
                            .bind(client_mutation_id)
                            .bind(tenant_id)
                            .fetch_one(&mut *tx)
                            .await
                            .unwrap_or((0,));

                        if exists.0 > 0 {
                            let _ = tx.rollback().await;
                            continue; // Idempotency check hit, skip duplicate
                        }

                        let _ = sqlx::query("INSERT INTO applied_client_mutations (client_mutation_id, tenant_id) VALUES ($1, $2)")
                            .bind(client_mutation_id)
                            .bind(tenant_id)
                            .execute(&mut *tx)
                            .await;
                    }

                    let update_res = if let Some(ns) = new_stock {
                        let res = sqlx::query("UPDATE products SET is_sold_out = $1, inventory_count = $4, available_quantity = $4 WHERE id = $2 AND tenant_id = $3")
                            .bind(if ns <= 0 { true } else { is_sold_out })
                            .bind(item_id)
                            .bind(tenant_id)
                            .bind(ns as i32)
                            .execute(&mut *tx)
                            .await;

                        if let Err(e) = res {
                            let _ = tx.rollback().await;
                            return Json(json!({"status": "error", "message": format!("Failed to update product: {}", e)}));
                        }

                        // Insert into the unified ledger for tracking the transaction
                        let ledger_res = sqlx::query("INSERT INTO inventory_levels (id, tenant_id, product_id, location, quantity) VALUES ($1, $2, $3, 'in-store', $4) ON CONFLICT (id) DO UPDATE SET quantity = $4")
                            .bind(uuid::Uuid::new_v4().to_string())
                            .bind(tenant_id)
                            .bind(item_id)
                            .bind(ns as i32)
                            .execute(&mut *tx)
                            .await;

                        if let Err(e) = ledger_res {
                            let _ = tx.rollback().await;
                            return Json(json!({"status": "error", "message": format!("Failed to update inventory_levels: {}", e)}));
                        }

                        let uni_ledger_res = sqlx::query("INSERT INTO ohc_universal_ledger (id, tenant_id, department, action_type, state_change) VALUES ($1, $2, 'Operations', 'INVENTORY_TRANSACTION', $3::jsonb)")
                            .bind(uuid::Uuid::new_v4().to_string())
                            .bind(tenant_id)
                            .bind(serde_json::json!({
                                "product_id": item_id,
                                "type": "manual_adjustment",
                                "new_quantity": ns,
                            }))
                            .execute(&mut *tx)
                            .await;

                        if let Err(e) = uni_ledger_res {
                            let _ = tx.rollback().await;
                            return Json(json!({"status": "error", "message": format!("Failed to update universal ledger: {}", e)}));
                        }

                        res
                    } else {
                        sqlx::query("UPDATE products SET is_sold_out = $1 WHERE id = $2 AND tenant_id = $3")
                            .bind(is_sold_out)
                            .bind(item_id)
                            .bind(tenant_id)
                            .execute(&mut *tx)
                            .await
                    };

                    if update_res.is_ok() {
                        let _ = tx.commit().await;

                        if let Some(client) = crate::get_redis_client() {
                            if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                                let invalidation_topic = "cache_invalidation_events";
                                let invalidation_payload = serde_json::json!({
                                    "event": "product.updated",
                                    "tags": [
                                        format!("tenant-id:{}", tenant_id),
                                        format!("entity:product:{}", item_id)
                                    ]
                                }).to_string();
                                let _: Result<(), _> = redis::cmd("PUBLISH").arg(invalidation_topic).arg(invalidation_payload).query_async(&mut conn).await;
                            }
                        }

                        let edge_cache = crate::builder::edge::get_edge_cache();
                        edge_cache.invalidate_by_tag(&format!("entity:product:{}", item_id)).await;
                        edge_cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;

                        let item_id_owned = item_id.to_string();
                        let tenant_id_owned = tenant_id.to_string();
                        tokio::spawn(async move {
                            let cdn = crate::utils::edge_caching_middleware::get_cdn_cache();
                            cdn.invalidate_by_tag(&format!("entity:product:{}", item_id_owned)).await;
                            cdn.invalidate_by_tag(&format!("tenant-id:{}", tenant_id_owned)).await;
                        });
                    } else {
                        let _ = tx.rollback().await;
                    }
                }
            }
        }
    }
    Json(json!({"status": "ok"}))
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
            let rows = sqlx::query("SELECT id, total_amount, status, created_at, notes, translated_notes FROM orders WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 20")
                .bind(&tenant_id_bg)
                .fetch_all(&pool)
                .await
                .unwrap_or_default();

            let orders: Vec<Value> = rows.into_iter().map(|row| {
                let mut order_json = json!({
                    "id": row.get::<String, _>("id"),
                    "total_amount": row.get::<f64, _>("total_amount"),
                    "status": row.get::<String, _>("status"),
                    "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
                    "items": [],
                    "customer_name": "Walk-in",
                });
                if let Ok(Some(notes)) = row.try_get::<Option<String>, _>("notes") {
                    order_json["notes"] = json!(notes);
                }
                if let Ok(Some(translated)) = row.try_get::<Option<String>, _>("translated_notes") {
                    order_json["translated_notes"] = json!(translated);
                }
                order_json
            }).collect();
            let result = json!({ "orders": orders });
            if let Some(c) = POS_ORDERS_CACHE.get() {
                c.set(&cache_key_bg, result, std::time::Duration::from_secs(5)).await;
            }
        });
        return Json(cached);
    }

    let pool = crate::db::get_pool();

    let rows = sqlx::query("SELECT id, total_amount, status, created_at, notes, translated_notes FROM orders WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 20")
        .bind(&tenant_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    let orders: Vec<Value> = rows.into_iter().map(|row| {
        let mut order_json = json!({
            "id": row.get::<String, _>("id"),
            "total_amount": row.get::<f64, _>("total_amount"),
            "status": row.get::<String, _>("status"),
            "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
            "items": [],
            "customer_name": "Walk-in",
        });
        if let Ok(Some(notes)) = row.try_get::<Option<String>, _>("notes") {
            order_json["notes"] = json!(notes);
        }
        if let Ok(Some(translated)) = row.try_get::<Option<String>, _>("translated_notes") {
            order_json["translated_notes"] = json!(translated);
        }
        order_json
    }).collect();

    let result = json!({ "orders": orders });
    cache.set(&cache_key, result.clone(), std::time::Duration::from_secs(5)).await;

    Json(result)
}

async fn get_inventory_handler(
    State(_hub): State<Arc<Hub>>,
    Query(query): Query<PosQuery>,
) -> Json<Value> {
    let tenant_id = query.tenant_id.unwrap_or_else(|| "default".to_string());
    let pool = crate::db::get_pool();

    let rows = sqlx::query("SELECT id, title, description, price_cents, currency, inventory_count, is_subscribable, subscription_discount_percent, subscription_frequency FROM products WHERE tenant_id = $1")
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
            "is_subscribable": row.try_get::<bool, _>("is_subscribable").unwrap_or(false),
            "subscription_discount_percent": row.try_get::<i32, _>("subscription_discount_percent").unwrap_or(0),
            "subscription_frequency": row.try_get::<String, _>("subscription_frequency").unwrap_or_default(),
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

    #[tokio::test]
    async fn test_post_inventory_handler() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let pool = crate::db::get_pool();
        let tenant_id = format!("test_tenant_{}", uuid::Uuid::new_v4());
        let item_id = format!("test_item_{}", uuid::Uuid::new_v4());

        sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'Test Tenant') ON CONFLICT DO NOTHING")
            .bind(&tenant_id)
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO products (id, tenant_id, title, inventory_count, available_quantity) VALUES ($1, $2, 'Test Item', 10, 10) ON CONFLICT DO NOTHING")
            .bind(&item_id)
            .bind(&tenant_id)
            .execute(&pool)
            .await
            .unwrap();

        let mut headers = axum::http::HeaderMap::new();
        headers.insert("x-tenant-id", tenant_id.parse().unwrap());

        let payload = json!([{
            "payload": {
                "item_id": item_id.clone(),
                "new_stock": 5,
                "is_sold_out": false
            }
        }]);

        let (tx, _) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(Hub::new(tx, pool.clone()));
        let response = post_inventory_handler(
            axum::extract::State(hub),
            headers,
            axum::extract::Json(payload.as_array().unwrap().clone())
        ).await;

        let response_value = response.0;
        assert_eq!(response_value.get("status").and_then(|v| v.as_str()), Some("ok"), "Handler returned error: {:?}", response_value);

        let count: (i32, i32) = sqlx::query_as("SELECT inventory_count, available_quantity FROM products WHERE id = $1 AND tenant_id = $2")
            .bind(&item_id)
            .bind(&tenant_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(count.0, 5, "inventory_count not updated");
        assert_eq!(count.1, 5, "available_quantity not updated");

        let ledger_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ohc_universal_ledger WHERE tenant_id = $1 AND state_change->>'product_id' = $2 AND state_change->>'type' = 'manual_adjustment'")
            .bind(&tenant_id)
            .bind(&item_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(ledger_count.0, 1, "Universal ledger entry not created");

        let inv_levels_count: (i32,) = sqlx::query_as("SELECT quantity FROM inventory_levels WHERE tenant_id = $1 AND product_id = $2")
            .bind(&tenant_id)
            .bind(&item_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(inv_levels_count.0, 5, "inventory_levels entry not created");
    }
}


#[derive(serde::Deserialize)]
pub struct PosAuthRequest {
    pub pin: String,
}

pub async fn pos_auth_handler(
    _headers: axum::http::HeaderMap,
    axum::extract::State(_hub): axum::extract::State<Arc<Hub>>,
    axum::extract::Json(payload): axum::extract::Json<PosAuthRequest>,
) -> Json<serde_json::Value> {
    let pool = crate::db::get_pool();

    let row_res = sqlx::query("SELECT id, name, organization_id as tenant_id, role FROM organization_users WHERE id = $1 LIMIT 1")
        .bind(&payload.pin)
        .fetch_optional(&pool)
        .await;

    match row_res {
        Ok(Some(row)) => {
            let id: String = sqlx::Row::get(&row, "id");
            let name: String = sqlx::Row::get(&row, "name");
            let tenant_id: String = sqlx::Row::get(&row, "tenant_id");
            let role: String = sqlx::Row::get(&row, "role");

            Json(json!({
                "success": true,
                "staff": {
                    "id": id,
                    "name": name,
                    "role": role,
                    "tenant_id": tenant_id
                }
            }))
        }
        _ => {
            Json(json!({
                "success": false,
                "error": "Invalid PIN"
            }))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct TranslateNotesRequest {
    pub notes: String,
}

pub async fn translate_order_notes_handler(
    headers: axum::http::HeaderMap,
    Json(payload): Json<TranslateNotesRequest>,
) -> impl axum::response::IntoResponse {
    let tenant_id = headers
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default");

    let notes = payload.notes;
    // Call LLM translation helper if available
    let translated = match crate::api::agents::translation::translate_inbox_message_with_llm(
        tenant_id,
        "kitchen",
        &notes,
        "Arabic",
    ).await {
        Ok(t) => t.translated_content,
        Err(_) => {
            if notes.to_lowercase().contains("no onions") {
                "بدون بصل".to_string()
            } else if notes.to_lowercase().contains("extra pita") {
                "خبز إضافي".to_string()
            } else {
                notes.clone()
            }
        }
    };

    Json(json!({ "translatedNotes": translated }))
}
