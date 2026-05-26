use axum::{Json, extract::State, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct CrdtDelta {
    pub id: String,
    pub entity_id: String,
    pub data: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CrdtSyncRequest {
    pub deltas: Vec<CrdtDelta>,
}

#[derive(Debug, Serialize)]
pub struct CrdtSyncResponse {
    pub status: String,
}

pub async fn handle_crdt_sync(
    State(pool): State<PgPool>,
    Json(payload): Json<CrdtSyncRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for delta in payload.deltas {
        let query = r#"
            INSERT INTO crdt_deltas (id, entity_id, data, updated_at)
            VALUES ($1, $2, $3, $4::timestamptz)
            ON CONFLICT (id) DO UPDATE SET
                data = EXCLUDED.data,
                updated_at = EXCLUDED.updated_at
            WHERE crdt_deltas.updated_at < EXCLUDED.updated_at
        "#;

        sqlx::query(query)
            .bind(&delta.id)
            .bind(&delta.entity_id)
            .bind(&delta.data)
            .bind(&delta.updated_at)
            .execute(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(CrdtSyncResponse {
        status: "success".to_string(),
    }))
}
