use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use axum::http::HeaderMap;
use crate::db::DB;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use ohc_builtin_agent::memory_store::VectorRepository;

#[derive(Serialize)]
pub struct MemoryResponse {
    pub id: String,
    pub content: String,
    pub source_type: String,
    pub owner_override: bool,
    pub reference_count: i32,
    pub reliability_score: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_referenced_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct OverrideRequest {
    pub override_value: bool,
    pub content: Option<String>,
}

pub fn router(db: Arc<DB>) -> Router<std::sync::Arc<(dyn crate::mesh_handler::MeshTransport + 'static)>> {
    let repo = if matches!(db.store, crate::db::DbStore::Postgres) {
        Arc::new(VectorRepository::new(db.pool.clone()))
    } else {
        match &db.store {
            crate::db::DbStore::Sqlite(pool) => Arc::new(VectorRepository::new_sqlite(pool.clone())),
            _ => unreachable!(),
        }
    };
    Router::new()
        .route("/", get(list_memories))
        .route("/:id/override", post(override_memory))
        .with_state(repo)
}

async fn list_memories(
    State(repo): State<Arc<VectorRepository>>,
    auth_info: axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<Vec<MemoryResponse>>, (axum::http::StatusCode, String)> {
    let tenant_id = auth_info.organization_id.clone().unwrap_or_else(|| "default".to_string());

    let results = repo.cross_department_search(&tenant_id, &vec![0.0; 1536], 100).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;

    let out = results.into_iter().map(|r| MemoryResponse {
        id: r.id,
        content: r.content,
        source_type: r.source_type,
        owner_override: r.owner_override,
        reference_count: r.reference_count,
        reliability_score: r.reliability_score,
        created_at: r.created_at,
        last_referenced_at: r.last_referenced_at,
    }).collect();

    Ok(Json(out))
}

async fn override_memory(
    State(repo): State<Arc<VectorRepository>>,
    Path(id): Path<String>,
    auth_info: axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<OverrideRequest>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let tenant_id = auth_info.organization_id.clone().unwrap_or_else(|| "default".to_string());

    let mut record = repo.get_by_id(&id).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?
        .ok_or((axum::http::StatusCode::NOT_FOUND, "Memory not found".to_string()))?;

    if record.tenant_id != tenant_id {
        return Err((axum::http::StatusCode::FORBIDDEN, "Access denied".to_string()));
    }

    record.owner_override = payload.override_value;
    if let Some(c) = payload.content {
        record.content = c;
    }

    repo.upsert(&record).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(serde_json::json!({"success": true})))
}
