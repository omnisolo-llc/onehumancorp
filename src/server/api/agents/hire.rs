use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::hub::Hub;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct HireAgentRequest {
    pub name: String,
    pub role: String,
    #[serde(default)]
    #[serde(rename = "providerType")]
    pub provider_type: String,
    #[serde(default)]
    pub model: String,
}

#[derive(Serialize, Debug)]
pub struct HireAgentResponse {
    pub id: String,
    pub status: String,
    pub agent_id: String,
    pub workflow_id: String,
    pub message: String,
}

pub fn router<S>(hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list_agents_handler))
        .route("/hire", post(hire_handler))
        .with_state(hub)
}

use axum::extract::FromRequest;

async fn hire_handler(
    State(hub): State<Arc<Hub>>,
    req: axum::extract::Request,
) -> impl IntoResponse {
    let tenant_id = match req.extensions().get::<::server_common::Claims>() {
        Some(claims) => claims.organization_id.clone().unwrap_or_else(|| ::server_common::auth_utils::get_default_tenant()),
        None => ::server_common::auth_utils::get_default_tenant(),
    };

    let (parts, body) = req.into_parts();
    let req2 = axum::extract::Request::from_parts(parts, body);

    let payload: HireAgentRequest = match axum::extract::Json::<HireAgentRequest>::from_request(req2, &()).await {
        Ok(Json(payload)) => payload,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(HireAgentResponse { id: "".to_string(), status: "error".to_string(), agent_id: "".to_string(), workflow_id: "".to_string(), message: "Invalid payload".to_string() })).into_response(),
    };

    let now = chrono::Utc::now().timestamp();
    let agent_id = format!("agent-{}-{}", now, uuid::Uuid::new_v4().simple());
    let provider_type = if payload.provider_type.is_empty() {
        "builtin".to_string()
    } else {
        payload.provider_type.clone()
    };

    let agent = ::server_ohc::orchestration::Agent {
        id: agent_id.clone(),
        name: payload.name.clone(),
        role: payload.role.clone(),
        organization_id: tenant_id,
        status: "RUNNING".to_string(),
        provider_type,
    };

    hub.register_agent(agent);
    let model = if payload.model.trim().is_empty() {
        std::env::var("OHC_LLM_MODEL")
            .or_else(|_| std::env::var("MINIMAX_MODEL"))
            .unwrap_or_else(|_| "MiniMax-M3".to_string())
    } else {
        payload.model.clone()
    };
    let workflow_task = format!(
        "A newly hired OHC agent named '{}' with role '{}' should start improving the business now. \
         Run a practical business operating swarm for this company, identify the highest leverage work, \
         and assign concrete next actions to specialist agents. Use model {}.",
        payload.name, payload.role, model
    );
    let workflow_id = uuid::Uuid::new_v4().to_string();
    let binary = crate::workflow_agent_binary();
    let agent_task = crate::workflow_agent_task("ohc_business_swarm", &workflow_task);
    let record = crate::WorkflowRecord {
        id: workflow_id.clone(),
        name: format!("{} business swarm", payload.name),
        workflow: "ohc_business_swarm".to_string(),
        task: workflow_task,
        status: "running".to_string(),
        command: format!("{} --task {}", binary, serde_json::to_string(&agent_task).unwrap_or_default()),
        created_at: chrono::Utc::now().to_rfc3339(),
        output: None,
        error: None,
    };
    if let Ok(mut workflows) = crate::get_workflow_registry().write() {
        workflows.insert(0, record.clone());
    }
    crate::dispatch_workflow(record);

    let response = HireAgentResponse {
        id: agent_id.clone(),
        status: "running".to_string(),
        agent_id,
        workflow_id,
        message: format!("Hired {} as {} and started a real MiniMax business swarm", payload.name, payload.role),
    };

    (StatusCode::CREATED, Json(response)).into_response()
}

async fn list_agents_handler(State(hub): State<Arc<Hub>>) -> impl IntoResponse {
    (StatusCode::OK, Json((*hub.get_agents().await).clone())).into_response()
}
