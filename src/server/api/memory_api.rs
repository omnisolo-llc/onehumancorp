use axum::{
    extract::{State, Json, Extension},
    routing::post,
    Router,
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::domain::memory::pgvector_memory::MemoryService;
use ohc_builtin_agent::mesh::transport::MeshTransport;

#[derive(Deserialize)]
pub struct IngestMemoryRequest {
    pub department: String,
    pub content: String,
    pub embedding: Vec<f32>,
}

#[derive(Serialize)]
pub struct IngestMemoryResponse {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct RecallMemoryRequest {
    pub embedding: Vec<f32>,
    pub limit: i64,
}

#[derive(Serialize)]
pub struct AgentMemoryDto {
    pub id: i32,
    pub department: String,
    pub content: String,
    pub embedding: Vec<f32>,
}

#[derive(Clone)]
pub struct ApiState {
    pub memory_service: Arc<MemoryService>,
}

pub fn router(state: Arc<ApiState>) -> Router<Arc<dyn MeshTransport>> {
    let memory_router = Router::new()
        .route("/ingest", post(ingest_memory))
        .route("/recall", post(recall_memory))
        .with_state(state);

    Router::new().nest("/api/v1/memory", memory_router)
}

async fn ingest_memory(
    State(state): State<Arc<ApiState>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<IngestMemoryRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let tenant_id = claims.sub;

    let id = state.memory_service
        .ingest_memory(&tenant_id, &payload.department, &payload.content, payload.embedding)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(IngestMemoryResponse { id }))
}

async fn recall_memory(
    State(state): State<Arc<ApiState>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<RecallMemoryRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let tenant_id = claims.sub;

    let memories = state.memory_service
        .recall_memory(&tenant_id, payload.embedding, payload.limit)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let dtos: Vec<AgentMemoryDto> = memories.into_iter().map(|m| AgentMemoryDto {
        id: m.id,
        department: m.department,
        content: m.content,
        embedding: m.embedding,
    }).collect();

    Ok(Json(dtos))
}
