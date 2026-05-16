use axum::{
    extract::{State, Json, Path},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::orchestration::journey::state::{JourneyManager, JourneyPhase, TransitionEvent};

pub fn router(manager: Arc<JourneyManager>) -> Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    let r = Router::new()
        .route("/:tenant_id/phase", get(get_phase))
        .route("/:tenant_id/events", post(post_event))
        .with_state(manager);

    Router::new().merge(r)
}

async fn get_phase(
    State(manager): State<Arc<JourneyManager>>,
    Path(tenant_id): Path<String>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    match manager.get_current_phase(&tenant_id).await {
        Ok(phase) => Ok(Json(serde_json::json!({ "phase": phase.to_string() }))),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn post_event(
    State(manager): State<Arc<JourneyManager>>,
    Path(tenant_id): Path<String>,
    Json(event): Json<TransitionEvent>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    match manager.process_event(&tenant_id, event).await {
        Ok(new_phase) => Ok(Json(serde_json::json!({ "new_phase": new_phase.to_string() }))),
        Err(_) => Err(axum::http::StatusCode::BAD_REQUEST),
    }
}
