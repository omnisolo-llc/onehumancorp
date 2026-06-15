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
pub static POS_CATALOG_CACHE: OnceLock<HybridCache<Value>> = OnceLock::new();

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

async fn get_inventory_handler(
    State(_hub): State<Arc<Hub>>,
    Query(query): Query<PosQuery>,
) -> Json<Value> {
    let tenant_id = query.tenant_id.unwrap_or_else(|| "default".to_string());
    let cache_key = format!("pos_catalog:{}", tenant_id);
    let cache = POS_CATALOG_CACHE.get_or_init(|| HybridCache::new(crate::get_redis_client()));

    let catalog_task = async {
        if let Some((cached, is_stale)) = cache.get_with_swr(&cache_key).await {
            if !is_stale {
                return cached;
            }
            let tenant_id_bg = tenant_id.clone();
            let cache_key_bg = cache_key.clone();
            tokio::spawn(async move {
                let pool = crate::db::get_pool();
                let rows = sqlx::query("SELECT id, title, description, price_cents, currency FROM products WHERE tenant_id = $1")
                    .bind(&tenant_id_bg)
                    .fetch_all(&pool)
                    .await
                    .unwrap_or_default();

                let catalog: Vec<Value> = rows.into_iter().map(|row| {
                    json!({
                        "id": row.get::<String, _>("id"),
                        "name": row.get::<String, _>("title"),
                        "description": row.get::<Option<String>, _>("description"),
                        "price_cents": row.get::<i64, _>("price_cents"),
                        "currency": row.get::<String, _>("currency"),
                    })
                }).collect();
                let result = json!(catalog);
                if let Some(c) = POS_CATALOG_CACHE.get() {
                    c.set(&cache_key_bg, result, std::time::Duration::from_secs(3600)).await;
                }
            });
            return cached;
        }

        let pool = crate::db::get_pool();
        let rows = sqlx::query("SELECT id, title, description, price_cents, currency FROM products WHERE tenant_id = $1")
            .bind(&tenant_id)
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

        let catalog: Vec<Value> = rows.into_iter().map(|row| {
            json!({
                "id": row.get::<String, _>("id"),
                "name": row.get::<String, _>("title"),
                "description": row.get::<Option<String>, _>("description"),
                "price_cents": row.get::<i64, _>("price_cents"),
                "currency": row.get::<String, _>("currency"),
            })
        }).collect();

        let result = json!(catalog);
        cache.set(&cache_key, result.clone(), std::time::Duration::from_secs(3600)).await;
        result
    };

    let inventory_task = async {
        let pool = crate::db::get_pool();
        let rows = sqlx::query("SELECT id, inventory_count FROM products WHERE tenant_id = $1")
            .bind(&tenant_id)
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

        let mut inventory_map = std::collections::HashMap::new();
        for row in rows {
            let id: String = row.get("id");
            let count: i32 = row.get("inventory_count");
            inventory_map.insert(id, count);
        }
        inventory_map
    };

    let (mut catalog_val, inventory_map) = tokio::join!(catalog_task, inventory_task);

    if let Some(catalog_array) = catalog_val.as_array_mut() {
        for item in catalog_array.iter_mut() {
            if let Some(obj) = item.as_object_mut() {
                if let Some(id_val) = obj.get("id") {
                    if let Some(id_str) = id_val.as_str() {
                        let stock = inventory_map.get(id_str).copied().unwrap_or(0);
                        obj.insert("stock".to_string(), json!(stock));
                    }
                }
            }
        }
    }

    Json(json!({ "inventory": catalog_val }))
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
    async fn test_pos_inventory_caching_strategy() {
        let tenant_id = "test_tenant_inventory";
        let cache_key = format!("pos_catalog:{}", tenant_id);
        let cache = POS_CATALOG_CACHE.get_or_init(|| HybridCache::new(None));

        let initial_val = cache.get(&cache_key).await;
        assert!(initial_val.is_none(), "Catalog cache should be empty initially");

        cache.set(&cache_key, json!([{"id": "prod_1", "name": "Test Product", "price_cents": 1000, "currency": "USD"}]), std::time::Duration::from_secs(60)).await;

        let cached_val = cache.get(&cache_key).await;
        assert!(cached_val.is_some(), "Catalog cache should hit after set");
    }
}
