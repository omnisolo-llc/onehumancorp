use crate::api::expenses::{router, ExpenseReceipt, UploadExpenseRequest};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;
use tower::ServiceExt;

#[sqlx::test]
async fn test_upload_expense(pool: PgPool) {
    let app = router(pool);

    let tenant_id = "test_tenant";

    // Switch to the correct tenant for RLS
    sqlx::query("SET app.current_tenant = 'test_tenant'")
        .execute(&pool)
        .await
        .unwrap();

    let payload = UploadExpenseRequest {
        image_path: Some("s3://bucket/receipt.jpg".to_string()),
        vendor: Some("Home Depot".to_string()),
        amount: Some(150.50),
        date: Some(chrono::Utc::now()),
    };

    let request = Request::builder()
        .method("POST")
        .uri(&format!("/api/v1/tenants/{}/expenses", tenant_id))
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[sqlx::test]
async fn test_list_expenses(pool: PgPool) {
    let app = router(pool.clone());

    let tenant_id = "test_tenant";

    // Setup some data
    sqlx::query("SET app.current_tenant = 'test_tenant'")
        .execute(&pool)
        .await
        .unwrap();

    let id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO ohc_expense_receipts (id, tenant_id, vendor, amount, status) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(&id)
    .bind(tenant_id)
    .bind("Lowes")
    .bind(45.00)
    .bind("pending")
    .execute(&pool)
    .await
    .unwrap();

    let request = Request::builder()
        .method("GET")
        .uri(&format!("/api/v1/tenants/{}/expenses", tenant_id))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
