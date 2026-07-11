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
use crate::orchestration::queue::redis_lock::RedisLock;

#[derive(Deserialize)]
pub struct ReserveRequest {
    pub customer_id: Option<String>,
    pub service_id: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Serialize)]
pub struct ReserveResponse {
    pub success: bool,
    pub booking_id: Option<String>,
    pub error: Option<String>,
    pub checkout_url: Option<String>,
}

pub fn router<S>(db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_reserve))
        .with_state(db)
}

async fn handle_reserve(
    State(db): State<Arc<DB>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<ReserveRequest>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (axum::http::StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let redis_url = std::env::var("OHC_REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
    if let Ok(redis_lock) = RedisLock::new(&redis_url) {
        let slot_time_id = format!("{}_{}_{}", payload.service_id, payload.start_time, payload.end_time);
        if let Ok(None) = redis_lock.acquire_lock(&tenant_id, "booking_slot", &slot_time_id, 300).await {
            return (
                StatusCode::CONFLICT,
                Json(ReserveResponse {
                    success: false,
                    booking_id: None,
                    error: Some("Failed to reserve booking slot: Time slot is currently being held by another request.".to_string()),
                    checkout_url: None,
                }),
            ).into_response();
        }
    }

    let pool = db.pool.clone();

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("failed to begin tx: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ReserveResponse {
                    success: false,
                    booking_id: None,
                    error: Some("internal error".to_string()),
                    checkout_url: None,
                }),
            )
                .into_response();
        }
    };

    let _ = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await;

    let st = match chrono::DateTime::parse_from_rfc3339(&payload.start_time) {
        Ok(d) => d.with_timezone(&chrono::Utc),
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ReserveResponse {
                    success: false,
                    booking_id: None,
                    error: Some("invalid start_time format".to_string()),
                    checkout_url: None,
                }),
            )
                .into_response();
        }
    };

    let et = match chrono::DateTime::parse_from_rfc3339(&payload.end_time) {
        Ok(d) => d.with_timezone(&chrono::Utc),
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ReserveResponse {
                    success: false,
                    booking_id: None,
                    error: Some("invalid end_time format".to_string()),
                    checkout_url: None,
                }),
            )
                .into_response();
        }
    };

    let booking_id = uuid::Uuid::new_v4().to_string();

    let c_id = payload.customer_id.and_then(|c| uuid::Uuid::parse_str(&c).ok());

    let res = sqlx::query(
        r#"
        INSERT INTO bookings (id, tenant_id, customer_id, service_id, start_time, end_time, status)
        VALUES ($1, $2, $3, $4, $5, $6, 'pending')
        "#,
    )
    .bind(&booking_id)
    .bind(&tenant_id)
    .bind(&c_id)
    .bind(&payload.service_id)
    .bind(st)
    .bind(et)
    .execute(&mut *tx)
    .await;

    if let Err(e) = res {
        tracing::error!("failed to insert booking: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ReserveResponse {
                success: false,
                booking_id: None,
                error: Some("failed to create booking".to_string()),
                checkout_url: None,
            }),
        )
            .into_response();
    }

    // Attempt to mark block as booked
    let _ = sqlx::query(
        r#"
        UPDATE availability_blocks SET is_available = false
        WHERE tenant_id = $1 AND service_id = $2 AND start_time = $3 AND end_time = $4
        "#,
    )
    .bind(&tenant_id)
    .bind(&payload.service_id)
    .bind(st)
    .bind(et)
    .execute(&mut *tx)
    .await;


    // Add feed item
    let feed_id = uuid::Uuid::new_v4().to_string();
    let _ = sqlx::query(
        r#"
        INSERT INTO agent_feed (id, tenant_id, event_source, lifecycle_state, context_payload)
        VALUES ($1, $2, 'booking_request', 'new', $3)
        "#
    )
    .bind(&feed_id)
    .bind(&tenant_id)
    .bind(serde_json::json!({
        "booking_id": booking_id,
        "service_id": payload.service_id,
        "start_time": payload.start_time,
        "end_time": payload.end_time
    }))
    .execute(&mut *tx)
    .await;

    let _ = tx.commit().await;



    // Check if deposit is required
    let mut requires_deposit = false;
    if let Ok(Some(price)) = sqlx::query_scalar::<_, i64>(
        "SELECT price_cents FROM services WHERE id = $1 AND tenant_id = $2"
    )
    .bind(&payload.service_id)
    .bind(&tenant_id)
    .fetch_optional(&pool)
    .await
    {
        if price > 0 {
            requires_deposit = true;
        }
    }


    let checkout_url = if requires_deposit {
        Some(format!("/api/v1/booking/deposit?booking_id={}", booking_id))
    } else {
        None
    };

    (
        StatusCode::OK,
        Json(ReserveResponse {
            success: true,
            booking_id: Some(booking_id),
            error: None,
            checkout_url,
        }),
    )
        .into_response()
}
