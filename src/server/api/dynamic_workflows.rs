use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde_json::json;
use std::sync::Arc;

use crate::orchestration::dynamic_workflows::{DynamicWorkflowManager, DynamicWorkflowRequest};

pub fn router<S>(manager: Arc<DynamicWorkflowManager>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(start_workflow))
        .route("/{id}", get(get_workflow))
        .route("/{id}/confirm", post(confirm_workflow))
        .with_state(manager)
}

async fn start_workflow(
    State(manager): State<Arc<DynamicWorkflowManager>>,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
    Json(mut request): Json<DynamicWorkflowRequest>,
) -> axum::response::Response {
    if request.prompt.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "prompt is required" })),
        )
            .into_response();
    }

    // OVERRIDE the request body's tenant_id with the one from the authenticated session
    // to prevent multi-tenant safety issue where tenant_id is read from request body
    if !auth_info.spiffe_id.is_empty() {
        request.tenant_id = auth_info.spiffe_id;
    } else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "missing tenant identity in session" })),
        )
            .into_response();
    }

    match manager.start_workflow(request).await {
        Ok(start) => (StatusCode::OK, Json(json!(start))).into_response(),
        Err(error) => (StatusCode::BAD_REQUEST, Json(json!({ "error": error }))).into_response(),
    }
}

async fn confirm_workflow(
    State(manager): State<Arc<DynamicWorkflowManager>>,
    Path(id): Path<String>,
) -> axum::response::Response {
    match manager.confirm_workflow(&id).await {
        Ok(start) => (StatusCode::OK, Json(json!(start))).into_response(),
        Err(error) => (StatusCode::NOT_FOUND, Json(json!({ "error": error }))).into_response(),
    }
}

async fn get_workflow(
    State(manager): State<Arc<DynamicWorkflowManager>>,
    Path(id): Path<String>,
) -> axum::response::Response {
    match manager.get_workflow(&id) {
        Ok(Some(plan)) => (StatusCode::OK, Json(json!(plan))).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "workflow not found" })),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": error })),
        )
            .into_response(),
    }
}
