use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::db::DB;
use crate::auth::RequireAuth;

#[derive(Clone)]
pub struct EscrowApiState {
    pub db: Arc<DB>,
}

pub fn escrow_routes(db: Arc<DB>) -> Router {
    Router::new()
        .route("/escrow", post(handle_create_escrow))
        .route("/escrow/:escrow_id/milestone", post(handle_add_milestone))
        .route("/escrow/milestones/:milestone_id/approve", post(handle_approve_milestone))
        .layer(Extension(EscrowApiState { db }))
}

#[derive(Debug, Deserialize)]
pub struct CreateEscrowRequest {
    pub total_amount: f64,
    pub fbo_account_id: String,
}

#[derive(Debug, Deserialize)]
pub struct AddMilestoneRequest {
    pub release_amount: f64,
    pub proof_required: String,
}

async fn handle_create_escrow(
    Extension(state): Extension<EscrowApiState>,
    auth: RequireAuth,
    Json(payload): Json<CreateEscrowRequest>,
) -> impl IntoResponse {
    let tenant_id = auth.user.organization_id.clone();
    match state.db.create_escrow(&tenant_id, payload.total_amount, &payload.fbo_account_id).await {
        Ok(escrow) => Json(serde_json::json!({ "status": "success", "escrow": escrow })).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e }))).into_response(),
    }
}

async fn handle_add_milestone(
    Extension(state): Extension<EscrowApiState>,
    Path(escrow_id): Path<String>,
    auth: RequireAuth,
    Json(payload): Json<AddMilestoneRequest>,
) -> impl IntoResponse {
    let tenant_id = auth.user.organization_id.clone();
    match state.db.add_milestone(&escrow_id, &tenant_id, payload.release_amount, &payload.proof_required).await {
        Ok(milestone) => Json(serde_json::json!({ "status": "success", "milestone": milestone })).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e }))).into_response(),
    }
}

async fn handle_approve_milestone(
    Extension(state): Extension<EscrowApiState>,
    Path(milestone_id): Path<String>,
    auth: RequireAuth,
) -> impl IntoResponse {
    let tenant_id = auth.user.organization_id.clone();
    match state.db.approve_milestone(&milestone_id, &tenant_id).await {
        Ok(tx) => Json(serde_json::json!({ "status": "success", "transaction": tx })).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e }))).into_response(),
    }
}
