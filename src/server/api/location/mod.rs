use axum::{
    extract::{State, Json},
    routing::{post, get},
    Router,
};
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new()
        .route("/update", post(update_location))
        .route("/status", get(get_status).post(update_status))
}

#[derive(Deserialize)]
pub struct LocationUpdate {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub accepting_orders: bool,
    pub is_mobile: bool,
}

#[derive(Deserialize)]
pub struct StatusUpdate {
    pub accepting_orders: bool,
}

async fn update_location(Json(payload): Json<LocationUpdate>) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    Ok(axum::http::StatusCode::OK)
}

async fn get_status() -> Result<Json<StatusResponse>, axum::http::StatusCode> {
    Ok(Json(StatusResponse {
        accepting_orders: false,
        is_mobile: true,
    }))
}

async fn update_status(Json(payload): Json<StatusUpdate>) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    Ok(axum::http::StatusCode::OK)
}
