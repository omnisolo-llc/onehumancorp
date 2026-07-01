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
            if !order_id.is_empty() && !status.is_empty() {
                let _ = sqlx::query("UPDATE orders SET status = $1 WHERE id = $2 AND tenant_id = $3")
                    .bind(status)
                    .bind(order_id)
                    .bind(tenant_id)
                    .execute(&pool)
                    .await;
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
            if !item_id.is_empty() {
                let _ = sqlx::query("UPDATE products SET is_sold_out = $1 WHERE id = $2 AND tenant_id = $3")
                    .bind(is_sold_out)
                    .bind(item_id)
                    .bind(tenant_id)
                    .execute(&pool)
                    .await;
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
