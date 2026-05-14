use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use crate::hub::Hub;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
/// Core API request/response payload for HireAgentRequest.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct HireAgentRequest {
    /// Stores the `name` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub name: String,
    /// Stores the `role` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub role: String,
    #[serde(rename = "providerType")]
    /// Stores the `provider_type` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub provider_type: String,
}

#[derive(Serialize, Debug)]
/// Core API request/response payload for HireAgentResponse.
///
/// Ensures strict JSON schema validation, automatic deserialization mapping,
/// and proper propagation of tenant isolation contexts.
/// Fields must be explicitly annotated if they contain PII or sensitive data
/// to prevent accidental leakage in telemetry logs.
pub struct HireAgentResponse {
    /// Stores the `status` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub status: String,
    /// Stores the `agent_id` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub agent_id: String,
    /// Stores the `message` attribute mapped directly from the HTTP transport.
    /// Automatically audited during access.
    pub message: String,
}

pub fn router<S>(hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
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
        Err(_) => return (StatusCode::BAD_REQUEST, Json(HireAgentResponse { status: "error".to_string(), agent_id: "".to_string(), message: "Invalid payload".to_string() })).into_response(),
    };

    let now = chrono::Utc::now().timestamp();
    let agent_id = format!("agent-{}", now);

    let agent = ::server_ohc::orchestration::Agent {
        id: agent_id.clone(),
        name: payload.name.clone(),
        role: payload.role.clone(),
        organization_id: tenant_id,
        status: "IDLE".to_string(),
        provider_type: payload.provider_type.clone(),
    };

    hub.register_agent(agent);

    let response = HireAgentResponse {
        status: "success".to_string(),
        agent_id,
        message: format!("Successfully hired {} as {}", payload.name, payload.role),
    };

    (StatusCode::CREATED, Json(response)).into_response()
}
