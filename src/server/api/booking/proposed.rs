use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use crate::services::booking::BookingService;

#[derive(Deserialize)]
pub struct CreateProposedBookingPayload {
    pub tenant_id: Uuid,
    pub customer_id: Uuid,
    pub conversation_id: Uuid,
    pub requested_service: String,
    pub proposed_time: String,
    pub estimated_value: f64,
}

pub async fn create_proposed_booking(
    Json(payload): Json<CreateProposedBookingPayload>,
) -> impl IntoResponse {
    match BookingService::create_proposed_booking(
        payload.tenant_id,
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
            eprintln!("Failed to create proposed booking: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create booking").into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct ApproveProposedBookingPayload {
    pub tenant_id: Uuid,
}

pub async fn approve_proposed_booking(
    Path(id): Path<Uuid>,
    Json(payload): Json<ApproveProposedBookingPayload>,
) -> impl IntoResponse {
    match BookingService::approve_proposed_booking(id, payload.tenant_id).await {
        Ok(booking) => (StatusCode::OK, Json(booking)).into_response(),
        Err(e) => {
            eprintln!("Failed to approve proposed booking: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to approve booking").into_response()
        }
    }
}

pub async fn list_proposed_bookings(
    Path(tenant_id): Path<Uuid>,
) -> impl IntoResponse {
    match BookingService::get_proposed_bookings(tenant_id).await {
        Ok(bookings) => (StatusCode::OK, Json(bookings)).into_response(),
        Err(e) => {
            eprintln!("Failed to list proposed bookings: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch bookings").into_response()
        }
    }
}
use axum::{routing::post, Router};

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/", post(create_proposed_booking))
        .route("/{id}/approve", post(approve_proposed_booking))
        .route("/list/{tenant_id}", axum::routing::get(list_proposed_bookings))
}
