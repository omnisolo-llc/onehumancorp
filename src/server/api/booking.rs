use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::PgPool;

#[derive(Clone, Serialize, Deserialize)]
pub struct Booking {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: String,
    pub product_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreateBookingRequest {
    pub tenant_id: String,
    pub customer_id: String,
    pub product_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: String,
}

pub struct AppState {
    pub db: PgPool,
}

pub fn router() -> Router {
    Router::new()
        .route("/api/booking", post(create_booking).get(list_bookings))
}

async fn create_booking(Json(payload): Json<CreateBookingRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // In a real app we'd insert this into a DB
    // For this mock, we just return the object with an ID
    let new_booking = Booking {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id,
        customer_id: payload.customer_id,
        product_id: payload.product_id,
        start_time: payload.start_time,
        end_time: payload.end_time,
        status: payload.status,
    };

    Ok((StatusCode::CREATED, Json(new_booking)))
}

async fn list_bookings() -> Result<impl IntoResponse, StatusCode> {
    // Return empty list for now
    let bookings: Vec<Booking> = vec![];
    Ok((StatusCode::OK, Json(bookings)))
}
