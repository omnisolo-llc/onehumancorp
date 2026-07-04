use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum::http::StatusCode;
use crate::services::storefront::service::StorefrontService;
use crate::domain::storefront::Storefront;

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/storefront/:tenant_id/draft", post(create_draft))
        .route("/api/v1/storefront/:tenant_id/publish", post(publish))
        .route("/api/v1/storefront/:tenant_id/:product_id", get(get_storefront))
}

async fn create_draft(Path(tenant_id): Path<String>) -> impl IntoResponse {
    let storefront = StorefrontService::create_draft(&tenant_id).await;
    (StatusCode::CREATED, Json(storefront))
}

async fn publish(Path(tenant_id): Path<String>) -> impl IntoResponse {
    let mut storefront = StorefrontService::create_draft(&tenant_id).await;
    storefront = StorefrontService::publish(storefront).await;
    (StatusCode::OK, Json(storefront))
}

async fn get_storefront(Path((tenant_id, _product_id)): Path<(String, String)>) -> impl IntoResponse {
    let mut storefront = StorefrontService::create_draft(&tenant_id).await;
    storefront = StorefrontService::publish(storefront).await;
    axum::response::Html(storefront.html_content)
}
