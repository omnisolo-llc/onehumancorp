use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
    routing::post,
    Router,
};
use ohc_builtin_agent_core::pubsub::SubagentBus;
use scout_lib::agent::ScoutAgent;
use scout_lib::db::ScoutDb;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ToolRequestPayload {
    pub tool_name: String,
    pub description: Option<String>,
    pub api_url: Option<String>,
}

pub fn router() -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    Router::new().route("/tool_request", post(tool_request_handler))
}

async fn tool_request_handler(
    Extension(claims): Extension<crate::common::Claims>,
    Json(payload): Json<ToolRequestPayload>,
) -> axum::response::Response {
    let pool = crate::db::get_pool();
    let db = ScoutDb::new_pg(pool);
    let bus = Arc::new(SubagentBus::new());
    let agent = ScoutAgent::new(db, bus);

    let result = agent
        .process_tool_request(
            &claims.organization_id.unwrap_or_else(|| "default".to_string()),
            &payload.tool_name,
            payload.description.as_deref(),
            payload.api_url.as_deref(),
        )
        .await;

    match result {
        Ok(id) => (
            axum::http::StatusCode::OK,
            Json(serde_json::json!({ "status": "success", "id": id.to_string() })),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to process tool request: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Failed to process tool request" })),
            )
                .into_response()
        }
    }
}
