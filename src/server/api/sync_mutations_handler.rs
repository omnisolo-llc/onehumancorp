use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct GenericOfflineMutation {
    pub idempotency_key: String,
    pub entity_type: String,
    pub entity_id: String,
    pub action: String,
    pub payload: serde_json::Value,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct GenericOfflineSyncRequest {
    pub mutations: Vec<GenericOfflineMutation>,
}

#[derive(Serialize)]
pub struct GenericOfflineSyncResponse {
    pub success: bool,
    pub processed_count: i32,
    pub failed_count: i32,
    pub failed_mutations: Vec<String>,
}

pub async fn sync_mutations_handler(
    State(db): State<sqlx::PgPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<GenericOfflineSyncRequest>,
) -> impl IntoResponse {
    tracing::info!("Received {} generic offline mutations.", payload.mutations.len());

    let (tenant_id, _) = match crate::api::offline_sync::validate_token_and_get_tenant(&db, &headers).await {
        Ok(t) => t,
        Err(e) => return e,
    };

    if tenant_id.is_empty() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(GenericOfflineSyncResponse { success: false, processed_count: 0, failed_count: 0, failed_mutations: vec![] }),
        ).into_response();
    }

    let mut processed = 0;
    let mut failed = 0;
    let mut failed_ids = Vec::new();

    for mutation in payload.mutations {
        let tenant_uuid = uuid::Uuid::parse_str(&tenant_id).unwrap_or_default();
        let entity_uuid = uuid::Uuid::parse_str(&mutation.entity_id).unwrap_or_default();

        let result = sqlx::query(
            r#"
            INSERT INTO offline_mutations (tenant_id, idempotency_key, entity_type, entity_id, action, payload, status)
            VALUES ($1, $2, $3, $4, $5, $6, 'processed')
            ON CONFLICT (tenant_id, idempotency_key) DO NOTHING
            "#)
            .bind(tenant_uuid)
            .bind(mutation.idempotency_key.clone())
            .bind(mutation.entity_type)
            .bind(entity_uuid)
            .bind(mutation.action)
            .bind(mutation.payload)
        .execute(&db)
        .await;

        match result {
            Ok(res) => {
                if res.rows_affected() > 0 {
                    processed += 1;
                } else {
                    // Already processed
                    processed += 1;
                }
            }
            Err(e) => {
                tracing::error!("Failed to store mutation {}: {}", mutation.idempotency_key, e);
                failed += 1;
                failed_ids.push(mutation.idempotency_key);
            }
        }
    }

    (
        StatusCode::OK,
        Json(GenericOfflineSyncResponse {
            success: failed == 0,
            processed_count: processed,
            failed_count: failed,
            failed_mutations: failed_ids,
        }),
    ).into_response()
}
