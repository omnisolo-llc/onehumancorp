use axum::{
    extract::{Path, Query, State},
    routing::{get, post, put},
    Json, Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use crate::hub::Hub;
use sqlx::Row;
use crate::utils::cache::HybridCache;
use std::sync::OnceLock;
use ::server_ohc::app::pos_service_server::PosService;
use ::server_ohc::app::{
    EndTerminalSessionRequest, StartTerminalSessionRequest, SyncOfflineTransactionsRequest,
    UpdateTerminalSessionStatusRequest, PosOfflineTransaction,
};
use tonic::Request;
use crate::services::pos::service::MyPosService;
use crate::db::DbStore;

pub static POS_ORDERS_CACHE: OnceLock<HybridCache<Value>> = OnceLock::new();

pub fn pos_routes<S>(hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/orders", get(get_orders_handler))
        .route("/inventory", get(get_inventory_handler))
        .route("/sync", post(sync_offline_handler))
        .route("/sessions/start", post(start_session_handler))
        .route("/sessions/{id}/status", put(update_session_status_handler))
        .route("/sessions/{id}/end", post(end_session_handler))
        .with_state(hub)
}

#[derive(serde::Deserialize)]
pub struct PosQuery {
    pub tenant_id: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct SyncOfflineRequestPayload {
    pub mutations: Vec<serde_json::Value>,
}

async fn sync_offline_handler(
    State(hub): State<Arc<Hub>>,
    Query(query): Query<PosQuery>,
    Json(payload): Json<SyncOfflineRequestPayload>,
) -> Json<Value> {
    let tenant_id = query.tenant_id.unwrap_or_else(|| "default".to_string());

    let db = Arc::new(crate::db::DB {
        pool: hub.pool.clone(),
        store: DbStore::Postgres,
    });
    let service = MyPosService::new(db);

    let mut transactions = Vec::new();
    for mutation in payload.mutations {
        let tx = PosOfflineTransaction {
            id: mutation.get("transaction_id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            tenant_id: tenant_id.clone(),
            client_id: "api_client".to_string(),
            amount_cents: mutation.get("amount").and_then(|v| v.as_i64()).unwrap_or(0),
            currency: mutation.get("currency").and_then(|v| v.as_str()).unwrap_or("USD").to_string(),
            payload: mutation.to_string(),
            status: "PENDING".to_string(),
            created_at_unix: 0,
        };
        transactions.push(tx);
    }

    let req = SyncOfflineTransactionsRequest {
        tenant_id: tenant_id.clone(),
        client_id: "api_client".to_string(),
        transactions,
        session_id: None,
    };

    let mut request = Request::new(req);
    request.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "api".to_string(),
        org_id: tenant_id,
        agent_id: "".to_string(),
    });

    match service.sync_offline_transactions(request).await {
        Ok(res) => {
            let inner = res.into_inner();
            Json(json!({
                "success": inner.success,
                "synced_count": inner.synced_count,
                "failed_transaction_ids": inner.failed_transaction_ids
            }))
        },
        Err(e) => {
            Json(json!({
                "success": false,
                "error": e.message()
            }))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct StartSessionPayload {
    pub device_id: String,
}

async fn start_session_handler(
    State(hub): State<Arc<Hub>>,
    Query(query): Query<PosQuery>,
    Json(payload): Json<StartSessionPayload>,
) -> Json<Value> {
    let tenant_id = query.tenant_id.unwrap_or_else(|| "default".to_string());

    let db = Arc::new(crate::db::DB {
        pool: hub.pool.clone(),
        store: DbStore::Postgres,
    });
    let service = MyPosService::new(db);

    let req = StartTerminalSessionRequest {
        tenant_id: tenant_id.clone(),
        device_id: payload.device_id,
    };

    let mut request = Request::new(req);
    request.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "api".to_string(),
        org_id: tenant_id,
        agent_id: "".to_string(),
    });

    match service.start_terminal_session(request).await {
        Ok(res) => {
            let inner = res.into_inner();
            Json(json!({
                "success": inner.success,
                "session_id": inner.session_id,
                "error_message": inner.error_message
            }))
        },
        Err(e) => {
            Json(json!({
                "success": false,
                "error": e.message()
            }))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct UpdateSessionStatusPayload {
    pub status: String,
}

async fn update_session_status_handler(
    State(hub): State<Arc<Hub>>,
    Path(id): Path<String>,
    Query(query): Query<PosQuery>,
    Json(payload): Json<UpdateSessionStatusPayload>,
) -> Json<Value> {
    let tenant_id = query.tenant_id.unwrap_or_else(|| "default".to_string());

    let db = Arc::new(crate::db::DB {
        pool: hub.pool.clone(),
        store: DbStore::Postgres,
    });
    let service = MyPosService::new(db);

    let req = UpdateTerminalSessionStatusRequest {
        tenant_id: tenant_id.clone(),
        session_id: id,
        status: payload.status,
    };

    let mut request = Request::new(req);
    request.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "api".to_string(),
        org_id: tenant_id,
        agent_id: "".to_string(),
    });

    match service.update_terminal_session_status(request).await {
        Ok(res) => {
            let inner = res.into_inner();
            Json(json!({
                "success": inner.success,
                "error_message": inner.error_message
            }))
        },
        Err(e) => {
            Json(json!({
                "success": false,
                "error": e.message()
            }))
        }
    }
}

async fn end_session_handler(
    State(hub): State<Arc<Hub>>,
    Path(id): Path<String>,
    Query(query): Query<PosQuery>,
) -> Json<Value> {
    let tenant_id = query.tenant_id.unwrap_or_else(|| "default".to_string());

    let db = Arc::new(crate::db::DB {
        pool: hub.pool.clone(),
        store: DbStore::Postgres,
    });
    let service = MyPosService::new(db);

    let req = EndTerminalSessionRequest {
        tenant_id: tenant_id.clone(),
        session_id: id,
    };

    let mut request = Request::new(req);
    request.extensions_mut().insert(::server_auth::orchestration::AuthInfo {
        spiffe_id: "api".to_string(),
        org_id: tenant_id,
        agent_id: "".to_string(),
    });

    match service.end_terminal_session(request).await {
        Ok(res) => {
            let inner = res.into_inner();
            Json(json!({
                "success": inner.success,
                "error_message": inner.error_message
            }))
        },
        Err(e) => {
            Json(json!({
                "success": false,
                "error": e.message()
            }))
        }
    }
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
    let pool = crate::db::get_pool();

    let rows = sqlx::query("SELECT id, title, description, price_cents, currency, inventory_count FROM products WHERE tenant_id = $1")
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
