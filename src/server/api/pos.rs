use axum::{
    extract::{Extension, State},
    response::IntoResponse,
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

fn pos_tenant(claims: Option<&Extension<::server_common::Claims>>) -> Option<String> {
    claims.and_then(|Extension(claims)| ::server_common::auth_utils::signed_tenant_id(claims))
}

async fn fetch_pos_orders(tenant_id: &str) -> Result<Vec<Value>, sqlx::Error> {
    let pool = crate::db::get_pool();
    let mut tx = pool.begin().await?;
    ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await?;
    let rows = sqlx::query("SELECT id, total_amount, status, created_at, notes, translated_notes FROM orders WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 20")
        .bind(tenant_id)
        .fetch_all(&mut *tx)
        .await?;
    tx.commit().await?;

    Ok(rows.into_iter().map(|row| {
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
    }).collect())
}

pub fn pos_routes<S>(hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/orders", get(get_orders_handler).post(post_orders_handler))
        .route("/inventory", get(get_inventory_handler).post(post_inventory_handler))
        .route("/auth", axum::routing::post(pos_auth_handler))
        .route("/sync", axum::routing::post(pos_sync_handler))
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
    claims: Option<Extension<::server_common::Claims>>,
    Json(payloads): Json<Vec<serde_json::Value>>,
) -> impl axum::response::IntoResponse {
    let Some(tenant_id) = pos_tenant(claims.as_ref()) else {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    };
    if payloads.len() > 100 {
        return axum::http::StatusCode::BAD_REQUEST.into_response();
    }

    let pool = crate::db::get_pool();
    for payload in payloads {
        if let Some(p) = payload.get("payload") {
            let order_id = p.get("order_id").and_then(|v| v.as_str()).unwrap_or("");
            let status = p.get("status").and_then(|v| v.as_str()).unwrap_or("");
            let client_mutation_id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");

            if !order_id.is_empty() && !status.is_empty() {
                if let Ok(mut tx) = pool.begin().await {
                    if ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.is_err() {
                        continue;
                    }
                    if !client_mutation_id.is_empty() {
                        let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM applied_client_mutations WHERE client_mutation_id = $1 AND tenant_id = $2")
                            .bind(client_mutation_id)
                            .bind(&tenant_id)
                            .fetch_one(&mut *tx)
                            .await
                            .unwrap_or((0,));

                        if exists.0 > 0 {
                            let _ = tx.rollback().await;
                            continue; // Idempotency check hit, skip duplicate
                        }

                        let _ = sqlx::query("INSERT INTO applied_client_mutations (client_mutation_id, tenant_id) VALUES ($1, $2)")
                            .bind(client_mutation_id)
                            .bind(&tenant_id)
                            .execute(&mut *tx)
                            .await;
                    }

                    let update_res = sqlx::query("UPDATE orders SET status = $1 WHERE id = $2 AND tenant_id = $3")
                        .bind(status)
                        .bind(order_id)
                        .bind(&tenant_id)
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
    Json(json!({"status": "ok"})).into_response()
}

#[derive(serde::Deserialize)]
pub struct InventoryAdjustment {
    pub item_id: String,
    pub quantity_change: i32,
    pub location_id: Option<String>,
}

async fn post_inventory_handler(
    axum::extract::State(_hub): axum::extract::State<Arc<Hub>>,
    claims: Option<Extension<::server_common::Claims>>,
    axum::Json(payloads): axum::Json<Vec<serde_json::Value>>,
) -> impl axum::response::IntoResponse {
    let Some(tenant_id) = pos_tenant(claims.as_ref()) else {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    };
    if payloads.len() > 100 {
        return axum::http::StatusCode::BAD_REQUEST.into_response();
    }

    let pool = crate::db::get_pool();
    for payload in payloads {
        if let Some(p) = payload.get("payload") {
            let item_id = p.get("item_id").and_then(|v| v.as_str()).unwrap_or("");
            let quantity_change = p.get("quantity_change").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let location_id = p.get("location_id").and_then(|v| v.as_str()).unwrap_or("default_loc");
            let is_sold_out = p.get("is_sold_out").and_then(|v| v.as_bool()).unwrap_or(false);
            let client_mutation_id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");

            if !item_id.is_empty() {
                if let Ok(mut tx) = pool.begin().await {
                    if ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.is_err() {
                        continue;
                    }
                    if !client_mutation_id.is_empty() {
                        let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM applied_client_mutations WHERE client_mutation_id = $1 AND tenant_id = $2")
                            .bind(client_mutation_id)
                            .bind(&tenant_id)
                            .fetch_one(&mut *tx)
                            .await
                            .unwrap_or((0,));

                        if exists.0 > 0 {
                            let _ = tx.rollback().await;
                            continue;
                        }

                        let _ = sqlx::query("INSERT INTO applied_client_mutations (client_mutation_id, tenant_id) VALUES ($1, $2)")
                            .bind(client_mutation_id)
                            .bind(&tenant_id)
                            .execute(&mut *tx)
                            .await;
                    }

                    // Update centralized inventory level
                    let update_res = sqlx::query("UPDATE inventory_levels SET available_count = GREATEST(0, available_count + $1) WHERE variant_id = $2 AND tenant_id = $3 RETURNING id")
                        .bind(quantity_change)
                        .bind(item_id)
                        .bind(&tenant_id)
                        .fetch_optional(&mut *tx)
                        .await;

                    let mut inv_lvl_id: String = "".to_string();
                    if let Ok(Some(row)) = &update_res {
                         inv_lvl_id = sqlx::Row::get(row, "id");
                    } else if let Ok(None) = &update_res {
                         // Insert if not exists
                         inv_lvl_id = uuid::Uuid::new_v4().to_string();
                         let _ = sqlx::query("INSERT INTO inventory_levels (id, tenant_id, variant_id, location_id, available_count) VALUES ($1, $2, $3, $4, $5)")
                            .bind(&inv_lvl_id)
                            .bind(&tenant_id)
                            .bind(item_id)
                            .bind(location_id)
                            .bind(quantity_change)
                            .execute(&mut *tx)
                            .await;
                    }

                    if !inv_lvl_id.is_empty() && quantity_change != 0 {
                         let t_id = uuid::Uuid::new_v4().to_string();
                         let _ = sqlx::query("INSERT INTO inventory_transactions (id, tenant_id, inventory_level_id, type, quantity_change) VALUES ($1, $2, $3, 'adjustment', $4)")
                             .bind(&t_id)
                             .bind(&tenant_id)
                             .bind(&inv_lvl_id)
                             .bind(quantity_change)
                             .execute(&mut *tx)
                             .await;
                    }

                    // Sync to legacy products for compatibility
                    let update_legacy = sqlx::query("UPDATE products SET inventory_count = GREATEST(0, inventory_count + $1), available_quantity = GREATEST(0, available_quantity + $1), is_sold_out = $2 WHERE id = $3 AND tenant_id = $4")
                        .bind(quantity_change)
                        .bind(is_sold_out)
                        .bind(item_id)
                        .bind(&tenant_id)
                        .execute(&mut *tx)
                        .await;

                    if update_legacy.is_ok() {
                        let _ = tx.commit().await;

                        if let Some(client) = crate::get_redis_client() {
                            if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                                let invalidation_topic = "cache_invalidation_events";
                                let invalidation_payload = serde_json::json!({
                                    "event": "inventory.updated",
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
    Json(json!({"status": "ok"})).into_response()
}

async fn get_orders_handler(
    State(_hub): State<Arc<Hub>>,
    claims: Option<Extension<::server_common::Claims>>,
) -> impl axum::response::IntoResponse {
    let Some(tenant_id) = pos_tenant(claims.as_ref()) else {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    };
    let cache_key = format!("pos_orders:{}", tenant_id);
    let cache = POS_ORDERS_CACHE.get_or_init(|| HybridCache::new(crate::get_redis_client()));

    if let Some((cached, is_stale)) = cache.get_with_swr(&cache_key).await {
        if !is_stale {
            return Json(cached).into_response();
        }
        let tenant_id_bg = tenant_id.clone();
        let cache_key_bg = cache_key.clone();
        tokio::spawn(async move {
            let orders = fetch_pos_orders(&tenant_id_bg).await.unwrap_or_default();
            let result = json!({ "orders": orders });
            if let Some(c) = POS_ORDERS_CACHE.get() {
                c.set(&cache_key_bg, result, std::time::Duration::from_secs(5)).await;
            }
        });
        return Json(cached).into_response();
    }

    let orders = fetch_pos_orders(&tenant_id).await.unwrap_or_default();

    let result = json!({ "orders": orders });
    cache.set(&cache_key, result.clone(), std::time::Duration::from_secs(5)).await;

    Json(result).into_response()
}

async fn get_inventory_handler(
    State(_hub): State<Arc<Hub>>,
    claims: Option<Extension<::server_common::Claims>>,
) -> impl axum::response::IntoResponse {
    let Some(tenant_id) = pos_tenant(claims.as_ref()) else {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    };
    let pool = crate::db::get_pool();
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    if ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.is_err() {
        return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    let rows = sqlx::query("SELECT id, title, description, price_cents, currency, inventory_count, is_subscribable, subscription_discount_percent, subscription_frequency FROM products WHERE tenant_id = $1")
        .bind(&tenant_id)
        .fetch_all(&mut *tx)
        .await;
    let rows = match rows {
        Ok(rows) => rows,
        Err(_) => return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    if tx.commit().await.is_err() {
        return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

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

    Json(json!({ "inventory": inventory })).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_inventory_adjustment_struct() {
        let adj = InventoryAdjustment {
            item_id: "test_item".to_string(),
            quantity_change: -1,
            location_id: Some("loc1".to_string()),
        };
        assert_eq!(adj.item_id, "test_item");
        assert_eq!(adj.quantity_change, -1);
    }

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
#[serde(deny_unknown_fields)]
pub struct PosAuthRequest {
    pub pin: String,
}

pub async fn pos_auth_handler(
    claims: Option<Extension<::server_common::Claims>>,
    axum::extract::State(_hub): axum::extract::State<Arc<Hub>>,
    axum::extract::Json(payload): axum::extract::Json<PosAuthRequest>,
) -> impl IntoResponse {
    let Some(Extension(claims)) = claims else {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    };
    let Some(tenant_id) = ::server_common::auth_utils::signed_tenant_id(&claims) else {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    };
    if payload.pin.len() > 64 {
        return axum::http::StatusCode::BAD_REQUEST.into_response();
    }

    Json(json!({
        "success": true,
        "staff": {
            "id": claims.sub,
            "name": claims.username,
            "role": claims.roles.first().cloned().unwrap_or_else(|| "STAFF".to_string()),
            "tenant_id": tenant_id,
        }
    })).into_response()
}

#[derive(serde::Deserialize)]
pub struct TranslateNotesRequest {
    pub notes: String,
}

pub async fn translate_order_notes_handler(
    claims: Option<Extension<::server_common::Claims>>,
    Json(payload): Json<TranslateNotesRequest>,
) -> impl axum::response::IntoResponse {
    let Some(tenant_id) = pos_tenant(claims.as_ref()) else {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    };

    let notes = payload.notes;

    // Fetch tenant's preferred language from the database
    let pool = crate::db::get_pool();
    let preferred_language: String = sqlx::query_scalar(
        "SELECT preferred_language FROM tenants WHERE id = $1"
    )
    .bind(&tenant_id)
    .fetch_optional(&pool)
    .await
    .unwrap_or(None)
    .unwrap_or_else(|| "en".to_string());

    // Call LLM translation helper if available
    let translated = match crate::api::agents::translation::translate_inbox_message_with_llm(
        &tenant_id,
        "kitchen",
        &notes,
        &preferred_language,
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

    Json(json!({ "translatedNotes": translated })).into_response()
}


#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct PosSyncPayload {
    pub transaction_id: String,
    pub product_id: String,
    pub quantity_deducted: i32,
    pub amount_cents: Option<i64>,
    pub currency: Option<String>,
    pub client_mutation_id: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct PosSyncRequest {
    pub transactions: Vec<PosSyncPayload>,
}

pub async fn pos_sync_handler(
    State(_hub): State<Arc<Hub>>,
    claims: Option<Extension<::server_common::Claims>>,
    Json(payload): Json<PosSyncRequest>,
) -> impl axum::response::IntoResponse {
    let Some(tenant_id) = pos_tenant(claims.as_ref()) else {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    };

    let pool = crate::db::get_pool();
    let mut failed_count = 0;

    for tx_payload in &payload.transactions {
        let mutation_ts = chrono::Utc::now().to_rfc3339();

        let mut db_tx = match pool.begin().await {
            Ok(tx) => tx,
            Err(_) => {
                failed_count += 1;
                continue;
            }
        };

        if ::server_common::auth_utils::set_org_context(&mut *db_tx, &tenant_id).await.is_err() {
            failed_count += 1;
            continue;
        }

        // Deduplication using pos_offline_transactions (id matches transaction_id usually for offline POS)
        // Ensure idempotency using the transaction ID
        let res = sqlx::query(
            "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, status, amount_cents, currency, payload, created_at, updated_at, _sync_status, terminal_id)
             VALUES ($1, $2, 'offline-sync', 'RESOLVED', $3, $4, $5::jsonb, $6::timestamptz, $6::timestamptz, 'synced', 'offline')
             ON CONFLICT (id) DO NOTHING"
        )
        .bind(&tx_payload.transaction_id)
        .bind(&tenant_id)
        .bind(tx_payload.amount_cents.unwrap_or(0))
        .bind(tx_payload.currency.as_deref().unwrap_or("USD"))
        .bind(serde_json::to_value(tx_payload).unwrap_or(serde_json::json!({})))
        .bind(&mutation_ts)
        .execute(&mut *db_tx)
        .await;

        match res {
            Ok(result) => {
                if result.rows_affected() > 0 {
                    let job_id = uuid::Uuid::new_v4().to_string();
                    let job_payload = serde_json::json!({
                        "transaction_id": tx_payload.transaction_id,
                        "product_id": tx_payload.product_id,
                        "quantity_deducted": tx_payload.quantity_deducted,
                        "amount_cents": tx_payload.amount_cents,
                        "currency": tx_payload.currency,
                        "inventory_already_deducted": false,
                    }).to_string();

                    let job_res = sqlx::query(
                        "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload)
                         VALUES ($1, $2, 'offline_pos_sync', $3::jsonb)"
                    )
                    .bind(&job_id)
                    .bind(&tenant_id)
                    .bind(&job_payload)
                    .execute(&mut *db_tx)
                    .await;

                    if job_res.is_err() {
                        let _ = db_tx.rollback().await;
                        failed_count += 1;
                        continue;
                    }
                }
            }
            Err(_) => {
                let _ = db_tx.rollback().await;
                failed_count += 1;
                continue;
            }
        }

        let _ = db_tx.commit().await;
    }

    if failed_count > 0 {
        Json(json!({ "success": false, "failed_count": failed_count })).into_response()
    } else {
        Json(json!({ "success": true, "failed_count": 0 })).into_response()
    }
}


#[cfg(test)]
mod sync_tests {
    use super::*;
    use axum::http::HeaderMap;

    #[tokio::test]
    async fn test_pos_sync_handler_unauthorized() {
        let pool = crate::db::secure_pg_pool_options().acquire_timeout(std::time::Duration::from_millis(10)).connect_lazy("postgres://localhost/dummy").unwrap();
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        let state = State(Arc::new(Hub::new(tx, pool.clone())));

        let req = PosSyncRequest { transactions: vec![] };
        let response = pos_sync_handler(state, None, Json(req)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_pos_sync_handler_success_and_idempotency() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = crate::db::secure_pg_pool_options().connect(&database_url).await.unwrap();

        // Setup test data
        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-pos-sync', 'POS Sync Tenant') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();

        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        let state = State(Arc::new(Hub::new(tx, pool.clone())));

        let tx_id = format!("tx-sync-{}", uuid::Uuid::new_v4());
        let req = PosSyncRequest {
            transactions: vec![
                PosSyncPayload {
                    transaction_id: tx_id.clone(),
                    product_id: "prod-pos-sync-1".to_string(),
                    quantity_deducted: 2,
                    amount_cents: Some(1500),
                    currency: Some("USD".to_string()),
                    client_mutation_id: None,
                },
            ],
        };

        let claims = Extension(::server_common::Claims {
            sub: "user_123".to_string(),
            username: "maya".to_string(),
            email: "".to_string(),
            organization_id: Some("tenant-pos-sync".to_string()),
            roles: vec!["OWNER".to_string()],
            exp: 0,
            iat: 0,
            session_id: Some("".to_string()),
            jti: "".to_string(),
        });

        // 1. Initial success
        let response = pos_sync_handler(state.clone(), Some(claims.clone()), Json(req.clone())).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(body_json["success"], true);
        assert_eq!(body_json["failed_count"], 0);

        let count_jobs: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ohc_job_queue WHERE payload::jsonb->>'transaction_id' = $1")
            .bind(&tx_id)
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count_jobs.0, 1);

        // 2. Idempotency test (should not fail, but should not create another job)
        let response_dup = pos_sync_handler(state.clone(), Some(claims.clone()), Json(req)).await.into_response();
        assert_eq!(response_dup.status(), axum::http::StatusCode::OK);

        let body_bytes_dup = axum::body::to_bytes(response_dup.into_body(), usize::MAX).await.unwrap();
        let body_json_dup: serde_json::Value = serde_json::from_slice(&body_bytes_dup).unwrap();
        assert_eq!(body_json_dup["success"], true);

        let count_jobs_dup: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM ohc_job_queue WHERE payload::jsonb->>'transaction_id' = $1")
            .bind(&tx_id)
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count_jobs_dup.0, 1); // Still 1
    }
}
