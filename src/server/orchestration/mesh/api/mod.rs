use axum::{routing::post, Router, extract::State, response::IntoResponse, http::StatusCode};
use std::sync::Arc;
use super::transport::MeshTransport;


pub struct MeshApiState {
    pub transport: Arc<dyn MeshTransport>,
}

pub fn mesh_router(transport: Arc<dyn MeshTransport>) -> Router {
    let state = Arc::new(MeshApiState { transport });
    Router::new()
        .route("/api/mesh/v2/broadcast", post(broadcast_handler))
        .with_state(state)
}

async fn broadcast_handler(
    State(state): State<Arc<MeshApiState>>,
    payload_bytes: axum::body::Bytes,
) -> impl IntoResponse {
    let payload: crate::ohc::orchestration::PublishTeammateMeshEventRequest = match prost::Message::decode(payload_bytes.as_ref()) {
        Ok(p) => p,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid Protobuf payload".to_string()).into_response(),
    };
    let event = match payload.event {
        Some(e) => e,
        None => return (StatusCode::BAD_REQUEST, "Missing event".to_string()).into_response(),
    };
    match state.transport.publish(&payload.channel, "", &event.payload).await {
        Ok(_) => {
            (StatusCode::OK,
                "{\"status\": \"success\", \"message\": \"Event broadcast successfully\"}".to_string()).into_response()
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR,
                format!("{{\"status\": \"error\", \"message\": \"{}\"}}", e.replace('"', "\""))).into_response()
        }
    }
}
