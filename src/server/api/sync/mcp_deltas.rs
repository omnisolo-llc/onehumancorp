use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CrdtDeltaPayload {
    pub id: String,
    pub entity_id: String,
    pub data: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct SyncMcpDeltasRequest {
    pub deltas: Vec<CrdtDeltaPayload>,
}

#[derive(Debug, Serialize)]
pub struct SyncMcpDeltasResponse {
    pub status: String,
    pub synced_count: i32,
}

pub async fn sync_mcp_deltas_handler(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<SyncMcpDeltasRequest>,
) -> impl IntoResponse {
    let mut synced_count = 0;

    // Using Last-Writer-Wins (LWW) based on updated_at
    for delta in payload.deltas {
        let query = r#"
            INSERT INTO crdt_deltas (id, entity_id, data, updated_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE SET
                data = EXCLUDED.data,
                updated_at = EXCLUDED.updated_at
            WHERE crdt_deltas.updated_at < EXCLUDED.updated_at
        "#;

        match sqlx::query(query)
            .bind(&delta.id)
            .bind(&delta.entity_id)
            .bind(&delta.data)
            .bind(&delta.updated_at)
            .execute(&pool)
            .await {
                Ok(_) => { synced_count += 1; },
                Err(e) => {
                    tracing::error!("Failed to sync CRDT delta {}: {}", delta.id, e);
                }
            }
    }

    (
        StatusCode::OK,
        Json(SyncMcpDeltasResponse {
            status: "success".to_string(),
            synced_count,
        }),
    )
}
