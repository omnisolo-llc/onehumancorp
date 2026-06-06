use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;

#[derive(Deserialize)]
pub struct AddPointsRequest {
    pub points: i32,
}

pub fn router<S: Clone + Send + Sync + 'static>(
    orchestrator: Arc<DepartmentOrchestrator>,
) -> Router<S> {
    Router::new()
        .route("/{tenant_id}/customer/{customer_id}", get(get_balance))
        .route("/{tenant_id}/customer/{customer_id}/add", post(add_points))
        .route("/{tenant_id}/customer/{customer_id}/redeem", post(redeem_points))
        .with_state(orchestrator)
}

async fn get_balance(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Path((tenant_id, customer_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match orchestrator.get_loyalty_ledger(&tenant_id, &customer_id).await {
        Ok(Some(ledger)) => (StatusCode::OK, Json(ledger)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Customer loyalty record not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn add_points(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Path((tenant_id, customer_id)): Path<(String, String)>,
    Json(payload): Json<AddPointsRequest>,
) -> impl IntoResponse {
    match orchestrator.add_loyalty_points(&tenant_id, &customer_id, payload.points).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn redeem_points(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Path((tenant_id, customer_id)): Path<(String, String)>,
    Json(payload): Json<AddPointsRequest>,
) -> impl IntoResponse {
    // Check balance first
    match orchestrator.get_loyalty_ledger(&tenant_id, &customer_id).await {
        Ok(Some(ledger)) => {
            if ledger.points_balance < payload.points {
                return (StatusCode::BAD_REQUEST, "Insufficient points").into_response();
            }
        }
        Ok(None) => return (StatusCode::NOT_FOUND, "Customer loyalty record not found").into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }

    // Attempting to maintain the orchestration method logic.
    // The orchestrator has direct access to the database via add_loyalty_points.
    // In a real scenario we'd create an atomic `redeem_loyalty_points` in the orchestrator.
    // For this context, checking it immediately above is the best available with existing methods.
    match orchestrator.add_loyalty_points(&tenant_id, &customer_id, -payload.points).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}
