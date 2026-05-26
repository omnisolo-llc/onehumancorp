// API layer for Yield Management Engine
use axum::{
    extract::{State, Path},
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;
use super::engine::YieldEngine;
use uuid::Uuid;

#[derive(Clone)]
pub struct YieldAppState {
    pub engine: Arc<YieldEngine>,
}

#[derive(Deserialize)]
pub struct GetPriceRequest {
    pub tenant_id: String,
    pub base_price_cents: i64,
}

#[derive(Serialize)]
pub struct GetPriceResponse {
    pub current_price_cents: i64,
}

#[derive(Deserialize)]
pub struct ConfigureProfileRequest {
    pub tenant_id: String,
    pub target_type: String,
    pub enabled: bool,
    pub min_price_cents: i64,
    pub max_price_cents: i64,
}

#[derive(Deserialize)]
pub struct UpdateCapacityRequest {
    pub tenant_id: String,
    pub available: i64,
    pub total: i64,
}

#[derive(Deserialize)]
pub struct AddDemandSignalRequest {
    pub tenant_id: String,
    pub signal_type: String,
    pub score: f64,
}

pub fn yield_router(db: DB) -> Router {
    let state = YieldAppState {
        engine: Arc::new(YieldEngine::new(db)),
    };

    Router::new()
        .route("/api/v1/yield/:target_id/price", post(get_current_price))
        .route("/api/v1/yield/:target_id/configure", put(configure_profile))
        .route("/api/v1/yield/:target_id/capacity", post(update_capacity))
        .route("/api/v1/yield/:target_id/signal", post(add_demand_signal))
        .with_state(state)
}

async fn get_current_price(
    State(state): State<YieldAppState>,
    Path(target_id): Path<String>,
    Json(payload): Json<GetPriceRequest>,
) -> Result<Json<GetPriceResponse>, String> {
    let price = state.engine.get_current_price(&payload.tenant_id, &target_id, payload.base_price_cents).await?;
    Ok(Json(GetPriceResponse { current_price_cents: price }))
}

async fn configure_profile(
    State(state): State<YieldAppState>,
    Path(target_id): Path<String>,
    Json(payload): Json<ConfigureProfileRequest>,
) -> Result<Json<()>, String> {
    state.engine.configure_profile(&payload.tenant_id, &target_id, &payload.target_type, payload.enabled, payload.min_price_cents, payload.max_price_cents).await?;
    Ok(Json(()))
}

async fn update_capacity(
    State(state): State<YieldAppState>,
    Path(target_id): Path<String>,
    Json(payload): Json<UpdateCapacityRequest>,
) -> Result<Json<()>, String> {
    state.engine.update_capacity(&payload.tenant_id, &target_id, payload.available, payload.total).await?;
    Ok(Json(()))
}

async fn add_demand_signal(
    State(state): State<YieldAppState>,
    Path(target_id): Path<String>,
    Json(payload): Json<AddDemandSignalRequest>,
) -> Result<Json<()>, String> {
    state.engine.add_demand_signal(&payload.tenant_id, &target_id, &payload.signal_type, payload.score).await?;
    Ok(Json(()))
}
