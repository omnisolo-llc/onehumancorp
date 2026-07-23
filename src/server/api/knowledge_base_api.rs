use axum::{
    extract::{State, Path},
    routing::{post, get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use axum::http::HeaderMap;

use crate::services::knowledge_base::service::KnowledgeBaseService;
use crate::services::knowledge_base::types::{SearchResult, KnowledgeBaseDocument};

#[derive(Clone)]
pub struct KnowledgeBaseApiState {
    pub db: sqlx::PgPool,
}

#[derive(Deserialize)]
pub struct IngestDocumentRequest {
    pub title: String,
    pub content: String,
    pub source_type: String,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct IngestDocumentResponse {
    pub id: String,
}

#[derive(Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}

pub fn router(db: sqlx::PgPool) -> Router {
    let state = KnowledgeBaseApiState { db };
    Router::new()
        .route("/knowledge_base/documents", post(ingest_document))
        .route("/knowledge_base/search", post(search_documents))
        .with_state(state)
}

fn extract_tenant_id(headers: &HeaderMap) -> Option<String> {
    headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).map(|s| s.to_string())
}

async fn ingest_document(
    headers: HeaderMap,
    State(state): State<KnowledgeBaseApiState>,
    Json(req): Json<IngestDocumentRequest>,
) -> Result<Json<IngestDocumentResponse>, (axum::http::StatusCode, String)> {
    let tenant_id = extract_tenant_id(&headers).ok_or_else(|| (axum::http::StatusCode::UNAUTHORIZED, "Missing x-tenant-id".to_string()))?;

    let service = KnowledgeBaseService::new(state.db.clone());
    let doc_id = service.ingest_document(&tenant_id, &req.title, &req.content, &req.source_type, req.metadata)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(IngestDocumentResponse { id: doc_id }))
}

async fn search_documents(
    headers: HeaderMap,
    State(state): State<KnowledgeBaseApiState>,
    Json(req): Json<SearchRequest>,
) -> Result<Json<SearchResponse>, (axum::http::StatusCode, String)> {
    let tenant_id = extract_tenant_id(&headers).ok_or_else(|| (axum::http::StatusCode::UNAUTHORIZED, "Missing x-tenant-id".to_string()))?;

    let service = KnowledgeBaseService::new(state.db.clone());
    let limit = req.limit.unwrap_or(5);

    let results = service.search(&tenant_id, &req.query, limit)
        .await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(SearchResponse { results }))
}
