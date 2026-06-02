use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    http::StatusCode,
    routing::{get},
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::{AIAgentPersona, AIaaSWorkflow, DepartmentType, ActionRisk};
use ::server_common::Claims;

#[derive(Serialize)]
pub struct PersonasResponse {
    pub personas: Vec<AIAgentPersona>,
}

#[derive(Serialize)]
pub struct WorkflowsResponse {
    pub workflows: Vec<AIaaSWorkflow>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/personas", get(list_personas))
        .route("/workflows", get(list_workflows))
        .with_state(orchestrator)
}

async fn list_personas(
    _state: State<Arc<DepartmentOrchestrator>>,
    Extension(_claims): Extension<Claims>,
) -> impl IntoResponse {
    let personas = vec![
        AIAgentPersona {
            id: "persona_cs_1".to_string(),
            name: "Friendly Ambassador".to_string(),
            department: DepartmentType::CustomerSuccess,
            tone_of_voice: "Friendly and helpful".to_string(),
            system_prompt: "You are a customer success representative...".to_string(),
            capabilities: vec!["draft_replies".to_string(), "send_refunds".to_string()],
        },
        AIAgentPersona {
            id: "persona_mkt_1".to_string(),
            name: "Creative Promoter".to_string(),
            department: DepartmentType::Marketing,
            tone_of_voice: "Enthusiastic and persuasive".to_string(),
            system_prompt: "You are a marketing expert...".to_string(),
            capabilities: vec!["draft_social_posts".to_string(), "create_campaigns".to_string()],
        },
    ];

    (StatusCode::OK, Json(PersonasResponse { personas })).into_response()
}

async fn list_workflows(
    _state: State<Arc<DepartmentOrchestrator>>,
    Extension(_claims): Extension<Claims>,
) -> impl IntoResponse {
    let workflows = vec![
        AIaaSWorkflow {
            id: "wf_social_post".to_string(),
            name: "Draft Social Post".to_string(),
            trigger_event: "tenant.job.completed".to_string(),
            persona_id: "persona_mkt_1".to_string(),
            description_template: "Draft social media post for {job_name}".to_string(),
            default_risk: ActionRisk::DraftForReview,
            enabled: true,
        },
        AIaaSWorkflow {
            id: "wf_customer_reply".to_string(),
            name: "Auto Reply to Customer".to_string(),
            trigger_event: "tenant.message.received".to_string(),
            persona_id: "persona_cs_1".to_string(),
            description_template: "Auto-reply to message from {customer_name}".to_string(),
            default_risk: ActionRisk::DraftForReview,
            enabled: true,
        },
    ];

    (StatusCode::OK, Json(WorkflowsResponse { workflows })).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DbStore;
    use crate::orchestration::mesh::CentrifugeNode;
    use ohc_builtin_agent::mesh::transport::InProcessTransport;
    use axum::http::Request;
    use axum::body::Body;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_list_personas_and_workflows() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let db = Arc::new(crate::db::DB::new().await.unwrap());
        let transport = Arc::new(InProcessTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let orchestrator = Arc::new(DepartmentOrchestrator::new(db, mesh));

        let app = router(orchestrator);

        let req_personas = Request::builder()
            .uri("/personas")
            .method("GET")
            .extension(Claims {
                sub: "test".to_string(),
                email: "test@example.com".to_string(),
                organization_id: Some("org1".to_string()),
                roles: vec![],
                exp: 0,
            })
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(req_personas).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let req_workflows = Request::builder()
            .uri("/workflows")
            .method("GET")
            .extension(Claims {
                sub: "test".to_string(),
                email: "test@example.com".to_string(),
                organization_id: Some("org1".to_string()),
                roles: vec![],
                exp: 0,
            })
            .body(Body::empty())
            .unwrap();

        let response2 = app.clone().oneshot(req_workflows).await.unwrap();
        assert_eq!(response2.status(), StatusCode::OK);
    }
}
