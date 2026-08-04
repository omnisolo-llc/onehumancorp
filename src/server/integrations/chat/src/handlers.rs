use axum::{extract::{State, Path}, Json, routing::post, Router};
use std::sync::Arc;
use uuid::Uuid;
use crate::gateway::OmnichannelGateway;
use crate::models::WebhookPayload;

pub struct AppState {
    pub gateway: Arc<OmnichannelGateway>,
}

pub async fn webhook_handler(
    State(state): State<Arc<AppState>>,
    Path(tenant_id): Path<Uuid>,
    Json(payload): Json<WebhookPayload>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {

    match state.gateway.process_webhook(tenant_id, payload).await {
        Ok(_) => Ok(Json(serde_json::json!({"status": "success"}))),
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn create_router(gateway: Arc<OmnichannelGateway>) -> Router {
    let state = Arc::new(AppState { gateway });
    Router::new()
        .route("/webhooks/:tenant_id", post(webhook_handler))
        .with_state(state)
}
