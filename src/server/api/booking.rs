use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct BookingRequestPayload {
    pub description: Option<String>,
    pub service_id: Option<String>,
    pub start_time: Option<String>, // RFC3339
}

#[derive(Serialize)]
pub struct BookingResponsePayload {
    pub success: bool,
    pub booking_id: Option<String>,
    pub message: Option<String>,
}

pub async fn create_booking_request(
    State(db): State<Arc<crate::db::DB>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<BookingRequestPayload>,
) -> axum::response::Response {
    use axum::response::IntoResponse;

    // Auth validation check - enforce via middleware or manual validation.
    // OHC platform requires valid session/SPIFFE ID typically,
    // but here we check typical auth_utils headers to enforce context
    let tenant_id = headers
        .get("x-tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default")
        .to_string();

    let customer_id = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("default")
        .to_string();

    // For local dev / e2e we accept simple IDs, but in production this should be strongly validated.
    if tenant_id == "default" {
        return (
            axum::http::StatusCode::UNAUTHORIZED,
            Json(BookingResponsePayload {
                success: false,
                booking_id: None,
                message: Some("Missing or invalid tenant authentication".to_string()),
            }),
        ).into_response();
    }

    let booking_id = Uuid::new_v4().to_string();
    let start_time = payload.start_time.clone().unwrap_or_else(|| chrono::Utc::now().to_rfc3339());
    let service_id = payload.service_id.clone().unwrap_or_else(|| "default-service".to_string());

    let start_time_parsed = match chrono::DateTime::parse_from_rfc3339(&start_time) {
        Ok(t) => t.with_timezone(&chrono::Utc),
        Err(_) => return (
            axum::http::StatusCode::BAD_REQUEST,
            Json(BookingResponsePayload {
                success: false,
                booking_id: None,
                message: Some("Invalid start_time format".to_string()),
            }),
        )
            .into_response(),
    };

    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(BookingResponsePayload {
                    success: false,
                    booking_id: None,
                    message: Some("Internal server error".to_string()),
                }),
            )
            .into_response();
        }
    };

    // Enforce RLS context
    if let Err(e) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        tracing::error!("Failed to set RLS context: {}", e);
        return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(BookingResponsePayload {
                success: false,
                booking_id: None,
                message: Some("Internal server error".to_string()),
            }),
        )
        .into_response();
    }

    match sqlx::query!(
        "INSERT INTO bookings (id, tenant_id, customer_id, service_id, start_time, status) VALUES ($1, $2, $3, $4, $5, 'pending')",
        booking_id,
        tenant_id,
        customer_id,
        service_id,
        start_time_parsed
    )
    .execute(&mut *tx)
    .await
    {
        Ok(_) => {
            let _ = tx.commit().await;
            (
                axum::http::StatusCode::OK,
                Json(BookingResponsePayload {
                    success: true,
                    booking_id: Some(booking_id),
                    message: Some("Booking created".to_string()),
                }),
            ).into_response()
        },
        Err(e) => {
            let _ = tx.rollback().await;
            tracing::error!("Failed to insert booking: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(BookingResponsePayload {
                    success: false,
                    booking_id: None,
                    message: Some("Internal server error".to_string()),
                }),
            ).into_response()
        }
    }
}
