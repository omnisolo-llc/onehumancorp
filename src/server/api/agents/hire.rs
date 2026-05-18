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
        Some(claims) => claims.organization_id.clone().unwrap_or_else(|| "system".to_string()),
        None => "system".to_string(),
    };

    let (parts, body) = req.into_parts();
    let req2 = axum::extract::Request::from_parts(parts, body);

    let payload: HireAgentRequest = match axum::extract::Json::<HireAgentRequest>::from_request(req2, &()).await {
        Ok(Json(payload)) => payload,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(HireAgentResponse { id: "".to_string(), status: "error".to_string(), agent_id: "".to_string(), message: "Invalid payload".to_string() })).into_response(),
    };

    let now = chrono::Utc::now().timestamp();
    let agent_id = format!("agent-{}", now);
    let provider_type = if payload.provider_type.is_empty() {
        payload.model.clone()
    } else {
        payload.provider_type.clone()
    };

    let agent = ::server_ohc::orchestration::Agent {
        id: agent_id.clone(),
        name: payload.name.clone(),
        role: payload.role.clone(),
        organization_id: tenant_id,
        status: "IDLE".to_string(),
        provider_type,
    };

    hub.register_agent(agent);

    let response = HireAgentResponse {
        id: agent_id.clone(),
        status: "success".to_string(),
        agent_id,
        message: format!("Successfully hired {} as {}", payload.name, payload.role),
    };

    (StatusCode::CREATED, Json(response)).into_response()
}

async fn list_agents_handler(State(hub): State<Arc<Hub>>) -> impl IntoResponse {
    (StatusCode::OK, Json((*hub.get_agents()).clone())).into_response()
}
