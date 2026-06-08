use axum::{
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn router() -> Router<Arc<crate::db::DB>> {
    Router::new()
        .route("/api/v1/storefront/{*path}", get(get_storefront_cached))
}

#[derive(Serialize)]
pub struct StorefrontResponse {
    pub content: String,
}

pub async fn get_storefront_cached(
    State(db): State<Arc<crate::db::DB>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    // In a real app we'd fetch actual content based on `path`
    // Here we're just setting caching headers and edge-routing rules per spec
    let content = format!("Dynamic storefront content for {}", path);

    headers.insert("Cache-Control", HeaderValue::from_static("public, s-maxage=3600, stale-while-revalidate=86400"));
    headers.insert("CDN-Cache-Control", HeaderValue::from_static("max-age=3600"));

    let res = StorefrontResponse { content };
    (StatusCode::OK, headers, Json(res))
}
