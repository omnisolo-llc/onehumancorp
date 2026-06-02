use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{AIAgentPersona, AIaaSWorkflow};
use ::server_common::Claims;

#[derive(Serialize)]
pub struct AIAgentPersonaResponse {
    pub personas: Vec<AIAgentPersona>,
}

#[derive(Serialize)]
pub struct AIaaSWorkflowResponse {
    pub workflows: Vec<AIaaSWorkflow>,
}

#[derive(Deserialize)]
pub struct CreatePersonaRequest {
    pub name: String,
    pub system_prompt: String,
    pub capabilities: Vec<String>,
}

#[derive(Deserialize)]
pub struct CreateWorkflowRequest {
    pub persona_id: String,
    pub trigger_event: String,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/personas", get(list_personas).post(create_persona))
        .route("/workflows", get(list_workflows).post(create_workflow))
        .with_state(orchestrator)
}

async fn list_personas(
    State(_orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let _tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(AIAgentPersonaResponse { personas: vec![] })).into_response(),
    };

    let personas = vec![
        AIAgentPersona {
            id: "persona-1".to_string(),
            name: "Default Persona".to_string(),
            system_prompt: "You are a helpful assistant.".to_string(),
            capabilities: vec!["drafting".to_string()],
        }
    ];

    (StatusCode::OK, Json(AIAgentPersonaResponse { personas })).into_response()
}

async fn create_persona(
    State(_orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreatePersonaRequest>,
) -> impl IntoResponse {
    let _tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(AIAgentPersonaResponse { personas: vec![] })).into_response(),
    };

    let new_persona = AIAgentPersona {
        id: format!("persona-{}", uuid::Uuid::new_v4()),
        name: payload.name,
        system_prompt: payload.system_prompt,
        capabilities: payload.capabilities,
    };

    (StatusCode::OK, Json(new_persona)).into_response()
}

async fn list_workflows(
    State(_orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let _tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(AIaaSWorkflowResponse { workflows: vec![] })).into_response(),
    };

    let workflows = vec![
        AIaaSWorkflow {
            workflow_id: "wf-1".to_string(),
            persona_id: "persona-1".to_string(),
            trigger_event: "customer_message".to_string(),
            status: "active".to_string(),
        }
    ];

    (StatusCode::OK, Json(AIaaSWorkflowResponse { workflows })).into_response()
}

async fn create_workflow(
    State(_orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateWorkflowRequest>,
) -> impl IntoResponse {
    let _tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(AIaaSWorkflowResponse { workflows: vec![] })).into_response(),
    };

    let new_workflow = AIaaSWorkflow {
        workflow_id: format!("wf-{}", uuid::Uuid::new_v4()),
        persona_id: payload.persona_id,
        trigger_event: payload.trigger_event,
        status: "active".to_string(),
    };

    (StatusCode::OK, Json(new_workflow)).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use tower::ServiceExt;
    use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_list_personas_unauthorized() {
        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db, mesh));

        let app = router(orchestrator).layer(axum::middleware::from_fn(|req, next| async move {
            let mut req = req;
            let claims = Claims {
                sub: "test".to_string(),
                email: None,
                name: None,
                organization_id: None, // No org ID means unauthorized
                roles: vec![],
                jti: "test".to_string(),
            };
            req.extensions_mut().insert(claims);
            next.run(req).await
        }));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/personas")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
