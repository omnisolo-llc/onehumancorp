use axum::{
    extract::{Extension, State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::aiaas::{AIAgentPersona, AIaaSWorkflow, AIaaSExecutionEngine};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use ::server_common::Claims;

#[derive(Deserialize)]
pub struct TriggerWorkflowRequest {
    pub workflow_type: String,
    pub persona_name: String,
    pub system_prompt: String,
    pub capabilities: Vec<String>,
    pub payload: serde_json::Value,
}

#[derive(Serialize)]
pub struct TriggerWorkflowResponse {
    pub success: bool,
    pub workflow_id: Option<String>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let engine = Arc::new(AIaaSExecutionEngine::new(orchestrator));

    Router::new()
        .route("/trigger", post(trigger_workflow))
        .with_state(engine)
}

async fn trigger_workflow(
    State(engine): State<Arc<AIaaSExecutionEngine>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<TriggerWorkflowRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(TriggerWorkflowResponse { success: false, workflow_id: None })).into_response(),
    };

    let persona = AIAgentPersona::new(
        &payload.persona_name,
        &payload.system_prompt,
        payload.capabilities,
        &tenant_id,
    );

    let workflow = AIaaSWorkflow::new(
        &payload.workflow_type,
        &persona.id,
        &tenant_id,
        payload.payload,
    );

    match engine.trigger_workflow(&persona, workflow).await {
        Ok(workflow) => (StatusCode::OK, Json(TriggerWorkflowResponse { success: true, workflow_id: Some(workflow.id) })).into_response(),
        Err(e) => {
            tracing::error!("Failed to trigger AIaaS workflow: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(TriggerWorkflowResponse { success: false, workflow_id: None })).into_response()
        }
    }
}
