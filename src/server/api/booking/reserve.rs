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
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ReserveTimeSlotRequest {
    pub customer_id: String,
    pub product_id: String,
    pub start_time: String, // RFC3339
    pub end_time: String,   // RFC3339
    #[serde(default)]
    pub requires_deposit: bool,
}

#[derive(Serialize)]
pub struct ReserveTimeSlotResponse {
    pub booking_id: String,
    pub deposit_stripe_link: Option<String>,
}

pub fn router<S>(db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(reserve_time_slot))
        .with_state(db)
}

async fn reserve_time_slot(
    State(db): State<Arc<DB>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<ReserveTimeSlotRequest>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let start_time = match DateTime::parse_from_rfc3339(&payload.start_time) {
        Ok(t) => t.with_timezone(&Utc),
        Err(_) => return (StatusCode::BAD_REQUEST, axum::Json(serde_json::json!({"error": "Invalid start_time RFC3339 format"}))).into_response(),
    };
    let end_time = match DateTime::parse_from_rfc3339(&payload.end_time) {
        Ok(t) => t.with_timezone(&Utc),
        Err(_) => return (StatusCode::BAD_REQUEST, axum::Json(serde_json::json!({"error": "Invalid end_time RFC3339 format"}))).into_response(),
    };

    if end_time <= start_time {
        return (StatusCode::BAD_REQUEST, axum::Json(serde_json::json!({"error": "end_time must be after start_time"}))).into_response();
    }

    let booking_id = Uuid::new_v4().to_string();

    let mut tx = match db.pool.begin().await {
        Ok(t) => t,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    };

    if let Err(e) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": e.to_string()}))).into_response();
    }

    let overlap_count: i64 = match sqlx::query_scalar(
        "SELECT COUNT(*) FROM bookings \
         WHERE tenant_id = $1 AND product_id = $2 AND start_time < $4 AND end_time > $3 \
         AND COALESCE(status, 'pending') <> 'cancelled'"
    )
    .bind(&tenant_id)
    .bind(&payload.product_id)
    .bind(&start_time)
    .bind(&end_time)
    .fetch_one(&mut *tx)
    .await
    {
        Ok(count) => count,
        Err(e) => {
            let _ = tx.rollback().await;
            return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": e.to_string()}))).into_response();
        }
    };

    if overlap_count > 0 {
        let _ = tx.rollback().await;
        return (StatusCode::CONFLICT, axum::Json(serde_json::json!({"error": "Time slot already booked"}))).into_response();
    }

    let initial_status = if payload.requires_deposit { "pending_payment" } else { "pending" };
    let payment_intent_id = if payload.requires_deposit { Some(format!("pi_test_{}", Uuid::new_v4().to_string().replace("-", ""))) } else { None };

    if let Err(e) = sqlx::query(
        "INSERT INTO bookings (id, tenant_id, customer_id, product_id, start_time, end_time, status, payment_intent_id) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(&booking_id)
    .bind(&tenant_id)
    .bind(&payload.customer_id)
    .bind(&payload.product_id)
    .bind(start_time)
    .bind(end_time)
    .bind(initial_status)
    .bind(&payment_intent_id)
    .execute(&mut *tx)
    .await
    {
        let _ = tx.rollback().await;
        return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": e.to_string()}))).into_response();
    }

    let ledger_id = Uuid::new_v4().to_string();
    if let Err(e) = sqlx::query(
        "INSERT INTO availability_ledger (id, tenant_id, product_id, start_time, end_time, status, booking_id) \
         VALUES ($1, $2, $3, $4, $5, 'BOOKED', $6)"
    )
    .bind(&ledger_id)
    .bind(&tenant_id)
    .bind(&payload.product_id)
    .bind(start_time)
    .bind(end_time)
    .bind(&booking_id)
    .execute(&mut *tx)
    .await
    {
        let _ = tx.rollback().await;
        return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": e.to_string()}))).into_response();
    }

    if let Err(e) = tx.commit().await {
        return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": e.to_string()}))).into_response();
    }

    let deposit_stripe_link = if payload.requires_deposit {
        Some(format!("https://checkout.stripe.com/pay/cs_test_{}", booking_id.replace("-", "")))
    } else {
        None
    };

    (StatusCode::OK, axum::Json(ReserveTimeSlotResponse {
        booking_id,
        deposit_stripe_link,
    })).into_response()
}
