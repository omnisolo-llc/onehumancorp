use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use crate::orchestration::tasks::TaskDecompositionService;
use crate::tasks::SharedTask;

#[derive(Deserialize)]
pub struct CreateTaskPayload {
    pub mission_id: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
}

#[derive(Deserialize)]
pub struct PollTasksQuery {
    pub agent_id: String,
}

#[derive(Deserialize)]
pub struct UpdateTaskStatusPayload {
    pub status: String,
    pub agent_id: String,
    pub result: Option<String>,
}

// Ensure proper multi-tenant scoping!
pub async fn create_task_handler(
    State(service): State<Arc<TaskDecompositionService>>,
    Extension(user): Extension<crate::common::Claims>,
    Json(payload): Json<CreateTaskPayload>,
) -> impl IntoResponse {
    let task = SharedTask {
        id: format!("task-{}", uuid::Uuid::new_v4()),
        organization_id: user.tenant_id.clone(), // Proper Multi-Tenant Scoping!
        mission_id: payload.mission_id,
        parent_plan_id: "".to_string(),
        dependencies: vec![],
        title: payload.title,
        description: payload.description,
        assigned_agent_id: None,
        status: "PENDING".to_string(),
        priority: payload.priority,
        payload: "{}".to_string(),
        locked_until: None,
        ultraplan_phase: None,
        deliberation_log: None,
        depth: Some(0),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        action_risk: None,
        approval_status: None,
        proposed_content: None,
    };

    match service.create_task(task).await {
        Ok(created_task) => (StatusCode::CREATED, Json(created_task)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

pub async fn poll_tasks_handler(
    State(service): State<Arc<TaskDecompositionService>>,
    Query(query): Query<PollTasksQuery>,
) -> impl IntoResponse {
    // Agents claim tasks globally based on permissions (agent_id scopes appropriately via the service layer)
    match service.claim_task(&query.agent_id).await {
        Ok(Some(task)) => (StatusCode::OK, Json(task)).into_response(),
        Ok(None) => (StatusCode::OK, Json(serde_json::json!(null))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

pub async fn update_task_status_handler(
    State(service): State<Arc<TaskDecompositionService>>,
    Path(task_id): Path<String>,
    Json(payload): Json<UpdateTaskStatusPayload>,
) -> impl IntoResponse {
    match service.update_status(&task_id, &payload.status, &payload.agent_id).await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"status": "success"}))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

pub fn router<S: Clone + Send + Sync + 'static>(service: Arc<TaskDecompositionService>) -> Router<S> {
    Router::new()
        .route("/", post(create_task_handler).get(poll_tasks_handler))
        .route("/:task_id/status", put(update_task_status_handler))
        .layer(axum::middleware::from_fn({
            let store = std::sync::Arc::new(crate::auth::Store::new());
            move |req: axum::extract::Request, next: axum::middleware::Next| {
                let store = store.clone();
                async move {
                    use axum::response::IntoResponse;
                    let auth_header = req.headers().get("authorization").and_then(|h| h.to_str().ok());
                    let token = match auth_header {
                        Some(h) if h.to_lowercase().starts_with("bearer ") => &h[7..],
                        _ => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
                    };
                    let claims = match store.validate_token(token).await {
                        Ok(c) => c,
                        Err(_) => return (StatusCode::UNAUTHORIZED, "Invalid Token").into_response(),
                    };
                    let mut req = req;
                    req.extensions_mut().insert(claims);
                    next.run(req).await
                }
            }
        }))
        .with_state(service)
}
