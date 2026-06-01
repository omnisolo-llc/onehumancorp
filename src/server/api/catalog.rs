use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;
use axum::http::StatusCode;

#[derive(Deserialize, Serialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub price: String,
    pub duration: Option<i32>,
    pub description: String,
    pub item_type: String,
}

#[derive(Serialize)]
pub struct CreateProductResponse {
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

async fn handle_create_product(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateProductRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());

    if payload.name.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "INVALID_INPUT".to_string(), message: "Product name cannot be empty".to_string() })).into_response();
    }
    if payload.price.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "INVALID_INPUT".to_string(), message: "Product price cannot be empty".to_string() })).into_response();
    }

    // Check product quota
    let quota_status = hub.tracker().check_product_quota(&tenant_id).await.unwrap_or_else(|e| {
        tracing::warn!("Failed to check product quota for tenant {}: {}", tenant_id, e);
        ::server_pricing::rate_limit::RateLimitStatus {
            is_allowed: true,
            soft_limit_reached: false,
            user_message: None,
        }
    });

    if quota_status.soft_limit_reached && !quota_status.is_allowed {
        let msg = quota_status.user_message.unwrap_or_else(|| "Tier limit reached. Please upgrade.".to_string());
        return (
            StatusCode::PAYMENT_REQUIRED,
            Json(ErrorResponse {
                error: "LIMIT_EXCEEDED".to_string(),
                message: msg,
            }),
        ).into_response();
    }

    // Record product addition
    let _ = hub.tracker().record_product_added(&tenant_id).await;

    // In a real app we'd save to the DB here

    (StatusCode::OK, Json(CreateProductResponse { success: true, message: Some(format!("Created {}", payload.name)) })).into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/product", post(handle_create_product))
        .layer(Extension(hub))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_create_product_empty_name() {
        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let fallback_pg = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        let db_standalone = crate::db::DB { pool: fallback_pg, store: crate::db::DbStore::Sqlite(sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap()) };
        let hub = Arc::new(Hub::new(tx, db_standalone.pool.clone()));

        let payload = CreateProductRequest {
            name: "  ".to_string(),
            price: "100".to_string(),
            duration: None,
            description: "test".to_string(),
            item_type: "item".to_string(),
        };

        let claims = ::server_common::Claims {
            sub: "test".to_string(),
            exp: 0,
            iat: 0,
            jti: "test".to_string(),
            organization_id: Some("system".to_string()),
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            roles: vec!["admin".to_string()],
            session_id: None,
        };

        let response = handle_create_product(Extension(hub), Extension(claims), Json(payload)).await.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_create_product_empty_price() {
        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let fallback_pg = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        let db_standalone = crate::db::DB { pool: fallback_pg, store: crate::db::DbStore::Sqlite(sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap()) };
        let hub = Arc::new(Hub::new(tx, db_standalone.pool.clone()));

        let payload = CreateProductRequest {
            name: "Test Product".to_string(),
            price: "".to_string(),
            duration: None,
            description: "test".to_string(),
            item_type: "item".to_string(),
        };

        let claims = ::server_common::Claims {
            sub: "test".to_string(),
            exp: 0,
            iat: 0,
            jti: "test".to_string(),
            organization_id: Some("system".to_string()),
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            roles: vec!["admin".to_string()],
            session_id: None,
        };

        let response = handle_create_product(Extension(hub), Extension(claims), Json(payload)).await.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
