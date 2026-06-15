use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use tonic::Request;
use crate::ohc::app::booking_engine_service_client::BookingEngineServiceClient;
use crate::ohc::app::ReserveTimeSlotRequest;
use crate::auth::orchestration::AuthInfo;

pub fn router<S>(spiffe_id: String, org_id: String, agent_id: String) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_reserve_time_slot))
        .with_state((spiffe_id, org_id, agent_id))
}

async fn handle_reserve_time_slot(
    State((spiffe_id, org_id, agent_id)): State<(String, String, String)>,
    Json(payload): Json<ReserveTimeSlotRequest>,
) -> impl IntoResponse {
    let mut client = match BookingEngineServiceClient::connect("http://localhost:50051").await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to connect to BookingEngineService: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "service unavailable"}))).into_response();
        }
    };

    let mut request = Request::new(payload);
    request.extensions_mut().insert(AuthInfo {
        spiffe_id,
        org_id,
        agent_id,
    });

    match client.reserve_time_slot(request).await {
        Ok(response) => {
            (StatusCode::OK, Json(response.into_inner())).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to reserve time slot: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": "failed to reserve time slot"}))).into_response()
        }
    }
}
