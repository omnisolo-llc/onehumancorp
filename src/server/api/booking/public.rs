use axum::{
    extract::{State, Json, Query},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::{DB, DbStore};
use chrono::{DateTime, Utc};
use sqlx::Row;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DB>,
}

#[derive(Deserialize)]
pub struct AvailableSlotsQuery {
    pub service_id: String,
    pub date: String, // YYYY-MM-DD
}

#[derive(Serialize, Deserialize)]
pub struct CreateBookingPayload {
    pub service_id: String,
    pub resource_id: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub customer_name: String,
    pub customer_email: String,
}

pub fn router<S>(db: Arc<DB>) -> Router<S> where S: Clone + Send + Sync + 'static, {
    let state = AppState { db };
    Router::new()
        .route("/slots", get(get_available_slots))
        .route("/checkout", post(create_checkout_session))
        .with_state(state)
}

async fn get_available_slots(
    State(_state): State<AppState>,
    headers: axum::http::HeaderMap,
    Query(query): Query<AvailableSlotsQuery>,
) -> impl IntoResponse {
    let _tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    // simplified mock logic returning available slots for demo
    // normally this would do a complex join between services (duration), resources, availability_blocks, and existing bookings

    let slots = vec![
        serde_json::json!({"start_time": format!("{}T09:00:00Z", query.date), "end_time": format!("{}T10:00:00Z", query.date)}),
        serde_json::json!({"start_time": format!("{}T11:00:00Z", query.date), "end_time": format!("{}T12:00:00Z", query.date)}),
        serde_json::json!({"start_time": format!("{}T14:00:00Z", query.date), "end_time": format!("{}T15:00:00Z", query.date)}),
    ];

    (StatusCode::OK, Json(serde_json::json!({"slots": slots}))).into_response()
}

async fn create_checkout_session(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<CreateBookingPayload>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    // 1. Fetch service info to get deposit requirements
    let service_res = match &state.db.store {
        DbStore::Sqlite(pool) => {
            sqlx::query("SELECT id, requires_deposit, deposit_amount_cents FROM services WHERE id = ? AND tenant_id = ?")
                .bind(&payload.service_id).bind(&tenant_id)
                .fetch_optional(pool).await
        }
        DbStore::Postgres(pool) => {
            let mut tx = pool.begin().await.unwrap();
            let _ = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;
            let result = sqlx::query("SELECT id, requires_deposit, deposit_amount_cents FROM services WHERE id = $1 AND tenant_id = $2")
                .bind(&payload.service_id).bind(&tenant_id)
                .fetch_optional(&mut *tx).await;
            let _ = tx.commit().await;
            result
        }
    };

    let (requires_deposit, deposit_cents) = match service_res {
        Ok(Some(s)) => {
            // Need to handle missing columns for sqlite grace
            let req_dep: bool = s.try_get("requires_deposit").unwrap_or(false);
            let dep_amt: i64 = s.try_get("deposit_amount_cents").unwrap_or(0);
            (req_dep, dep_amt)
        }
        Ok(None) => return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "service not found"}))).into_response(),
        Err(e) => {
            tracing::error!("DB error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db error"}))).into_response();
        }
    };

    // 2. Either create booking directly or generate a Stripe Checkout Session
    let booking_id = uuid::Uuid::new_v4().to_string();
    let mut stripe_url = None;
    let st = chrono::DateTime::parse_from_rfc3339(&payload.start_time).unwrap();
    let et = chrono::DateTime::parse_from_rfc3339(&payload.end_time).unwrap();

    let res = match &state.db.store {
        DbStore::Sqlite(pool) => {
            if requires_deposit && deposit_cents > 0 {
                stripe_url = Some(format!("https://checkout.stripe.com/pay/cs_test_{}", booking_id));
                sqlx::query("INSERT INTO bookings (id, tenant_id, service_id, resource_id, start_time, end_time, status) VALUES (?, ?, ?, ?, ?, ?, 'pending')")
                    .bind(&booking_id).bind(&tenant_id).bind(&payload.service_id).bind(&payload.resource_id).bind(&st.to_rfc3339()).bind(&et.to_rfc3339())
                    .execute(pool).await
            } else {
                sqlx::query("INSERT INTO bookings (id, tenant_id, service_id, resource_id, start_time, end_time, status) VALUES (?, ?, ?, ?, ?, ?, 'scheduled')")
                    .bind(&booking_id).bind(&tenant_id).bind(&payload.service_id).bind(&payload.resource_id).bind(&st.to_rfc3339()).bind(&et.to_rfc3339())
                    .execute(pool).await
            }
        }
        DbStore::Postgres(pool) => {
            let mut tx = pool.begin().await.unwrap();
            let _ = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;

            let result = if requires_deposit && deposit_cents > 0 {
                stripe_url = Some(format!("https://checkout.stripe.com/pay/cs_test_{}", booking_id));
                sqlx::query("INSERT INTO bookings (id, tenant_id, service_id, resource_id, start_time, end_time, status) VALUES ($1, $2, $3, $4, $5, $6, 'pending')")
                    .bind(&booking_id).bind(&tenant_id).bind(&payload.service_id).bind(&payload.resource_id).bind(st).bind(et)
                    .execute(&mut *tx).await
            } else {
                sqlx::query("INSERT INTO bookings (id, tenant_id, service_id, resource_id, start_time, end_time, status) VALUES ($1, $2, $3, $4, $5, $6, 'scheduled')")
                    .bind(&booking_id).bind(&tenant_id).bind(&payload.service_id).bind(&payload.resource_id).bind(st).bind(et)
                    .execute(&mut *tx).await
            };

            let _ = tx.commit().await;
            result
        }
    };

    if let Err(e) = res {
        tracing::error!("Failed to save booking: {}", e);
    }

    (StatusCode::OK, Json(serde_json::json!({
        "booking_id": booking_id,
        "stripe_url": stripe_url,
        "status": if stripe_url.is_some() { "pending_payment" } else { "confirmed" }
    }))).into_response()
}
