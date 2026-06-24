use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SyncEvent {
    pub id: String,
    pub batch_id: Option<String>,
    pub action_type: String,
    pub entity_id: Option<String>,
    pub base_version: Option<i64>,
    pub payload: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SyncEventsRequest {
    pub events: Vec<SyncEvent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SyncEventsResponse {
    pub status: String,
    pub processed_count: i32,
    pub conflict_count: i32,
}

pub async fn events_handler(
    State(pool): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<SyncEventsRequest>,
) -> impl IntoResponse {
    let spiffe_id_str = headers.get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
    let (tenant_id, _) = crate::auth::parse_spiffe_id(spiffe_id_str).unwrap_or(("".to_string(), "".to_string()));

    if tenant_id.is_empty() {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "status": "error",
            "message": "missing tenant identity in session",
        }))).into_response();
    }

    let mut processed_count = 0;
    let mut conflict_count = 0;

    for event in payload.events {
        match check_and_process_event(&pool, &tenant_id, &event).await {
            Ok(is_conflict) => {
                if is_conflict {
                    conflict_count += 1;
                } else {
                    processed_count += 1;
                }
            }
            Err(e) => {
                tracing::error!("Failed to process sync event {}: {:?}", event.id, e);
            }
        }
    }

    (StatusCode::OK, Json(SyncEventsResponse {
        status: "success".to_string(),
        processed_count,
        conflict_count,
    })).into_response()
}

async fn check_and_process_event(pool: &sqlx::PgPool, tenant_id: &str, event: &SyncEvent) -> Result<bool, sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

    // Check idempotency in both sync_events and conflict_queue
    let exists: Option<i64> = sqlx::query_scalar(
        "SELECT 1 FROM sync_events WHERE id = $1 AND tenant_id = $2"
    )
    .bind(&event.id)
    .bind(tenant_id)
    .fetch_optional(&mut *tx)
    .await?;

    if exists.is_some() {
        tx.commit().await?;
        return Ok(false);
    }

    let exists_conflict: Option<i64> = sqlx::query_scalar(
        "SELECT 1 FROM conflict_queue WHERE sync_event_id = $1 AND tenant_id = $2"
    )
    .bind(&event.id)
    .bind(tenant_id)
    .fetch_optional(&mut *tx)
    .await?;

    if exists_conflict.is_some() {
        tx.commit().await?;
        return Ok(true); // Return true because it was already flagged as a conflict previously
    }

    let mut is_conflict = false;

    if let (Some(entity_id), Some(base_version)) = (&event.entity_id, event.base_version) {
        let current_version: Option<i64> = sqlx::query_scalar(
            "SELECT version FROM entity_versions WHERE entity_id = $1 AND tenant_id = $2"
        )
        .bind(entity_id)
        .bind(tenant_id)
        .fetch_optional(&mut *tx)
        .await?;

        let current_version = current_version.unwrap_or(1);

        if current_version > base_version {
            is_conflict = true;
        }
    }

    if is_conflict {
        sqlx::query(
            "INSERT INTO conflict_queue (id, tenant_id, sync_event_id, entity_id, base_version, payload) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(tenant_id)
        .bind(&event.id)
        .bind(event.entity_id.clone().unwrap_or_default())
        .bind(event.base_version)
        .bind(&event.payload)
        .execute(&mut *tx)
        .await?;
    } else {
        sqlx::query(
            "INSERT INTO sync_events (id, tenant_id, batch_id, action_type, entity_id, base_version, payload) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(&event.id)
        .bind(tenant_id)
        .bind(&event.batch_id)
        .bind(&event.action_type)
        .bind(&event.entity_id)
        .bind(event.base_version)
        .bind(&event.payload)
        .execute(&mut *tx)
        .await?;

        if let Some(entity_id) = &event.entity_id {
            sqlx::query(
                "INSERT INTO entity_versions (entity_id, tenant_id, version) VALUES ($1, $2, $3) ON CONFLICT (tenant_id, entity_id) DO UPDATE SET version = entity_versions.version + 1"
            )
            .bind(entity_id)
            .bind(tenant_id)
            .bind(event.base_version.unwrap_or(1) + 1)
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;
    Ok(is_conflict)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;

    #[tokio::test]
    async fn test_sync_events_handler_unauthorized() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        let pool = crate::db::secure_pg_pool_options().acquire_timeout(std::time::Duration::from_millis(10)).connect_lazy(&database_url).unwrap();

        let state = State(pool);
        let req = SyncEventsRequest { events: vec![] };
        let headers = HeaderMap::new();

        let response = events_handler(state, headers, Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_sync_events_handler_success() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") { return; }
        let pool = crate::db::secure_pg_pool_options().connect(&database_url).await.unwrap();

        sqlx::query("INSERT INTO tenants (id, name) VALUES ('tenant-sync-events', 'Test Tenant') ON CONFLICT DO NOTHING").execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM sync_events WHERE tenant_id = 'tenant-sync-events'").execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM conflict_queue WHERE tenant_id = 'tenant-sync-events'").execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM entity_versions WHERE tenant_id = 'tenant-sync-events'").execute(&pool).await.unwrap();

        let state = State(pool.clone());
        let req = SyncEventsRequest {
            events: vec![
                SyncEvent {
                    id: "event-1".to_string(),
                    batch_id: None,
                    action_type: "update_status".to_string(),
                    entity_id: Some("entity-1".to_string()),
                    base_version: Some(1),
                    payload: "{}".to_string(),
                }
            ],
        };
        let mut headers = HeaderMap::new();
        headers.insert("x-spiffe-id", "spiffe://ohc/org/tenant-sync-events/agent/x".parse().unwrap());

        let response = events_handler(state.clone(), headers.clone(), Json(req)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(body_json["processed_count"], 1);
        assert_eq!(body_json["conflict_count"], 0);

        // Idempotency check
        let req2 = SyncEventsRequest {
            events: vec![
                SyncEvent {
                    id: "event-1".to_string(),
                    batch_id: None,
                    action_type: "update_status".to_string(),
                    entity_id: Some("entity-1".to_string()),
                    base_version: Some(1),
                    payload: "{}".to_string(),
                }
            ],
        };
        let response2 = events_handler(state.clone(), headers.clone(), Json(req2)).await.into_response();
        assert_eq!(response2.status(), StatusCode::OK);
        let body_bytes2 = axum::body::to_bytes(response2.into_body(), usize::MAX).await.unwrap();
        let body_json2: serde_json::Value = serde_json::from_slice(&body_bytes2).unwrap();
        assert_eq!(body_json2["processed_count"], 1); // We count it as processed if it's already there (idempotency, or you can count it as 0 processed). The current implementation counts it as processed (because it returns false for conflict). Actually, if it skips, we shouldn't fail. The user gets success.

        // Conflict check
        let req3 = SyncEventsRequest {
            events: vec![
                SyncEvent {
                    id: "event-2".to_string(),
                    batch_id: None,
                    action_type: "update_status".to_string(),
                    entity_id: Some("entity-1".to_string()),
                    base_version: Some(1), // Base is 1, but we bumped it to 2 on event-1
                    payload: "{}".to_string(),
                }
            ],
        };
        let response3 = events_handler(state.clone(), headers.clone(), Json(req3)).await.into_response();
        let body_bytes3 = axum::body::to_bytes(response3.into_body(), usize::MAX).await.unwrap();
        let body_json3: serde_json::Value = serde_json::from_slice(&body_bytes3).unwrap();
        assert_eq!(body_json3["processed_count"], 0);
        assert_eq!(body_json3["conflict_count"], 1);
    }
}
