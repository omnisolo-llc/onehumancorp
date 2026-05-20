use axum::{
    extract::{Extension, State, Path, Query},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::sip::SipDB;
use ::server_common::Claims;

#[derive(Serialize)]
pub struct ApprovalsResponse {
    pub pending_approvals: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct DecisionRequest {
    pub approved: bool,
}

#[derive(Serialize)]
pub struct DecisionResponse {
    pub success: bool,
}

pub fn router<S>(sip_db: Arc<SipDB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list_approvals))
        .route("/{id}", post(decide_approval))
        .with_state(sip_db)
}

async fn list_approvals(
    State(sip_db): State<Arc<SipDB>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(ApprovalsResponse { pending_approvals: vec![] })).into_response(),
    };

    match sip_db.get_pending_approvals(&tenant_id).await {
        Ok(approvals) => (StatusCode::OK, Json(ApprovalsResponse { pending_approvals: approvals })).into_response(),
        Err(e) => {
            tracing::error!("Failed to get pending approvals: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApprovalsResponse { pending_approvals: vec![] })).into_response()
        }
    }
}

async fn decide_approval(
    State(sip_db): State<Arc<SipDB>>,
    Path(id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<DecisionRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(DecisionResponse { success: false })).into_response(),
    };

    match sip_db.decide_approval(&id, &tenant_id, payload.approved).await {
        Ok(_) => (StatusCode::OK, Json(DecisionResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to decide approval: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DecisionResponse { success: false })).into_response()
        }
    }
}
