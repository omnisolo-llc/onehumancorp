use crate::services::booking::BookingService;
use axum::{
    Json,
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateProposedBookingPayload {
    pub customer_id: Uuid,
    pub conversation_id: Uuid,
    pub requested_service: String,
    pub proposed_time: String,
    pub estimated_value: f64,
}

pub async fn create_proposed_booking(
    claims: Option<Extension<::server_common::Claims>>,
    Json(payload): Json<CreateProposedBookingPayload>,
) -> impl IntoResponse {
    let Some(tenant_id) = claims
        .as_ref()
        .and_then(|Extension(claims)| ::server_common::auth_utils::signed_tenant_id(claims))
        .and_then(|tenant_id| Uuid::parse_str(&tenant_id).ok())
    else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    if payload.requested_service.trim().is_empty()
        || payload.requested_service.chars().count() > 200
        || !payload.estimated_value.is_finite()
        || !(0.0..=10_000_000.0).contains(&payload.estimated_value)
    {
        return StatusCode::BAD_REQUEST.into_response();
    }
    match BookingService::create_proposed_booking(
        tenant_id,
        payload.customer_id,
        payload.conversation_id,
        payload.requested_service,
        payload.proposed_time,
        payload.estimated_value,
    )
    .await
    {
        Ok(booking) => (StatusCode::CREATED, Json(booking)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create proposed booking: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create booking",
            )
                .into_response()
        }
    }
}

pub async fn approve_proposed_booking(
    claims: Option<Extension<::server_common::Claims>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let Some(tenant_id) = claims
        .as_ref()
        .and_then(|Extension(claims)| ::server_common::auth_utils::signed_tenant_id(claims))
        .and_then(|tenant_id| Uuid::parse_str(&tenant_id).ok())
    else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    match BookingService::approve_proposed_booking(id, tenant_id).await {
        Ok(booking) => (StatusCode::OK, Json(booking)).into_response(),
        Err(e) => {
            tracing::error!("Failed to approve proposed booking: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to approve booking",
            )
                .into_response()
        }
    }
}

pub async fn list_proposed_bookings(
    claims: Option<Extension<::server_common::Claims>>,
) -> impl IntoResponse {
    let Some(tenant_id) = claims
        .as_ref()
        .and_then(|Extension(claims)| ::server_common::auth_utils::signed_tenant_id(claims))
        .and_then(|tenant_id| Uuid::parse_str(&tenant_id).ok())
    else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    match BookingService::get_proposed_bookings(tenant_id).await {
        Ok(bookings) => (StatusCode::OK, Json(bookings)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list proposed bookings: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch bookings",
            )
                .into_response()
        }
    }
}
use axum::{Router, routing::post};

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/", post(create_proposed_booking))
        .route("/{id}/approve", post(approve_proposed_booking))
        .route("/list", axum::routing::get(list_proposed_bookings))
}
