use axum::{
    extract::{Json},
    routing::{get},
    Router,
};
use crate::services::booking::BookingService;
use crate::services::booking::{BookingRecord, Service};

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/services", get(list_services).post(create_service))
        .route("/records", get(list_bookings).post(create_booking))
}

async fn list_services(headers: axum::http::HeaderMap) -> Result<Json<Vec<Service>>, axum::http::StatusCode> {

    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok());
    let token = match auth_header {
        Some(h) if h.to_lowercase().starts_with("bearer ") => &h[7..],
        _ => return Err(axum::http::StatusCode::UNAUTHORIZED),
    };
    let store = crate::auth::Store::new();
    let claims = match store.validate_token(token).await {
        Ok(c) => c,
        Err(_) => return Err(axum::http::StatusCode::UNAUTHORIZED),
    };
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default_tenant".to_string());

    match BookingService::list_services(&tenant_id).await {
        Ok(services) => Ok(Json(services)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_service(
    headers: axum::http::HeaderMap,
    Json(payload): Json<Service>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {

    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok());
    let token = match auth_header {
        Some(h) if h.to_lowercase().starts_with("bearer ") => &h[7..],
        _ => return Err(axum::http::StatusCode::UNAUTHORIZED),
    };
    let store = crate::auth::Store::new();
    let claims = match store.validate_token(token).await {
        Ok(c) => c,
        Err(_) => return Err(axum::http::StatusCode::UNAUTHORIZED),
    };
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default_tenant".to_string());


    let mut service = payload;
    service.tenant_id = tenant_id.to_string();

    match BookingService::upsert_service(service).await {
        Ok(_) => Ok(axum::http::StatusCode::CREATED),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn list_bookings(headers: axum::http::HeaderMap) -> Result<Json<Vec<BookingRecord>>, axum::http::StatusCode> {

    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok());
    let token = match auth_header {
        Some(h) if h.to_lowercase().starts_with("bearer ") => &h[7..],
        _ => return Err(axum::http::StatusCode::UNAUTHORIZED),
    };
    let store = crate::auth::Store::new();
    let claims = match store.validate_token(token).await {
        Ok(c) => c,
        Err(_) => return Err(axum::http::StatusCode::UNAUTHORIZED),
    };
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default_tenant".to_string());

    match BookingService::get_bookings(&tenant_id).await {
        Ok(bookings) => Ok(Json(bookings)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_booking(
    headers: axum::http::HeaderMap,
    Json(payload): Json<BookingRecord>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {

    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok());
    let token = match auth_header {
        Some(h) if h.to_lowercase().starts_with("bearer ") => &h[7..],
        _ => return Err(axum::http::StatusCode::UNAUTHORIZED),
    };
    let store = crate::auth::Store::new();
    let claims = match store.validate_token(token).await {
        Ok(c) => c,
        Err(_) => return Err(axum::http::StatusCode::UNAUTHORIZED),
    };
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default_tenant".to_string());


    let mut booking = payload;
    booking.tenant_id = tenant_id.to_string();

    match BookingService::create_booking(booking).await {
        Ok(_) => Ok(axum::http::StatusCode::CREATED),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
