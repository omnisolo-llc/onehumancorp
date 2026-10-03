use crate::hub::Hub;
use axum::{
    Router,
    extract::{Json, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct HireAgentRequest {
    pub name: String,
    pub role: String,
    #[serde(default)]
    #[serde(rename = "providerType")]
    pub provider_type: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub task: Option<String>,
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
        .route("/execution-policy", get(execution_policy_handler))
        .route("/marketplace", get(list_marketplace_agents))
        .with_state(hub)
}

use axum::extract::FromRequest;

async fn execution_policy_handler(
    axum::extract::Extension(execution): axum::extract::Extension<
        Arc<crate::workflow_execution::WorkflowExecution>,
    >,
    claims: Option<axum::extract::Extension<::server_common::Claims>>,
) -> impl IntoResponse {
    let Some(claims) =
        claims.filter(|claims| ::server_common::auth_utils::signed_tenant_id(&claims.0).is_some())
    else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error":"Authentication required"})),
        )
            .into_response();
    };
    if !claims
        .roles
        .iter()
        .any(|role| role.eq_ignore_ascii_case("owner") || role.eq_ignore_ascii_case("admin"))
    {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error":"Owner or administrator access required"})),
        )
            .into_response();
    }
    Json(serde_json::json!({"available": execution.policy().is_some(), "policy": execution.policy(), "mode": "text_analysis", "workspace_access": false, "tools": []})).into_response()
}

async fn hire_handler(
    State(hub): State<Arc<Hub>>,
    req: axum::extract::Request,
) -> impl IntoResponse {
    let tenant_id = match req
        .extensions()
        .get::<::server_common::Claims>()
        .and_then(::server_common::auth_utils::signed_tenant_id)
    {
        Some(organization_id) => organization_id,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(HireAgentResponse {
                    id: "".to_string(),
                    status: "error".to_string(),
                    agent_id: "".to_string(),
                    workflow_id: "".to_string(),
                    message: "Authentication required".to_string(),
                }),
            )
                .into_response();
        }
    };

    let claims = req
        .extensions()
        .get::<::server_common::Claims>()
        .expect("validated signed tenant")
        .clone();
    if !claims
        .roles
        .iter()
        .any(|role| role.eq_ignore_ascii_case("owner") || role.eq_ignore_ascii_case("admin"))
    {
        return (
            StatusCode::FORBIDDEN,
            Json(HireAgentResponse {
                id: String::new(),
                status: "error".into(),
                agent_id: String::new(),
                workflow_id: String::new(),
                message: "An owner or administrator must start agent work".into(),
            }),
        )
            .into_response();
    }
    let actor_id = claims.sub.clone();
    let execution = req
        .extensions()
        .get::<Arc<crate::workflow_execution::WorkflowExecution>>()
        .cloned();
    let headers = req.headers().clone();

    let (parts, body) = req.into_parts();
    let req2 = axum::extract::Request::from_parts(parts, body);

    let payload: HireAgentRequest =
        match axum::extract::Json::<HireAgentRequest>::from_request(req2, &()).await {
            Ok(Json(payload)) => payload,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(HireAgentResponse {
                        id: "".to_string(),
                        status: "error".to_string(),
                        agent_id: "".to_string(),
                        workflow_id: "".to_string(),
                        message: "Invalid payload".to_string(),
                    }),
                )
                    .into_response();
            }
        };

    if payload
        .task
        .as_deref()
        .is_some_and(|task| task.trim().is_empty())
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(HireAgentResponse {
                id: String::new(),
                status: "error".into(),
                agent_id: String::new(),
                workflow_id: String::new(),
                message: "A supplied task must not be empty".into(),
            }),
        )
            .into_response();
    }

    let invalid_registration = payload.name.trim().is_empty()
        || payload.name.chars().count() > 200
        || payload.role.trim().is_empty()
        || payload.role.chars().count() > 120
        || (!payload.provider_type.is_empty() && payload.provider_type != "builtin")
        || (payload.task.is_none() && !payload.model.is_empty() && payload.model != "Auto");
    if invalid_registration {
        return (StatusCode::BAD_REQUEST, Json(HireAgentResponse {
            id: String::new(), status: "error".into(), agent_id: String::new(), workflow_id: String::new(),
            message: "Provide a supported agent name and role; execution configuration requires an explicit task".into(),
        })).into_response();
    }
    let agent_id = format!("agent-{}", uuid::Uuid::new_v4().simple());
    let mut agent = ::server_omnisolo::orchestration::Agent {
        id: agent_id.clone(),
        name: payload.name.clone(),
        role: payload.role.clone(),
        organization_id: tenant_id.clone(),
        status: "IDLE".into(),
        provider_type: "builtin".into(),
    };
    let Some(task) = payload.task.as_deref() else {
        hub.register_agent(agent).await;
        return (
            StatusCode::CREATED,
            Json(HireAgentResponse {
                id: agent_id.clone(),
                agent_id,
                status: "idle".into(),
                workflow_id: String::new(),
                message: "Registered an idle agent. No task was submitted or started.".into(),
            }),
        )
            .into_response();
    };
    let Some(execution) = execution else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(HireAgentResponse {
                id: String::new(),
                status: "error".into(),
                agent_id: String::new(),
                workflow_id: String::new(),
                message: "Tenant text analysis is unavailable; no agent was registered".into(),
            }),
        )
            .into_response();
    };
    let admitted = match execution
        .admit(&claims, &headers, task, &payload.model, "expert_task")
        .await
    {
        Ok(admitted) => admitted,
        Err(error) => {
            return (
                error.status(),
                Json(HireAgentResponse {
                    id: String::new(),
                    status: "error".into(),
                    agent_id: String::new(),
                    workflow_id: String::new(),
                    message: error.message().into(),
                }),
            )
                .into_response();
        }
    };
    let workflow_id = uuid::Uuid::new_v4().to_string();
    let record = crate::WorkflowRecord {
        id: workflow_id.clone(),
        tenant_id: tenant_id.clone(),
        actor_id,
        name: format!("{} text analysis", payload.name),
        workflow: "expert_task".into(),
        task: task.to_owned(),
        model: admitted.policy().model.clone(),
        provider: admitted.policy().provider.clone(),
        status: "queued".into(),
        command: "Configured tenant text analysis; no tools or workspace access".into(),
        created_at: chrono::Utc::now().to_rfc3339(),
        output: None,
        error: None,
    };
    {
        let Ok(mut workflows) = crate::get_workflow_registry().write() else {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(HireAgentResponse {
                    id: String::new(),
                    status: "error".into(),
                    agent_id: String::new(),
                    workflow_id: String::new(),
                    message: "Workflow records unavailable".into(),
                }),
            )
                .into_response();
        };
        workflows.insert(0, record.clone());
    }
    agent.status = "QUEUED".into();
    hub.register_agent(agent).await;
    let registration = Arc::new(crate::RegisteredWorkflowAgent {
        hub: hub.clone(),
        agent_id: agent_id.clone(),
        tenant_id,
    });
    crate::dispatch_workflow(record, admitted, execution, Some(registration));
    (
        StatusCode::CREATED,
        Json(HireAgentResponse {
            id: agent_id.clone(),
            status: "queued".into(),
            agent_id,
            workflow_id,
            message: format!("Accepted configured text analysis for {}", payload.name),
        }),
    )
        .into_response()
}

async fn list_agents_handler(
    State(hub): State<Arc<Hub>>,
    claims: Option<axum::extract::Extension<::server_common::Claims>>,
) -> impl IntoResponse {
    let Some(tenant_id) = claims
        .as_ref()
        .and_then(|claims| ::server_common::auth_utils::signed_tenant_id(&claims.0))
    else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    (
        StatusCode::OK,
        Json(hub.get_agents_by_org(&tenant_id).await),
    )
        .into_response()
}

#[derive(Deserialize, Debug)]
pub struct MarketplaceQuery {
    pub q: Option<String>,
}

pub async fn list_marketplace_agents(Query(query): Query<MarketplaceQuery>) -> impl IntoResponse {
    let marketplace_url = std::env::var("AGENT_MARKETPLACE_URL").unwrap_or_default();
    let provider =
        omnisolo_builtin_agent::tools::marketplace::configured_provider(&marketplace_url);
    let marketplace = omnisolo_builtin_agent::tools::marketplace::MarketplaceClient::new(provider);

    let q = query.q.unwrap_or_default();

    match marketplace.search(&q).await {
        Ok(agents) => {
            let json_agents =
                serde_json::to_value(agents).unwrap_or_else(|_| serde_json::json!([]));
            (StatusCode::OK, Json(json_agents)).into_response()
        }
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "error": "Marketplace registry is unavailable" })),
        )
            .into_response(),
    }
}
