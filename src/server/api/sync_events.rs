use axum::{
    extract::State,
    response::IntoResponse,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SyncEvent {
    pub id: String,
    pub action_type: String,
    pub payload: String,
    pub timestamp_ms: i64,
}

#[derive(Debug, Deserialize)]
pub struct SyncBatchRequest {
    pub tenant_id: String,
    pub batch_id: String,
    pub events: Vec<SyncEvent>,
}

#[derive(Debug, Serialize)]
pub struct SyncBatchResponse {
    pub success: bool,
    pub applied_count: i32,
    pub conflict_count: i32,
    pub conflict_event_ids: Vec<String>,
}

pub async fn sync_events_handler(
    State(pool): State<PgPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<SyncBatchRequest>,
) -> impl IntoResponse {
    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() || tenant_id != payload.tenant_id {
        return (
            StatusCode::UNAUTHORIZED,
            Json(SyncBatchResponse {
                success: false,
                applied_count: 0,
                conflict_count: 0,
                conflict_event_ids: vec![],
            })
        ).into_response();
    }

    let mut applied_count = 0;
    let mut conflict_count = 0;
    let mut conflict_event_ids = Vec::new();

    for event in payload.events {
        let mut tx = match pool.begin().await {
            Ok(tx) => tx,
            Err(e) => {
                tracing::error!("Failed to begin transaction for sync event {}: {}", event.id, e);
                continue;
            }
        };

        let _ = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;

        let res = sqlx::query(
            "INSERT INTO sync_events (id, tenant_id, batch_id, action_type, payload)
             VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&event.id)
        .bind(&tenant_id)
        .bind(&payload.batch_id)
        .bind(&event.action_type)
        .bind(&event.payload)
        .execute(&mut *tx)
        .await;

        if res.is_err() {
            let _ = tx.rollback().await;
            continue;
        }

        let job_id = uuid::Uuid::new_v4().to_string();
        let job_payload = serde_json::json!({
            "sync_event_id": event.id,
            "action_type": event.action_type,
            "payload": event.payload,
            "timestamp_ms": event.timestamp_ms
        }).to_string();

        let queue_res = sqlx::query(
            "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload)
             VALUES ($1, $2, 'mutation_sync', $3::jsonb)"
        )
        .bind(&job_id)
        .bind(&tenant_id)
        .bind(&job_payload)
        .execute(&mut *tx)
        .await;

        if queue_res.is_ok() {
            let _ = tx.commit().await;
            applied_count += 1;
        } else {
            let _ = tx.rollback().await;
            conflict_count += 1;
            conflict_event_ids.push(event.id.clone());
        }
    }

    (
        StatusCode::OK,
        Json(SyncBatchResponse {
            success: true,
            applied_count,
            conflict_count,
            conflict_event_ids,
        })
    ).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_sync_events_handler_unauthorized() {
        let pool = PgPoolOptions::new().acquire_timeout(std::time::Duration::from_millis(10)).connect_lazy("postgres://localhost/dummy").unwrap();
        let state = State(pool);

        let req = SyncBatchRequest { tenant_id: "test-tenant".to_string(), batch_id: "batch-1".to_string(), events: vec![] };
        let headers = HeaderMap::new();

        let response = sync_events_handler(state, headers, Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
