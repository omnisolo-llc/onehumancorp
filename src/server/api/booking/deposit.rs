use axum::{
    extract::{State, Query},
    response::{IntoResponse, Redirect},
    http::StatusCode,
    routing::get,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::db::DB;

#[derive(Deserialize)]
pub struct DepositQuery {
    pub booking_id: String,
    pub tenant_id: Option<String>,
}

pub fn router<S>(db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(handle_deposit_session))
        .with_state(db)
}

async fn handle_deposit_session(
    State(db): State<Arc<DB>>,
    Query(query): Query<DepositQuery>,
) -> impl IntoResponse {
    let tenant_id = match query.tenant_id {
        Some(t) if !t.trim().is_empty() => t,
        _ => return (StatusCode::BAD_REQUEST, "Missing tenant_id parameter").into_response(),
    };

    let pool = db.pool.clone();

    // Verify booking
    let booking_res = sqlx::query(
        r#"
        SELECT b.id, b.service_id, b.status, s.requires_deposit, s.deposit_amount_cents
        FROM bookings b
        JOIN services s ON b.service_id = s.id
        WHERE b.id = $1 AND b.tenant_id = $2
        "#,
    )
    .bind(&query.booking_id)
    .bind(&tenant_id)
    .fetch_one(&pool)
    .await;

    let booking = match booking_res {
        Ok(b) => b,
        Err(_) => {
            return (StatusCode::NOT_FOUND, "booking not found").into_response();
        }
    };

    use sqlx::Row;
    let requires_deposit: bool = booking.get("requires_deposit");
    let deposit_amount_cents: i64 = booking.get("deposit_amount_cents");
    let status: String = booking.get("status");
    let booking_id: String = booking.get("id");

    if !requires_deposit || deposit_amount_cents == 0 {
        return (StatusCode::BAD_REQUEST, "no deposit required for this booking").into_response();
    }

    if status != "pending_payment" {
        return (StatusCode::BAD_REQUEST, format!("booking is not in pending_payment state, current: {}", status)).into_response();
    }

    // Generate real checkout session or internal payment intent. For our backend this
    // creates an intent identifier
    let payment_intent_id = format!("pi_{}", uuid::Uuid::new_v4());
    let checkout_url = format!("/checkout/session/{}", payment_intent_id);

    // Save payment intent id on booking
    let result = sqlx::query(
        "UPDATE bookings SET payment_intent_id = $1 WHERE id = $2 AND tenant_id = $3",
    )
    .bind(&payment_intent_id)
    .bind(&booking_id)
    .bind(&tenant_id)
    .execute(&pool)
    .await;

    if let Err(e) = result {
        tracing::error!("Failed to update booking with payment intent: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "failed to initiate deposit checkout").into_response();
    }

    Redirect::temporary(&checkout_url).into_response()
}
