use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct KnowledgeState {
    pub pool: PgPool,
}

pub fn router(state: KnowledgeState) -> Router {
    Router::new()
        .route("/v1/knowledge", post(upload_document).get(list_documents))
        .route("/v1/knowledge/:id", get(get_document))
        .route("/v1/knowledge/query", post(query_knowledge))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct UploadRequest {
    title: String,
    file_type: String,
    content: String,
}

#[derive(Serialize)]
pub struct DocumentResponse {
    id: Uuid,
    title: String,
    status: String,
}

async fn upload_document(
    State(state): State<KnowledgeState>,
    // TODO: Extract tenant_id from auth token
    Json(payload): Json<UploadRequest>,
) -> Result<Json<DocumentResponse>, (StatusCode, String)> {
    let tenant_id = "tenant_1"; // Mock tenant

    let id = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO knowledge_documents (id, tenant_id, title, file_type, content) VALUES ($1, $2, $3, $4, $5)",
        id, tenant_id, payload.title, payload.file_type, payload.content
    )
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(DocumentResponse {
        id,
        title: payload.title,
        status: "PENDING".to_string(),
    }))
}

async fn list_documents(
    State(state): State<KnowledgeState>,
) -> Result<Json<Vec<DocumentResponse>>, (StatusCode, String)> {
    let tenant_id = "tenant_1";

    let docs = sqlx::query_as!(
        DocumentResponse,
        "SELECT id, title, status FROM knowledge_documents WHERE tenant_id = $1 ORDER BY created_at DESC",
        tenant_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(docs))
}

async fn get_document(
    State(state): State<KnowledgeState>,
    Path(id): Path<Uuid>,
) -> Result<Json<DocumentResponse>, (StatusCode, String)> {
    let tenant_id = "tenant_1";

    let doc = sqlx::query_as!(
        DocumentResponse,
        "SELECT id, title, status FROM knowledge_documents WHERE id = $1 AND tenant_id = $2",
        id, tenant_id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "Not found".to_string()))?;

    Ok(Json(doc))
}

#[derive(Deserialize)]
pub struct QueryRequest {
    query: String,
}

#[derive(Serialize)]
pub struct QueryResponse {
    answer: String,
    sources: Vec<Uuid>,
}

async fn query_knowledge(
    State(state): State<KnowledgeState>,
    Json(payload): Json<QueryRequest>,
) -> Result<Json<QueryResponse>, (StatusCode, String)> {
    let tenant_id = "tenant_1";
    // Mock querying via vector DB
    Ok(Json(QueryResponse {
        answer: "Based on your documents, the policy is...".to_string(),
        sources: vec![],
    }))
}
