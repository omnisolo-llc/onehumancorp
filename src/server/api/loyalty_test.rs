use sqlx::PgPool;
use std::sync::Arc;
use axum::{body::Body, http::{Request, StatusCode}, Router};
use tower::ServiceExt;
use serde_json::json;

use crate::api::loyalty;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;

async fn setup_db() -> PgPool {
    let database_url = std::env::var("OHC_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
    crate::db::secure_pg_pool_options()
        .acquire_timeout(std::time::Duration::from_millis(500))
        .max_connections(1)
        .connect(&database_url)
        .await
        .unwrap()
}

async fn create_app(pool: PgPool) -> Router<()> {
    loyalty::router(pool, None)
}

#[tokio::test]
async fn test_get_customer_loyalty_status() {
    let pool = setup_db().await;
    let app = create_app(pool.clone()).await;

    // Create a dummy customer
    let tenant_id = "tenant123";
    let customer_id = uuid::Uuid::new_v4().to_string();

    if sqlx::query("SELECT 1").execute(&pool).await.is_err() {
        tracing::debug!("Skipping DB test, DB not available");
        return;
    }

    sqlx::query!(
        "INSERT INTO customers (id, tenant_id, name) VALUES ($1, $2, 'Test Customer')",
        uuid::Uuid::parse_str(&customer_id).unwrap(),
        tenant_id
    )
    .execute(&pool)
    .await
    .unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/status?tenant_id={}&customer_id={}", tenant_id, customer_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_generate_referral_link() {
    let pool = setup_db().await;
    let app = create_app(pool.clone()).await;

    let tenant_id = "tenant123";
    let customer_id = uuid::Uuid::new_v4().to_string();

    if sqlx::query("SELECT 1").execute(&pool).await.is_err() {
        tracing::debug!("Skipping DB test, DB not available");
        return;
    }

    sqlx::query!(
        "INSERT INTO customers (id, tenant_id, name) VALUES ($1, $2, 'Test Customer')",
        uuid::Uuid::parse_str(&customer_id).unwrap(),
        tenant_id
    )
    .execute(&pool)
    .await
    .unwrap();

    let payload = json!({
        "tenant_id": tenant_id,
        "customer_id": customer_id,
        "campaign_name": "Summer2024"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/referrals")
                .header("Content-Type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_record_loyalty_event() {
    let pool = setup_db().await;
    let app = create_app(pool.clone()).await;

    let tenant_id = "tenant123";
    let customer_id = uuid::Uuid::new_v4().to_string();

    if sqlx::query("SELECT 1").execute(&pool).await.is_err() {
        tracing::debug!("Skipping DB test, DB not available");
        return;
    }

    sqlx::query!(
        "INSERT INTO customers (id, tenant_id, name) VALUES ($1, $2, 'Test Customer')",
        uuid::Uuid::parse_str(&customer_id).unwrap(),
        tenant_id
    )
    .execute(&pool)
    .await
    .unwrap();

    let payload = json!({
        "tenant_id": tenant_id,
        "customer_id": customer_id,
        "event_type": "purchase",
        "event_points": 50,
        "source": "pos"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/events")
                .header("Content-Type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
