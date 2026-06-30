use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::db::DB;

#[derive(Deserialize)]
pub struct CreateServiceRequest {
    pub title: String,
    pub description: Option<String>,
    pub price_cents: Option<i64>,
}

#[derive(Serialize)]
pub struct CreateServiceResponse {
    pub success: bool,
    pub service_id: Option<String>,
    pub error: Option<String>,
}

pub fn router<S>(db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_create_service))
        .with_state(db)
}

async fn handle_create_service(
    State(db): State<Arc<DB>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<CreateServiceRequest>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (axum::http::StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let service_id = uuid::Uuid::new_v4().to_string();
    let price = payload.price_cents.unwrap_or(0);
    let desc = payload.description.unwrap_or_else(|| "".to_string());

    match &db.store {
        crate::db::DbStore::Postgres => {
            let pool = db.pool.clone();
            let mut tx = match pool.begin().await {
                Ok(t) => t,
                Err(e) => {
                    tracing::error!("failed to begin tx: {}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(CreateServiceResponse {
                            success: false,
                            service_id: None,
                            error: Some("internal error".to_string()),
                        }),
                    )
                        .into_response();
                }
            };

            let _ = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;

            let res = sqlx::query(
                r#"
                INSERT INTO services (id, tenant_id, title, description, price_cents, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
                "#,
            )
            .bind(&service_id)
            .bind(&tenant_id)
            .bind(&payload.title)
            .bind(&desc)
            .bind(price)
            .execute(&mut *tx)
            .await;

            if let Err(e) = res {
                tracing::error!("failed to create service: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(CreateServiceResponse {
                        success: false,
                        service_id: None,
                        error: Some("failed to create service".to_string()),
                    }),
                )
                    .into_response();
            }
            let _ = tx.commit().await;
        }
        crate::db::DbStore::Sqlite(pool) => {
            let _ = sqlx::query("INSERT INTO tenants (id, business_name, plan_tier) VALUES (?, 'Test Tenant', 'starter') ON CONFLICT (id) DO NOTHING")
                .bind(&tenant_id)
                .execute(pool)
                .await;

            let res = sqlx::query(
                r#"
                INSERT INTO services (id, tenant_id, title, description, price_cents, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                "#,
            )
            .bind(&service_id)
            .bind(&tenant_id)
            .bind(&payload.title)
            .bind(&desc)
            .bind(price)
            .execute(pool)
            .await;

            if let Err(e) = res {
                tracing::error!("failed to create service in sqlite: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(CreateServiceResponse {
                        success: false,
                        service_id: None,
                        error: Some("failed to create service".to_string()),
                    }),
                )
                    .into_response();
            }
        }
    }

    (
        StatusCode::OK,
        Json(CreateServiceResponse {
            success: true,
            service_id: Some(service_id),
            error: None,
        }),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DB;
    use axum::http::Request;
    use tower::ServiceExt; // for `oneshot` and `ready`
    use serde_json::json;

    #[tokio::test]
    async fn test_create_service_success() {
        let pool = crate::db::create_sqlite_pool_for_test().await;

        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS tenants (id TEXT PRIMARY KEY, business_name TEXT, plan_tier TEXT)")
            .execute(&pool)
            .await;

        let _ = sqlx::query("CREATE TABLE IF NOT EXISTS services (id TEXT PRIMARY KEY, tenant_id TEXT, title TEXT, description TEXT, price_cents BIGINT, created_at TEXT, updated_at TEXT)")
            .execute(&pool)
            .await;

        let db = Arc::new(DB {
            pool: crate::db::get_pool(),
            store: crate::db::DbStore::Sqlite(pool),
        });
        let app = router(db.clone());

        let payload = json!({
            "title": "Test Service",
            "price_cents": 5000,
            "description": "Test description"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("x-tenant-id", "test-tenant-1")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_service_unauthorized() {
        let pool = crate::db::create_sqlite_pool_for_test().await;
        let db = Arc::new(DB {
            pool: crate::db::get_pool(),
            store: crate::db::DbStore::Sqlite(pool),
        });
        let app = router(db.clone());

        let payload = json!({
            "title": "Test Service",
            "price_cents": 5000,
            "description": "Test description"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-type", "application/json")
                    .body(axum::body::Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
