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
        .route("/marketplace", get(list_marketplace_agents))
        .with_state(hub)
}

use axum::extract::FromRequest;

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
        .expect("validated signed tenant");
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

    let now = chrono::Utc::now().timestamp();
    let agent_id = format!("agent-{}-{}", now, uuid::Uuid::new_v4().simple());
    let provider_type = if payload.provider_type.is_empty() {
        "builtin".to_string()
    } else {
        payload.provider_type.clone()
    };

    let agent = ::server_omnisolo::orchestration::Agent {
        id: agent_id.clone(),
        name: payload.name.clone(),
        role: payload.role.clone(),
        organization_id: tenant_id.clone(),
        status: "RUNNING".to_string(),
        provider_type,
    };

    let model = if payload.model.trim().is_empty() {
        std::env::var("OMNISOLO_LLM_MODEL")
            .or_else(|_| std::env::var("MINIMAX_MODEL"))
            .unwrap_or_else(|_| "MiniMax-M3".to_string())
    } else {
        payload.model.clone()
    };
    let workflow_task = payload.task.clone().unwrap_or_else(|| format!(
        "A newly hired OmniSolo agent named '{}' with role '{}' should start improving the business now. \
         Run a practical business operating swarm for this company, identify the highest leverage work, \
         and assign concrete next actions to specialist agents. Use model {}.",
        payload.name, payload.role, model
    ));
    let workflow_id = uuid::Uuid::new_v4().to_string();
    let binary = crate::workflow_agent_binary();
    let agent_task = crate::workflow_agent_task("ohc_business_swarm", &workflow_task);
    let record = crate::WorkflowRecord {
        id: workflow_id.clone(),
        tenant_id,
        actor_id,
        name: format!("{} business swarm", payload.name),
        workflow: "ohc_business_swarm".to_string(),
        task: workflow_task,
        status: "running".to_string(),
        command: format!(
            "{} --task {}",
            binary,
            serde_json::to_string(&agent_task).unwrap_or_default()
        ),
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
    hub.register_agent(agent).await;
    crate::dispatch_workflow(record);

    let response = HireAgentResponse {
        id: agent_id.clone(),
        status: "running".to_string(),
        agent_id,
        workflow_id,
        message: format!("Accepted work for {} as {}", payload.name, payload.role),
    };

    (StatusCode::CREATED, Json(response)).into_response()
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
