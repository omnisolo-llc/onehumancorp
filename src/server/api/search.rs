use axum::{
    extract::{Query, State},
    response::Json,
    Extension, Router,
};
use serde::{Deserialize, Serialize};

use crate::db::{DB, SearchResult};

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub success: bool,
    pub results: Vec<SearchResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub fn router(db: DB) -> Router {
    Router::new()
        .route("/", axum::routing::get(search_workspace_handler))
        .with_state(db)
}

async fn search_workspace_handler(
    State(db): State<DB>,
    Extension(claims): Extension<::server_common::Claims>,
    Query(query): Query<SearchQuery>,
) -> Json<SearchResponse> {
    let tenant_id = claims
        .organization_id
        .clone()
        .unwrap_or_else(|| claims.sub.clone()); // Assuming tenant fallback to sub if org missing

    match db.search_workspace(&tenant_id, &query.q).await {
        Ok(results) => Json(SearchResponse {
            success: true,
            results,
            error: None,
        }),
        Err(e) => Json(SearchResponse {
            success: false,
            results: vec![],
            error: Some(e),
        }),
    }
}
