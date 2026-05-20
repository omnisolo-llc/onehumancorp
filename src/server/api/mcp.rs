use axum::{
    extract::{Query, State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::integrations::mcp_gateway::McpGateway;

#[derive(Deserialize)]
pub struct DiscoverQuery {
    pub query: String,
}

#[derive(Deserialize)]
pub struct InvokeRequest {
    pub spiffe_id: String,
    pub tool_name: String,
    pub arguments: Value,
}

#[derive(Serialize)]
pub struct DiscoverResponse {
    pub tools: Vec<crate::integrations::mcp_gateway::DynamicToolSchema>,
}

#[derive(Serialize)]
pub struct InvokeResponse {
    pub result: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn discover_tools(
    State(gateway): State<Arc<McpGateway>>,
    Query(params): Query<DiscoverQuery>,
) -> impl IntoResponse {
    let tools = gateway.discover_tools(&params.query).await;
    (StatusCode::OK, Json(DiscoverResponse { tools }))
}

pub async fn invoke_tool(
    State(gateway): State<Arc<McpGateway>>,
    Json(payload): Json<InvokeRequest>,
) -> impl IntoResponse {
    match gateway.invoke_tool(&payload.spiffe_id, &payload.tool_name, payload.arguments).await {
        Ok(result) => (StatusCode::OK, Json(InvokeResponse { result })).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: e })).into_response(),
    }
}

pub fn router(gateway: Arc<McpGateway>) -> Router {
    Router::new()
        .route("/discover", get(discover_tools))
        .route("/invoke", post(invoke_tool))
        .with_state(gateway)
}
