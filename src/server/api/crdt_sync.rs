use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use server_common::Claims;
use sqlx::PgPool;

#[derive(Deserialize, Debug, Clone)]
pub struct CrdtDelta {
    pub id: String,
    pub entity_id: String,
    pub data: String, // Stringified JSON
    pub updated_at: String, // ISO timestamp
}

#[derive(Deserialize, Debug)]
pub struct CrdtSyncRequest {
    pub deltas: Vec<CrdtDelta>,
}

#[derive(Serialize)]
pub struct CrdtSyncResponse {
    pub success: bool,
}

pub async fn crdt_sync_handler(
    State(db): State<PgPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<CrdtSyncRequest>,
) -> impl IntoResponse {
    let tenant_id = match crate::api::mesh_handler::check_spiffe_auth(&headers) {
        Ok(_) => headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("").to_string(),
        Err(err) => return err,
    };

    if tenant_id.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(CrdtSyncResponse { success: false }),
        ).into_response();
    }

    let mut db_tx = match db.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(CrdtSyncResponse { success: false })).into_response();
        }
    };

    let _ = sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(&tenant_id)
        .execute(&mut *db_tx)
        .await;

    for delta in payload.deltas {
        let updated_at_parsed = match chrono::DateTime::parse_from_rfc3339(&delta.updated_at) {
            Ok(dt) => dt,
            Err(e) => {
                tracing::error!("Invalid timestamp format for delta {}: {}", delta.id, e);
                continue;
            }
        };

        let data_json: serde_json::Value = match serde_json::from_str(&delta.data) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let res = sqlx::query(
            r#"
            INSERT INTO crdt_deltas (id, tenant_id, entity_id, data, updated_at)
            VALUES ($1, $2, $3, $4::jsonb, $5)
            ON CONFLICT (tenant_id, entity_id) DO UPDATE
            SET data = crdt_deltas.data || EXCLUDED.data, updated_at = EXCLUDED.updated_at
            WHERE crdt_deltas.updated_at < EXCLUDED.updated_at
            "#
        )
        .bind(&delta.id)
        .bind(&tenant_id)
        .bind(&delta.entity_id)
        .bind(&data_json)
        .bind(updated_at_parsed)
        .execute(&mut *db_tx)
        .await;

        if let Err(e) = res {
            tracing::error!("Failed to insert CRDT delta {}: {}", delta.id, e);
            continue;
        }

        // Queue the resolution of this CRDT delta to the job queue for async processing by workers
        let job_payload = serde_json::json!({
            "delta_id": delta.id,
            "entity_id": delta.entity_id,
            "data": data_json,
            "updated_at": delta.updated_at
        });

        let _ = sqlx::query(
            "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload) VALUES ($1, $2, 'crdt_sync_resolution', $3::jsonb)"
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(&tenant_id)
        .bind(&job_payload)
        .execute(&mut *db_tx)
        .await;
    }

    match db_tx.commit().await {
        Ok(_) => (StatusCode::OK, Json(CrdtSyncResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to commit transaction: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(CrdtSyncResponse { success: false })).into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_crdt_sync_handler() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return;
        }

        let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        // Tests use regular schema from migrations, but if we need it here:
        sqlx::query("CREATE TABLE IF NOT EXISTS crdt_deltas (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, entity_id TEXT NOT NULL, data JSONB NOT NULL, updated_at TIMESTAMPTZ NOT NULL, created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool).await.unwrap();
        sqlx::query("CREATE UNIQUE INDEX IF NOT EXISTS idx_crdt_deltas_tenant_entity ON crdt_deltas(tenant_id, entity_id)")
            .execute(&pool).await.unwrap();
        sqlx::query("ALTER TABLE crdt_deltas ENABLE ROW LEVEL SECURITY").execute(&pool).await.unwrap();
        sqlx::query("DROP POLICY IF EXISTS tenant_isolation_crdt_deltas ON crdt_deltas").execute(&pool).await.unwrap();
        sqlx::query("CREATE POLICY tenant_isolation_crdt_deltas ON crdt_deltas USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true))").execute(&pool).await.unwrap();

        // Ensure ohc_job_queue table exists
        sqlx::query("CREATE TABLE IF NOT EXISTS ohc_job_queue (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, job_type TEXT NOT NULL, payload JSONB NOT NULL, status TEXT DEFAULT 'pending')")
            .execute(&pool).await.unwrap();

        let state = State(pool.clone());

        let req = CrdtSyncRequest {
            deltas: vec![
                CrdtDelta {
                    id: "delta1".to_string(),
                    entity_id: "task1".to_string(),
                    data: "{\"status\": \"completed\"}".to_string(),
                    updated_at: "2026-04-17T12:00:00Z".to_string(),
                }
            ],
        };

        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-crdt/agent/x".parse().unwrap());
        headers.insert("x-tenant-id", "tenant-crdt".parse().unwrap());

        let response = crdt_sync_handler(state.clone(), headers.clone(), Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        // Verify it was inserted
        let row: (String,) = sqlx::query_as("SELECT data->>'status' FROM crdt_deltas WHERE entity_id = 'task1' AND tenant_id = 'tenant-crdt'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(row.0, "completed");

        // Try inserting older update (should be ignored by ON CONFLICT)
        let req2 = CrdtSyncRequest {
            deltas: vec![
                CrdtDelta {
                    id: "delta2".to_string(), // new ID
                    entity_id: "task1".to_string(), // same entity
                    data: "{\"status\": \"pending\"}".to_string(),
                    updated_at: "2026-04-17T10:00:00Z".to_string(), // older
                }
            ],
        };

        let response2 = crdt_sync_handler(state.clone(), headers.clone(), Json(req2)).await.into_response();
        assert_eq!(response2.status(), StatusCode::OK);

        let row2: (String,) = sqlx::query_as("SELECT data->>'status' FROM crdt_deltas WHERE entity_id = 'task1' AND tenant_id = 'tenant-crdt'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(row2.0, "completed"); // Remains "completed"

        // Newer update
        let req3 = CrdtSyncRequest {
            deltas: vec![
                CrdtDelta {
                    id: "delta3".to_string(), // new ID
                    entity_id: "task1".to_string(),
                    data: "{\"status\": \"archived\"}".to_string(),
                    updated_at: "2026-04-17T14:00:00Z".to_string(), // newer
                }
            ],
        };

        let response3 = crdt_sync_handler(state, headers, Json(req3)).await.into_response();
        assert_eq!(response3.status(), StatusCode::OK);

        let row3: (String,) = sqlx::query_as("SELECT data->>'status' FROM crdt_deltas WHERE entity_id = 'task1' AND tenant_id = 'tenant-crdt'")
            .fetch_one(&pool).await.unwrap();
        assert_eq!(row3.0, "archived"); // updated to "archived"
    }
}
