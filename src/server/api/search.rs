use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::domain::search::{SearchService, SearchRequest as DomainSearchRequest};
use axum::http::StatusCode;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub domain: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

pub fn search_router(search_service: Arc<SearchService>) -> Router {
    Router::new()
        .route("/", get(search_handler))
        .with_state(search_service)
}

async fn search_handler(
    State(search_service): State<Arc<SearchService>>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let tenant_id = "test-tenant-id".to_string(); // Mock auth

    let req = DomainSearchRequest {
        query: params.q,
        domain_filter: params.domain,
        limit: params.limit.unwrap_or(20),
        offset: params.offset.unwrap_or(0),
        tenant_id,
    };

    match search_service.search(req).await {
        Ok(results) => {
            Ok(Json(serde_json::json!({ "results": results })))
        },
        Err(e) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, e))
        }
    }
}
