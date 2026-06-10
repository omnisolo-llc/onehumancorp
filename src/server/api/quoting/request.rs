use axum::{
    extract::{State, Json, Path},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::db::DB;
use sqlx::Row;

#[derive(Serialize)]
pub struct QuoteItemResponse {
    pub id: String,
    pub description: String,
    pub price: f64,
    pub quantity: i32,
    pub is_optional: bool,
    pub selected: bool,
}

#[derive(Serialize)]
pub struct QuoteResponse {
    pub id: String,
    pub customer_name: String,
    pub request_text: String,
    pub status: String,
    pub items: Vec<QuoteItemResponse>,
}

#[derive(Deserialize)]
pub struct AcceptQuoteRequest {
    pub selected_item_ids: Vec<String>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/:quote_id", get(get_quote).post(accept_quote))
        .with_state(orchestrator)
}

async fn get_quote(
    State(_orchestrator): State<Arc<DepartmentOrchestrator>>,
    headers: axum::http::HeaderMap,
    Path(quote_id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let pool = crate::db::get_pool();
    let quote_record = match sqlx::query(
        "SELECT q.id, q.status, c.name as customer_name FROM quotes q LEFT JOIN customers c ON c.id = q.customer_id AND c.tenant_id = q.tenant_id WHERE q.tenant_id = $1 AND q.id = $2"
    )
    .bind(&tenant_id)
    .bind(&quote_id)
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(r)) => r,
        _ => return (StatusCode::NOT_FOUND, axum::Json(serde_json::json!({"error": "not found"}))).into_response(),
    };

    let items_records = sqlx::query(
        "SELECT id, description, price, quantity, is_optional, selected FROM quote_items WHERE tenant_id = $1 AND quote_id = $2"
    )
    .bind(&tenant_id)
    .bind(&quote_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let items: Vec<QuoteItemResponse> = items_records.into_iter().map(|r| QuoteItemResponse {
        id: r.try_get("id").unwrap_or_default(),
        description: r.try_get("description").unwrap_or_default(),
        price: (r.try_get::<i64, _>("price").unwrap_or(0) as f64) / 100.0,
        quantity: r.try_get("quantity").unwrap_or(1),
        is_optional: r.try_get("is_optional").unwrap_or(false),
        selected: r.try_get("selected").unwrap_or(true),
    }).collect();

    let customer_name: String = quote_record.try_get("customer_name").unwrap_or_else(|_| "Unknown Customer".to_string());
    let status: String = quote_record.try_get("status").unwrap_or_else(|_| "DRAFT".to_string());

    let res = QuoteResponse {
        id: quote_id,
        customer_name,
        request_text: "Service inquiry".to_string(), // In real implementation, link to messages/leads
        status,
        items,
    };

    (StatusCode::OK, Json(res)).into_response()
}

async fn accept_quote(
    State(_orchestrator): State<Arc<DepartmentOrchestrator>>,
    headers: axum::http::HeaderMap,
    Path(quote_id): Path<String>,
    Json(payload): Json<AcceptQuoteRequest>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let pool = crate::db::get_pool();
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "db error"}))).into_response(),
    };

    // In real app, calculate total and generate a Stripe Checkout Session
    // For now we simulate success.

    let _ = sqlx::query("UPDATE quotes SET status = 'ACCEPTED' WHERE tenant_id = $1 AND id = $2")
        .bind(&tenant_id)
        .bind(&quote_id)
        .execute(&mut *tx)
        .await;

    for item_id in payload.selected_item_ids {
        let _ = sqlx::query("UPDATE quote_items SET selected = true WHERE tenant_id = $1 AND quote_id = $2 AND id = $3")
            .bind(&tenant_id)
            .bind(&quote_id)
            .bind(&item_id)
            .execute(&mut *tx)
            .await;
    }

    // Automatically create a booking linked to this quote to block calendar time
    let booking_id = uuid::Uuid::new_v4().to_string();
    let _ = sqlx::query("INSERT INTO bookings (id, tenant_id, quote_id, status) VALUES ($1, $2, $3, 'Pending Deposit') ON CONFLICT DO NOTHING")
        .bind(&booking_id)
        .bind(&tenant_id)
        .bind(&quote_id)
        .execute(&mut *tx)
        .await;

    let _ = tx.commit().await;

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "checkout_url": "https://checkout.stripe.com/pay/cs_test_dummy"
    }))).into_response()
}