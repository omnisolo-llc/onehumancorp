use axum::{
    extract::Json,
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::services::booking::TIMESLOT_LOCK_TTL;

#[derive(Deserialize)]
pub struct ReserveRequest {
    pub tenant_id: String,
    pub customer_id: String,
    pub product_id: String,
    pub start_time: String,
    pub end_time: String,
    pub requires_deposit: bool,
}

#[derive(Serialize)]
pub struct ReserveResponse {
    pub booking_id: String,
    pub deposit_stripe_link: Option<String>,
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_reserve_time_slot))
}

async fn handle_reserve_time_slot(
    Json(payload): Json<ReserveRequest>,
) -> impl IntoResponse {
    let tenant_id = payload.tenant_id;
    let customer_id = payload.customer_id;
    let product_id = payload.product_id;
    let start_time_str = payload.start_time;
    let end_time_str = payload.end_time;

    let start_time = match DateTime::parse_from_rfc3339(&start_time_str) {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(_) => return (StatusCode::BAD_REQUEST, axum::Json(serde_json::json!({"error": "Invalid start_time RFC3339 format"}))).into_response(),
    };
    let end_time = match DateTime::parse_from_rfc3339(&end_time_str) {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(_) => return (StatusCode::BAD_REQUEST, axum::Json(serde_json::json!({"error": "Invalid end_time RFC3339 format"}))).into_response(),
    };

    if end_time <= start_time {
        return (StatusCode::BAD_REQUEST, axum::Json(serde_json::json!({"error": "end_time must be after start_time"}))).into_response();
    }

    let booking_id = Uuid::new_v4().to_string();

    let soft_locks = crate::services::booking::BookingSoftLockStore::for_service(crate::get_redis_client());
    let capacity_lock = match soft_locks
        .acquire_capacity_lock(
            &tenant_id,
            &product_id,
            start_time,
            end_time,
            &booking_id,
            TIMESLOT_LOCK_TTL,
        )
        .await {
            Ok(Some(lock)) => lock,
            Ok(None) => return (StatusCode::CONFLICT, axum::Json(serde_json::json!({"error": "Time slot is currently being held by another request"}))).into_response(),
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "lock_error"}))).into_response(),
        };

    let pool = crate::db::get_pool();
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(_) => {
            let _ = soft_locks.release(&capacity_lock).await;
            return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "db_error"}))).into_response();
        }
    };

    if let Err(_) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        let _ = soft_locks.release(&capacity_lock).await;
        return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "auth_error"}))).into_response();
    }

    let overlap_count: i64 = match sqlx::query_scalar(
        "SELECT COUNT(*) FROM bookings \
         WHERE tenant_id = $1 AND product_id = $2 AND start_time < $4 AND end_time > $3 \
         AND COALESCE(status, 'pending') <> 'cancelled'"
    )
    .bind(&tenant_id)
    .bind(&product_id)
    .bind(&start_time)
    .bind(&end_time)
    .fetch_one(&mut *tx)
    .await {
        Ok(count) => count,
        Err(_) => {
            let _ = tx.rollback().await;
            let _ = soft_locks.release(&capacity_lock).await;
            return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "db_query_error"}))).into_response();
        }
    };

    if overlap_count > 0 {
        let _ = tx.rollback().await;
        let _ = soft_locks.release(&capacity_lock).await;
        return (StatusCode::CONFLICT, axum::Json(serde_json::json!({"error": "Time slot already booked"}))).into_response();
    }

    let initial_status = if payload.requires_deposit { "pending_payment" } else { "pending" };
    let payment_intent_id = if payload.requires_deposit { Some(format!("pi_test_{}", Uuid::new_v4().to_string().replace("-", ""))) } else { None };

    if let Err(_) = sqlx::query(
        "INSERT INTO bookings (id, tenant_id, customer_id, product_id, start_time, end_time, status, payment_intent_id) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
    )
    .bind(&booking_id)
    .bind(&tenant_id)
    .bind(&customer_id)
    .bind(&product_id)
    .bind(start_time)
    .bind(end_time)
    .bind(initial_status)
    .bind(&payment_intent_id)
    .execute(&mut *tx)
    .await {
        let _ = tx.rollback().await;
        let _ = soft_locks.release(&capacity_lock).await;
        return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "db_insert_error"}))).into_response();
    }

    if let Err(_) = sqlx::query(
        "INSERT INTO availability_ledger (id, tenant_id, product_id, start_time, end_time, status, booking_id) \
         VALUES ($1, $2, $3, $4, $5, 'BOOKED', $6)"
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&tenant_id)
    .bind(&product_id)
    .bind(start_time)
    .bind(end_time)
    .bind(&booking_id)
    .execute(&mut *tx)
    .await {
        let _ = tx.rollback().await;
        let _ = soft_locks.release(&capacity_lock).await;
        return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "db_insert_ledger_error"}))).into_response();
    }

    let _ = tx.commit().await;
    let _ = soft_locks.release(&capacity_lock).await;

    let deposit_stripe_link = if payload.requires_deposit {
        Some(format!("https://checkout.stripe.com/pay/cs_test_{}", Uuid::new_v4().to_string().replace("-", "")))
    } else {
        None
    };

    (StatusCode::OK, axum::Json(ReserveResponse {
        booking_id,
        deposit_stripe_link,
    })).into_response()
}
