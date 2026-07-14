use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use server_domain_inbox::models::IncomingMessage as ExtIncomingMessage;

pub async fn handle_incoming_message(
    State(_state): State<()>, // Placeholder for app state
    Json(_payload): Json<ExtIncomingMessage>,
) -> impl IntoResponse {
    // 1. Resolve Identity
    // 2. Publish to Event Mesh for Ambassador Agent
    (StatusCode::ACCEPTED, "Message received")
}
