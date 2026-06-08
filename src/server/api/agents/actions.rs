use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::action_dispatcher::ActionDispatcher;
use ::server_common::Claims;
use ::server_ohc::interop::{DispatchActionRequest, DispatchActionResponse};

#[derive(Clone)]
pub struct ActionsState {
    pub dispatcher: Arc<ActionDispatcher>,
}

pub fn router<S>(dispatcher: Arc<ActionDispatcher>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = ActionsState {
        dispatcher,
    };
    Router::new()
        .route("/dispatch", post(handle_dispatch_action))
        .with_state(state)
}

async fn handle_dispatch_action(
    State(state): State<ActionsState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<DispatchActionRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()).into_response(),
    };

    match state.dispatcher.dispatch_action(
        tenant_id,
        payload.action_name,
        payload.payload_json,
    ).await {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e }))).into_response(),
    }
}
