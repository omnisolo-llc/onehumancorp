use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::{get},
    Router,
};
use serde::{Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::services::booking::{BookingService, Service, BookingRecord};
use crate::common::Claims;

#[derive(Deserialize)]
pub struct CreateServiceRequest {
    pub title: String,
    pub description: Option<String>,
    pub price_cents: i64,
}

#[derive(Deserialize)]
pub struct CreateBookingRequest {
    pub product_id: String,
    pub customer_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/services", get(list_services).post(create_service))
        .route("/", get(list_bookings).post(create_booking))
}

async fn list_services(
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(vec![])).into_response(),
    };

    match BookingService::list_services(&tenant_id).await {
        Ok(services) => (StatusCode::OK, Json(services)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])).into_response(),
    }
}

async fn create_service(
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateServiceRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let service = Service {
        id: Uuid::new_v4().to_string(),
        tenant_id,
        title: payload.title,
        description: payload.description,
        price_cents: payload.price_cents,
    };

    match BookingService::upsert_service(service).await {
        Ok(_) => (StatusCode::CREATED, "Service created").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create service").into_response(),
    }
}

async fn list_bookings(
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(vec![])).into_response(),
    };

    match BookingService::get_bookings(&tenant_id).await {
        Ok(bookings) => (StatusCode::OK, Json(bookings)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])).into_response(),
    }
}

async fn create_booking(
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateBookingRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let booking = BookingRecord {
        id: Uuid::new_v4().to_string(),
        tenant_id,
        customer_id: payload.customer_id,
        product_id: payload.product_id,
        start_time: payload.start_time,
        end_time: payload.end_time,
        status: "confirmed".to_string(),
    };

    match BookingService::create_booking(booking).await {
        Ok(_) => (StatusCode::CREATED, "Booking created").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create booking").into_response(),
    }
}
