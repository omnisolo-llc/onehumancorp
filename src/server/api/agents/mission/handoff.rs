use axum::{
    extract::{Extension, State, Path},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::sip::SipDB;
use ::server_common::Claims;

#[derive(Deserialize)]
pub struct HandoffRequest {
    pub blockers: String,
}

#[derive(Serialize)]
pub struct HandoffResponse {
    pub success: bool,
}

pub fn router<S>(sip_db: Arc<SipDB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/{id}/handoff", post(handoff_mission_endpoint))
        .with_state(sip_db)
}

async fn handoff_mission_endpoint(
    State(sip_db): State<Arc<SipDB>>,
    Path(id): Path<String>,
    Extension(_claims): Extension<Claims>,
    Json(payload): Json<HandoffRequest>,
) -> impl IntoResponse {
    match sip_db.handoff_mission(&id, &payload.blockers).await {
        Ok(_) => (StatusCode::OK, Json(HandoffResponse { success: true })).into_response(),
        Err(e) => {
            tracing::error!("Failed to handoff mission {}: {:?}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(HandoffResponse { success: false })).into_response()
        }
    }
}
