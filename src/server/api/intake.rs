use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize};
use std::sync::Arc;
use crate::domain::intake::{ProjectIntake, ProjectTask};
use sqlx::PgPool;
use crate::services::intake::IntakeService;

#[derive(Clone)]
pub struct IntakeAppState {
    pub intake_service: Arc<IntakeService>,
}

#[derive(Deserialize)]
pub struct CreateIntakeRequest {
    source: String,
    raw_content: String,
    client_info: Option<serde_json::Value>,
}

pub async fn create_intake(
    State(state): State<Arc<IntakeAppState>>,
    // In a real app we'd extract tenant_id from auth context. For simplicity:
    Json(payload): Json<CreateIntakeRequest>,
) -> Result<Json<ProjectIntake>, axum::http::StatusCode> {
    let tenant_id = "default_tenant"; // Placeholder

    match state.intake_service.create_intake(tenant_id, &payload.source, &payload.raw_content, payload.client_info).await {
        Ok(intake) => Ok(Json(intake)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_intakes(
    State(state): State<Arc<IntakeAppState>>,
) -> Result<Json<Vec<ProjectIntake>>, axum::http::StatusCode> {
    let tenant_id = "default_tenant"; // Placeholder

    match state.intake_service.list_intakes(tenant_id).await {
        Ok(intakes) => Ok(Json(intakes)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    proposal_id: String,
    title: String,
    description: Option<String>,
    assigned_to: Option<String>,
}

pub async fn create_task(
    State(state): State<Arc<IntakeAppState>>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<Json<ProjectTask>, axum::http::StatusCode> {
    let tenant_id = "default_tenant"; // Placeholder

    match state.intake_service.create_task_from_proposal(tenant_id, &payload.proposal_id, &payload.title, payload.description, payload.assigned_to).await {
        Ok(task) => Ok(Json(task)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_proposal_tasks(
    State(state): State<Arc<IntakeAppState>>,
    Path(proposal_id): Path<String>,
) -> Result<Json<Vec<ProjectTask>>, axum::http::StatusCode> {
    let tenant_id = "default_tenant"; // Placeholder

    match state.intake_service.get_tasks_for_proposal(tenant_id, &proposal_id).await {
        Ok(tasks) => Ok(Json(tasks)),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn router<S: Clone + Send + Sync + 'static>(pool: Arc<PgPool>) -> Router<S> {
    let state = Arc::new(IntakeAppState {
        intake_service: Arc::new(IntakeService::new(pool)),
    });

    Router::new()
        .route("/intake", post(create_intake).get(list_intakes))
        .route("/intake/tasks", post(create_task))
        .route("/intake/tasks/{proposal_id}", get(list_proposal_tasks))
        .with_state(state)
}
