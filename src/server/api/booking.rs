use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::services::booking::{BookingService, Service, BookingRecord};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpsertServiceRequest {
    pub title: String,
    pub description: Option<String>,
    pub price_cents: i64,
    pub duration_minutes: i64,
    pub availability: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBookingRequest {
    pub customer_id: String,
    pub product_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

pub fn router() -> Router {
    Router::new()
        .route("/services", get(list_services).post(upsert_service))
        .route("/bookings", get(list_bookings).post(create_booking))
}

fn get_tenant_from_headers(headers: &axum::http::HeaderMap) -> String {
    headers.get("x-tenant-id")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("default")
        .to_string()
}

async fn list_services(headers: axum::http::HeaderMap) -> impl IntoResponse {
    let tenant_id = get_tenant_from_headers(&headers);
    match BookingService::list_services(&tenant_id).await {
        Ok(services) => (StatusCode::OK, Json(services)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn upsert_service(
    headers: axum::http::HeaderMap,
    Json(req): Json<UpsertServiceRequest>
) -> impl IntoResponse {
    let tenant_id = get_tenant_from_headers(&headers);
    let service = Service {
        id: Uuid::new_v4().to_string(),
        tenant_id,
        title: req.title,
        description: req.description,
        price_cents: req.price_cents,
        duration_minutes: req.duration_minutes,
        availability: req.availability,
    };

    match BookingService::upsert_service(service.clone()).await {
        Ok(_) => (StatusCode::CREATED, Json(service)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn list_bookings(headers: axum::http::HeaderMap) -> impl IntoResponse {
    let tenant_id = get_tenant_from_headers(&headers);
    match BookingService::get_bookings(&tenant_id).await {
        Ok(bookings) => (StatusCode::OK, Json(bookings)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn create_booking(
    headers: axum::http::HeaderMap,
    Json(req): Json<CreateBookingRequest>
) -> impl IntoResponse {
    let tenant_id = get_tenant_from_headers(&headers);
    let booking = BookingRecord {
        id: Uuid::new_v4().to_string(),
        tenant_id,
        customer_id: req.customer_id,
        product_id: req.product_id,
        start_time: req.start_time,
        end_time: req.end_time,
        status: "scheduled".to_string(),
    };

    match BookingService::create_booking(booking.clone()).await {
        Ok(_) => (StatusCode::CREATED, Json(booking)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}
