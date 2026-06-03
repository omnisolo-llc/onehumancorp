use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
use std::sync::Arc;
use crate::hub::Hub;
use axum::http::HeaderMap;

#[derive(serde::Serialize)]
pub struct TerminalTokenResponse {
    pub secret: String,
}

pub fn router(hub: Arc<Hub>) -> axum::Router<Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    axum::Router::new()
        .route("/connection-token", axum::routing::post(generate_terminal_token_handler))
        .with_state(hub)
}

pub async fn generate_terminal_token_handler(
    _headers: HeaderMap,
    State(hub): State<Arc<Hub>>,
    request: axum::extract::Request,
) -> impl IntoResponse {
    let tenant_id = match request.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) => {
            if auth.org_id.is_empty() {
                "default".to_string()
            } else {
                auth.org_id.clone()
            }
        },
        None => return (StatusCode::UNAUTHORIZED, Json(TerminalTokenResponse { secret: "".to_string() })).into_response(),
    };

    let tracker = hub.tracker();
    if let Some(ref client) = tracker.stripe_client {
        match client.create_terminal_connection_token(&tenant_id).await {
            Ok(secret) => (StatusCode::OK, Json(TerminalTokenResponse { secret })).into_response(),
            Err(e) => {
                tracing::error!("Failed to create terminal token: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(TerminalTokenResponse { secret: "".to_string() })).into_response()
            }
        }
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(TerminalTokenResponse { secret: "".to_string() })).into_response()
    }
}
