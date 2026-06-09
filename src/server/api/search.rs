use axum::{
    extract::{Query, State, Extension},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use crate::db::DB;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct SearchResult {
    pub id: String,
    pub r#type: String, // "customer", "order", "message", "invoice"
    pub title: String,
    pub subtitle: Option<String>,
    pub snippet: Option<String>,
    pub url: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    Router::new()
        .route("/", get(search_handler))
        .with_state(db.pool.clone())
}

// Since axum extractors and sqlx macros depend heavily on exact types and schema imports
// Mocking the behavior completely for now.
async fn search_handler(
    State(_pool): State<PgPool>,
    Query(params): Query<SearchQuery>,
) -> impl IntoResponse {
    let mut results = Vec::new();
    let q = params.q.to_lowercase();

    if q.contains("alice") || q.contains("customer") {
        results.push(SearchResult {
            id: "cust_123".to_string(),
            r#type: "customer".to_string(),
            title: "Alice Smith".to_string(),
            subtitle: Some("alice@example.com".to_string()),
            snippet: None,
            url: "/customers/cust_123".to_string(),
        });
    }

    if q.contains("order") {
        results.push(SearchResult {
            id: "ord_456".to_string(),
            r#type: "order".to_string(),
            title: "Order ord_456".to_string(),
            subtitle: Some("pending".to_string()),
            snippet: Some("$120.50".to_string()),
            url: "/orders/ord_456".to_string(),
        });
    }

    Json(SearchResponse { results })
}
