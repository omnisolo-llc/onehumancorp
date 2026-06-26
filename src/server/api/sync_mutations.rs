use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct SyncMutationsRequest {
    pub mutations: Vec<SyncMutation>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SyncMutation {
    pub id: String,
    pub table_name: String,
    pub operation: String,
    pub payload_json: String,
    pub timestamp: i64,
    pub idempotency_key: String,
}

#[derive(Serialize)]
pub struct SyncMutationsResponse {
    pub success: bool,
    pub acked_ids: Vec<String>,
    pub failed_ids: Vec<String>,
}

pub async fn sync_mutations_handler(
    State(db): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<SyncMutationsRequest>,
) -> impl IntoResponse {
    tracing::info!("Received {} mutations for syncing.", payload.mutations.len());

    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(SyncMutationsResponse { success: false, acked_ids: vec![], failed_ids: vec![] }),
        ).into_response();
    }

    let mut acked_ids = vec![];
    let mut failed_ids = vec![];

    for mutation in payload.mutations {
        let mut tx = match db.begin().await {
            Ok(tx) => tx,
            Err(_) => {
                failed_ids.push(mutation.id);
                continue;
            }
        };

        // Check idempotency
        let exists: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM offline_sync_mutations WHERE tenant_id = $1 AND idempotency_key = $2"
        )
        .bind(&tenant_id)
        .bind(&mutation.idempotency_key)
        .fetch_one(&mut *tx)
        .await
        .unwrap_or((0,));

        if exists.0 > 0 {
            // Already processed
            acked_ids.push(mutation.id);
            let _ = tx.rollback().await;
            continue;
        }

        // Insert into offline_sync_mutations table
        let ts = chrono::DateTime::from_timestamp_millis(mutation.timestamp).unwrap_or_else(chrono::Utc::now);
        let res = sqlx::query(
            "INSERT INTO offline_sync_mutations (id, tenant_id, table_name, operation, payload, timestamp, idempotency_key, status)
             VALUES ($1, $2, $3, $4, $5::jsonb, $6, $7, 'PENDING')"
        )
        .bind(&mutation.id)
        .bind(&tenant_id)
        .bind(&mutation.table_name)
        .bind(&mutation.operation)
        .bind(&mutation.payload_json)
        .bind(ts)
        .bind(&mutation.idempotency_key)
        .execute(&mut *tx)
        .await;

        if res.is_ok() {
            if let Err(_) = tx.commit().await {
                failed_ids.push(mutation.id);
            } else {
                // If it's for customer contacts or tasks, we could trigger agent queue here
                // For now, it will be picked up by a background worker or process
                acked_ids.push(mutation.id);
            }
        } else {
            let _ = tx.rollback().await;
            failed_ids.push(mutation.id);
        }
    }

    (
        StatusCode::OK,
        Json(SyncMutationsResponse {
            success: failed_ids.is_empty(),
            acked_ids,
            failed_ids,
        }),
    ).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;

    #[tokio::test]
    async fn test_sync_mutations_handler_unauthorized() {
        let pool = crate::db::secure_pg_pool_options().acquire_timeout(std::time::Duration::from_millis(10)).connect_lazy("postgres://localhost/dummy").unwrap();
        let state = State(pool);
        let req = SyncMutationsRequest { mutations: vec![] };
        let headers = HeaderMap::new();

        let response = sync_mutations_handler(state, headers, Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
