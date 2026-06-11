use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router, response::IntoResponse, http::StatusCode,
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
        .route("/sync", post(pos_sync_handler))
        .with_state(hub)
}

#[derive(serde::Deserialize)]
pub struct PosQuery {
    pub tenant_id: Option<String>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SyncEventItem {
    pub id: String,
    pub entity_type: String,
    pub entity_id: String,
    pub mutation_payload: Value,
    pub idempotency_key: String,
    pub timestamp: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct SyncEventRequest {
    pub events: Vec<SyncEventItem>,
}

#[derive(serde::Serialize)]
pub struct SyncEventResponse {
    pub success: bool,
    pub queued: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

async fn pos_sync_handler(
    State(_hub): State<Arc<Hub>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<SyncEventRequest>,
) -> impl IntoResponse {
    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(SyncEventResponse { success: false, queued: 0, error: Some("Unauthorized".to_string()) }),
        ).into_response();
    }

    let mut queued = 0;
    let pool = crate::db::get_pool();

    for event in payload.events {
        let mut db_tx = match pool.begin().await {
            Ok(tx) => tx,
            Err(e) => {
                tracing::error!("Failed to begin transaction: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(SyncEventResponse { success: false, queued: 0, error: Some("Database error".to_string()) }),
                ).into_response();
            }
        };

        // Set RLS tenant
        if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&tenant_id)
            .execute(&mut *db_tx)
            .await
        {
            tracing::error!("Failed to set tenant context: {}", e);
            let _ = db_tx.rollback().await;
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SyncEventResponse { success: false, queued: 0, error: Some("RLS configuration failed".to_string()) }),
            ).into_response();
        }

        let timestamp = chrono::DateTime::parse_from_rfc3339(&event.timestamp)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now());

        // Use ON CONFLICT DO NOTHING for safe idempotency
        let insert_res = sqlx::query(
            "INSERT INTO sync_events (id, tenant_id, entity_type, entity_id, mutation_payload, idempotency_key, timestamp)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             ON CONFLICT (tenant_id, idempotency_key) DO NOTHING
             RETURNING id"
        )
        .bind(&event.id)
        .bind(&tenant_id)
        .bind(&event.entity_type)
        .bind(&event.entity_id)
        .bind(&event.mutation_payload)
        .bind(&event.idempotency_key)
        .bind(&timestamp)
        .fetch_optional(&mut *db_tx)
        .await;

        match insert_res {
            Ok(Some(_row)) => {
                // Event was successfully inserted
                let job_id = uuid::Uuid::new_v4().to_string();
                let job_payload = json!({
                    "sync_event_id": event.id,
                    "entity_type": event.entity_type,
                    "entity_id": event.entity_id,
                    "mutation_payload": event.mutation_payload,
                }).to_string();

                let enqueue_res = sqlx::query(
                    "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload)
                     VALUES ($1, $2, 'process_sync_event', $3::jsonb)"
                )
                .bind(&job_id)
                .bind(&tenant_id)
                .bind(&job_payload)
                .execute(&mut *db_tx)
                .await;

                if let Err(e) = enqueue_res {
                    tracing::error!("Failed to enqueue job: {}", e);
                    let _ = db_tx.rollback().await;
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(SyncEventResponse { success: false, queued: 0, error: Some("Job enqueue failed".to_string()) }),
                    ).into_response();
                }

                queued += 1;

                if let Err(e) = db_tx.commit().await {
                    tracing::error!("Failed to commit transaction: {}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(SyncEventResponse { success: false, queued: 0, error: Some("Commit failed".to_string()) }),
                    ).into_response();
                }
            }
            Ok(None) => {
                // Event already exists, do nothing, just commit the RLS context changes
                if let Err(e) = db_tx.commit().await {
                    tracing::error!("Failed to commit transaction (idempotency skip): {}", e);
                }
            }
            Err(e) => {
                tracing::error!("Failed to insert sync_event: {}", e);
                let _ = db_tx.rollback().await;
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(SyncEventResponse { success: false, queued: 0, error: Some("Database insert failed".to_string()) }),
                ).into_response();
            }
        }
    }

    (
        StatusCode::OK,
        Json(SyncEventResponse { success: true, queued, error: None }),
    ).into_response()
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
    use axum::http::HeaderMap;

    #[tokio::test]
    async fn test_pos_sync_tenant_isolation() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = crate::db::get_pool();
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(Hub::new(tx, pool));

        let req = SyncEventRequest { events: vec![] };
        let headers = HeaderMap::new();

        let response = pos_sync_handler(axum::extract::State(hub), headers, Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_pos_sync_idempotency() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = crate::db::get_pool();
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        let hub = Arc::new(Hub::new(tx, pool.clone()));

        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-pos-sync', 'POS Sync Tenant') ON CONFLICT DO NOTHING")
            .execute(&pool).await.unwrap();

        let req = SyncEventRequest {
            events: vec![
                SyncEventItem {
                    id: "event-1".to_string(),
                    entity_type: "transaction".to_string(),
                    entity_id: "tx-1".to_string(),
                    mutation_payload: json!({"amount": 100}),
                    idempotency_key: "idem-key-1".to_string(),
                    timestamp: chrono::Utc::now().to_rfc3339(),
                }
            ],
        };

        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-pos-sync/agent/pos".parse().unwrap());

        // First request should queue the event
        let response = pos_sync_handler(axum::extract::State(hub.clone()), headers.clone(), Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sync_events WHERE tenant_id = 'tenant-pos-sync'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count.0, 1);

        // Second identical request should return success but NOT queue it again
        let req2 = SyncEventRequest {
            events: vec![
                SyncEventItem {
                    id: "event-2".to_string(),
                    entity_type: "transaction".to_string(),
                    entity_id: "tx-1".to_string(),
                    mutation_payload: json!({"amount": 100}),
                    idempotency_key: "idem-key-1".to_string(), // SAME KEY
                    timestamp: chrono::Utc::now().to_rfc3339(),
                }
            ],
        };

        let response2 = pos_sync_handler(axum::extract::State(hub), headers, Json(req2)).await.into_response();
        assert_eq!(response2.status(), StatusCode::OK);

        let count2: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sync_events WHERE tenant_id = 'tenant-pos-sync'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(count2.0, 1); // Still 1! Idempotency worked.
    }
}
