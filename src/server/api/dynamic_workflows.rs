use axum::{
    Json, Router,
    extract::{Extension, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
};
use serde_json::json;
use server_auth::commit_authority::{AuthorityError, verify_owner};
use server_common::{Claims, auth_utils::signed_tenant_id};
use std::sync::Arc;

use crate::orchestration::dynamic_workflows::{
    DynamicWorkflowError, DynamicWorkflowManager, DynamicWorkflowRequest,
};

#[derive(Clone)]
struct WorkflowState {
    manager: Arc<DynamicWorkflowManager>,
    auth: Arc<server_auth::Store>,
}

pub fn router<S>(manager: Arc<DynamicWorkflowManager>, auth: Arc<server_auth::Store>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(start_workflow))
        .route("/{id}", get(get_workflow))
        .route("/{id}/confirm", post(confirm_workflow))
        .with_state(WorkflowState { manager, auth })
}

async fn start_workflow(
    State(state): State<WorkflowState>,
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
    Json(request): Json<DynamicWorkflowRequest>,
) -> axum::response::Response {
    let owner = match verify_owner(&state.auth, &claims, &headers).await {
        Ok(owner) => owner,
        Err(error) => return authority_error(error),
    };
    if request.prompt.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "prompt is required" })),
        )
            .into_response();
    }

    match state.manager.start_workflow(&owner, request).await {
        Ok(start) => (StatusCode::OK, Json(json!(start))).into_response(),
        Err(error) => workflow_error(error),
    }
}

async fn confirm_workflow(
    State(state): State<WorkflowState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> axum::response::Response {
    let owner = match verify_owner(&state.auth, &claims, &headers).await {
        Ok(owner) => owner,
        Err(error) => return authority_error(error),
    };
    match state.manager.confirm_workflow(&owner, &id).await {
        Ok(start) => (StatusCode::OK, Json(json!(start))).into_response(),
        Err(error) => workflow_error(error),
    }
}

async fn get_workflow(
    State(state): State<WorkflowState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> axum::response::Response {
    let Some(tenant) = signed_tenant_id(&claims) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    match state.manager.get_workflow(&tenant, &id) {
        Ok(Some(plan)) => (StatusCode::OK, Json(json!(plan))).into_response(),
        Ok(None) => workflow_error(DynamicWorkflowError::NotFound),
        Err(error) => workflow_error(error),
    }
}

fn authority_error(error: AuthorityError) -> axum::response::Response {
    let (status, message) = match error {
        AuthorityError::Forbidden => (StatusCode::FORBIDDEN, "current owner authority is required"),
        AuthorityError::Unavailable | AuthorityError::Database(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            "owner authority is unavailable",
        ),
    };
    (status, Json(json!({ "error": message }))).into_response()
}

fn workflow_error(error: DynamicWorkflowError) -> axum::response::Response {
    let (status, message) = match error {
        DynamicWorkflowError::NotFound => (StatusCode::NOT_FOUND, "workflow not found"),
        DynamicWorkflowError::NotTriggered => (
            StatusCode::BAD_REQUEST,
            "task does not require a dynamic workflow",
        ),
        DynamicWorkflowError::Internal(error) => {
            tracing::warn!(%error, "Dynamic workflow operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "workflow operation failed",
            )
        }
    };
    (status, Json(json!({ "error": message }))).into_response()
}
