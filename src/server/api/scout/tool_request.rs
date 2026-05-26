use axum::{extract::State, routing::post, Json, Router};
use axum::middleware;
use ohc_builtin_agent_core::pubsub::SubagentBus;
use scout_lib::{db::ScoutDb, ScoutAgent};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;

#[derive(Clone)]
pub struct ScoutAppState {
    pub db: PgPool,
    pub bus: Arc<SubagentBus>,
}

#[derive(Deserialize)]
pub struct ConnectToolRequest {
    pub tool_name: String,
}

#[derive(Serialize)]
pub struct ConnectToolResponse {
    pub success: bool,
    pub message: String,
    pub id: Option<String>,
}

pub async fn connect_tool_handler(
    State(state): State<ScoutAppState>,
    Json(payload): Json<ConnectToolRequest>,
) -> Json<ConnectToolResponse> {
    let scout_db = ScoutDb::new_pg(state.db.clone());
    let scout_agent = ScoutAgent::new(scout_db, state.bus.clone());

    let tenant_id = "DEFAULT";

    match scout_agent
        .process_tool_request(tenant_id, &payload.tool_name, None, None)
        .await
    {
        Ok(id) => Json(ConnectToolResponse {
            success: true,
            message: format!("Successfully requested integration for {}", payload.tool_name),
            id: Some(id.to_string()),
        }),
        Err(e) => Json(ConnectToolResponse {
            success: false,
            message: format!("Failed to request integration: {}", e),
            id: None,
        }),
    }
}

pub fn router(pool: PgPool, transport: Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>) -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    let state = ScoutAppState {
        db: pool,
        bus: Arc::new(SubagentBus::new()),
    };

    Router::new()
        .route("/connect", post(connect_tool_handler).with_state(state))
}
