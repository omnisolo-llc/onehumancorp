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

#[derive(Deserialize)]
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
    use axum::http::Request;
    use axum::body::Body;
    use tower::ServiceExt;
    use ::server_common::Claims;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_create_product() {
        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap();

        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS tenants (id TEXT, name TEXT, industry TEXT)").execute(&sqlite_pool).await;
        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS products (id TEXT, organization_id TEXT, title TEXT, type TEXT, price REAL)").execute(&sqlite_pool).await;

        unsafe {
            std::env::set_var("OHC_DATABASE_URL", "sqlite::memory:");
            std::env::set_var("OHC_SQLITE_KEY", "testkey");
        }

        let db_arc = Arc::new(crate::db::DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap(),
            store: crate::db::DbStore::Sqlite(sqlite_pool.clone())
        });

        let hub = Arc::new(Hub::new(tx, db_arc.pool.clone()));

        let app = Router::new()
            .route("/product", post(handle_create_product))
            .layer(Extension(hub))
            .layer(Extension(Claims {
                sub: "user-123".to_string(),
                email: "test@example.com".to_string(),
                exp: 0,
                organization_id: Some("tenant-123".to_string()),
                iat: 0,
                username: "test_user".to_string(),
                roles: vec![],
                session_id: Some("sess".to_string()),
                jti: "jti".to_string(),
            }));

        let payload = r#"{
            "name": "Test Cake",
            "price": "20.00",
            "duration": 60,
            "description": "Delicious test cake",
            "item_type": "physical"
        }"#;

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/product")
                    .header("Content-Type", "application/json")
                    .body(Body::from(payload))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
