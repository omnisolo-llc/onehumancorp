use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use crate::services::inbox::service::{InboxService, UnifiedTriageAction};

#[derive(Clone)]
pub struct AppState {
    pub inbox_service: Arc<InboxService>,
    pub pool: PgPool,
}

#[derive(Deserialize)]
pub struct ResolveActionRequest {
    pub tenant_id: String,
    pub resolution: String, // "approved", "rejected", "edited"
}

#[derive(Deserialize)]
pub struct UpdateInboxConfigRequest {
    pub working_hours_enabled: Option<bool>,
    pub out_of_office_message: Option<String>,
    pub greeting_enabled: Option<bool>,
    pub greeting_message: Option<String>,
}

pub fn router(pool: PgPool) -> Router {
    let state = AppState {
        inbox_service: Arc::new(InboxService::new(pool.clone())),
        pool,
    };

    Router::new()
        .route("/api/v1/inbox/:tenant_id/actions", get(get_pending_actions))
        .route("/api/v1/inbox/:tenant_id/actions/:action_id/resolve", post(resolve_action))
        .route("/api/v1/inbox/:tenant_id/inboxes/:inbox_id/config", post(update_inbox_config))
        .with_state(state)
}

async fn get_pending_actions(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
) -> Result<Json<Vec<UnifiedTriageAction>>, axum::http::StatusCode> {
    match state.inbox_service.get_pending_actions(&tenant_id).await {
        Ok(actions) => Ok(Json(actions)),
        Err(e) => {
            tracing::error!("Failed to fetch pending triage actions: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn resolve_action(
    State(state): State<AppState>,
    Path((tenant_id, action_id)): Path<(String, String)>,
    Json(payload): Json<ResolveActionRequest>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    if tenant_id != payload.tenant_id {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }
    match state.inbox_service.resolve_action(&tenant_id, &action_id, &payload.resolution).await {
        Ok(_) => Ok(axum::http::StatusCode::OK),
        Err(e) => {
            tracing::error!("Failed to resolve triage action: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn update_inbox_config(
    State(state): State<AppState>,
    Path((tenant_id, inbox_id)): Path<(String, String)>,
    Json(payload): Json<UpdateInboxConfigRequest>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_uuid = match Uuid::parse_str(&tenant_id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };
    let inbox_uuid = match Uuid::parse_str(&inbox_id) {
        Ok(uuid) => uuid,
        Err(_) => return Err(axum::http::StatusCode::BAD_REQUEST),
    };
    let chat_service = crate::services::chat::service::ChatService::new(state.pool);
    match chat_service.update_inbox_config(
        tenant_uuid,
        inbox_uuid,
        payload.working_hours_enabled,
        payload.out_of_office_message,
        payload.greeting_enabled,
        payload.greeting_message,
    ).await {
        Ok(_) => Ok(axum::http::StatusCode::OK),
        Err(e) => {
            tracing::error!("Failed to update inbox config: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
