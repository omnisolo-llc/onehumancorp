use crate::db::DB;
use axum::{
    Router,
    extract::{Extension, Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
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

fn reservation_window(
    start_time: &str,
    end_time: &str,
) -> Option<(chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)> {
    let start_time = chrono::DateTime::parse_from_rfc3339(start_time)
        .ok()?
        .with_timezone(&chrono::Utc);
    let end_time = chrono::DateTime::parse_from_rfc3339(end_time)
        .ok()?
        .with_timezone(&chrono::Utc);
    (end_time > start_time && end_time - start_time <= chrono::Duration::hours(24))
        .then_some((start_time, end_time))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reservation_window_rejects_inverted_and_oversized_ranges() {
        assert!(reservation_window("2026-07-15T10:00:00Z", "2026-07-15T11:00:00Z",).is_some());
        assert!(reservation_window("2026-07-15T11:00:00Z", "2026-07-15T10:00:00Z",).is_none());
        assert!(reservation_window("2026-07-15T10:00:00Z", "2026-07-17T10:00:01Z",).is_none());
    }
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
    claims: Option<Extension<::server_common::Claims>>,
    Json(payload): Json<ReserveRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims
        .as_ref()
        .and_then(|Extension(claims)| ::server_common::auth_utils::signed_tenant_id(claims))
    {
        Some(tenant_id) => tenant_id,
        _ => {
            return (
                axum::http::StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({"error": "unauthorized"})),
            )
                .into_response();
        }
    };
    if payload.service_id.trim().is_empty() || payload.service_id.chars().count() > 128 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "invalid service_id"})),
        )
            .into_response();
    }
    let Some((st, et)) = reservation_window(&payload.start_time, &payload.end_time) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "invalid reservation window"})),
        )
            .into_response();
    };
    let c_id = match payload.customer_id.as_deref() {
        Some(customer_id) => match uuid::Uuid::parse_str(customer_id) {
            Ok(customer_id) => Some(customer_id),
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({"error": "invalid customer_id"})),
                )
                    .into_response();
            }
        },
        None => None,
    };

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

    if let Err(error) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        tracing::error!("failed to bind reservation tenant context: {error}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "internal error"})),
        )
            .into_response();
    }

    let booking_id = uuid::Uuid::new_v4().to_string();

    let price = match sqlx::query_scalar::<_, i64>(
        "SELECT price_cents FROM services WHERE id = $1 AND tenant_id = $2",
    )
    .bind(&payload.service_id)
    .bind(&tenant_id)
    .fetch_optional(&mut *tx)
    .await
    {
        Ok(Some(price)) => price,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "service not found"})),
            )
                .into_response();
        }
        Err(error) => {
            tracing::error!("failed to load reservation service: {error}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "internal error"})),
            )
                .into_response();
        }
    };

    let claimed_slot = sqlx::query(
        "UPDATE availability_blocks SET is_available = false WHERE tenant_id = $1 AND service_id = $2 AND start_time = $3 AND end_time = $4 AND is_available = true RETURNING id",
    )
    .bind(&tenant_id)
    .bind(&payload.service_id)
    .bind(st)
    .bind(et)
    .fetch_optional(&mut *tx)
    .await;
    match claimed_slot {
        Ok(Some(_)) => {}
        Ok(None) => {
            return (
                StatusCode::CONFLICT,
                Json(serde_json::json!({"error": "slot unavailable"})),
            )
                .into_response();
        }
        Err(error) => {
            tracing::error!("failed to claim reservation slot: {error}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "internal error"})),
            )
                .into_response();
        }
    }

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

    // Add feed item within the same transaction as the reservation.
    let feed_id = uuid::Uuid::new_v4().to_string();
    let feed_result = sqlx::query(
        r#"
        INSERT INTO agent_feed (id, tenant_id, event_source, lifecycle_state, context_payload)
        VALUES ($1, $2, 'booking_request', 'new', $3)
        "#,
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
    if let Err(error) = feed_result {
        tracing::error!("failed to create booking feed item: {error}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "internal error"})),
        )
            .into_response();
    }
    if let Err(error) = tx.commit().await {
        tracing::error!("failed to commit reservation: {error}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "internal error"})),
        )
            .into_response();
    }

    let checkout_url = if price > 0 {
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
