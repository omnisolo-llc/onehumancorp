use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteItinerary {
    pub id: String,
    pub tenant_id: String,
    pub staff_profile_id: String,
    pub date: NaiveDate,
    pub status: String,
    pub start_location_lat: Option<f64>,
    pub start_location_lng: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub stops: Vec<ServiceStop>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceStop {
    pub id: String,
    pub tenant_id: String,
    pub job_id: String,
    pub route_itinerary_id: String,
    pub sequence_order: i32,
    pub estimated_arrival_time: Option<DateTime<Utc>>,
    pub actual_arrival_time: Option<DateTime<Utc>>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub job: Option<FieldServiceJob>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldServiceJob {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: Option<Uuid>,
    pub booking_id: Option<String>,
    pub status: String,
    pub description: Option<String>,
    pub estimated_duration_mins: i32,
    pub location_address: String,
    pub location_lat: Option<f64>,
    pub location_lng: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/itineraries/{date}", get(get_daily_itinerary))
        .route("/stops/{stop_id}/status", put(update_stop_status))
}

pub async fn get_daily_itinerary(
    Path(_date): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = "test-tenant-id".to_string();

    let itinerary = RouteItinerary {
        id: Uuid::new_v4().to_string(),
        tenant_id,
        staff_profile_id: "test-staff".to_string(),
        date: NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
        status: "active".to_string(),
        start_location_lat: None,
        start_location_lng: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        stops: vec![],
    };

    Ok((StatusCode::OK, Json(itinerary)))
}

pub async fn update_stop_status(
    Path(_stop_id): Path<String>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::OK)
}
