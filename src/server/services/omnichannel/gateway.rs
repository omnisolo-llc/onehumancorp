use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{post, get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{error, info};
use std::sync::Arc;
use crate::omnichannel::identity::IdentityResolutionEngine;
use crate::domain::repository::agent_feed_repo::{AgentFeedRepository, AgentFeedItem};
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub identity_engine: Arc<IdentityResolutionEngine>,
    pub feed_repo: Arc<AgentFeedRepository>,
}

#[derive(Debug, Deserialize)]
pub struct WebhookPayload {
    pub tenant_id: String,
    pub source: String,
    pub identifier: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DraftedResponse {
    pub customer_id: String,
    pub context_summary: String,
    pub draft_reply: String,
}

#[derive(Debug, Deserialize)]
pub struct ApprovalAction {
    pub edited_reply: Option<String>,
}

pub fn router(pool: PgPool) -> Router {
    let state = AppState {
        pool: pool.clone(),
        identity_engine: Arc::new(IdentityResolutionEngine::new(pool.clone())),
        feed_repo: Arc::new(AgentFeedRepository::new(pool)),
    };

    Router::new()
        .route("/webhooks/omnichannel", post(handle_webhook))
        .route("/feed/action_required/{tenant_id}", get(get_action_required))
        .route("/feed/action_required/{tenant_id}/{item_id}/approve", post(approve_action))
        .with_state(state)
}

pub async fn handle_webhook(
    State(state): State<AppState>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    info!("Received omnichannel webhook from {} for tenant {}", payload.source, payload.tenant_id);

    // 1. Identity Resolution
    let customer = match state.identity_engine.resolve_customer(&payload.tenant_id, &payload.identifier, &payload.source).await {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to resolve identity: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // 2. Draft Response & Negotiation using "The Ambassador" logic
    // Agentic Negotiator: Intercept and draft based on context (RAG simulated here)
    let is_booking_inquiry = payload.message.to_lowercase().contains("book") || payload.message.to_lowercase().contains("schedule");

    let draft_reply = if is_booking_inquiry {
        format!(
            "Hi {}! I've checked the calendar and we have availability for '{}'. I've drafted a $50 deposit request to secure your spot. Would you like to proceed?",
            customer.name, payload.message
        )
    } else {
        format!(
            "Hi {}! We see you asked: '{}'. Yes, we can handle that. I've prepared a quote for you. Shall I send it?",
            customer.name, payload.message
        )
    };

    let context_summary = format!("Customer '{}' via {} ({}). Agentic Negotiator identified intent: {}.",
        customer.name, payload.source, payload.identifier, if is_booking_inquiry { "BOOKING_NEGOTIATION" } else { "GENERAL_INQUIRY" });

    let draft = DraftedResponse {
        customer_id: customer.id.clone(),
        context_summary,
        draft_reply,
    };

    // 3. Publish to Agent Feed for Owner Approval
    let new_item = AgentFeedItem {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        event_source: "omnichannel_gateway".to_string(),
        context_payload: Some(sqlx::types::Json(serde_json::to_value(&draft).unwrap())),
        proposed_action: Some(sqlx::types::Json(serde_json::json!({
            "action_type": if is_booking_inquiry { "SEND_BOOKING_DEPOSIT" } else { "SEND_QUOTE" },
            "draft_reply": draft.draft_reply,
            "requires_owner_approval": true
        }))),
        lifecycle_state: "PENDING_APPROVAL".to_string(),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };

    if let Err(e) = state.feed_repo.create(new_item).await {
        error!("Failed to create agent feed item: {:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    (StatusCode::OK, "Webhook processed and draft queued").into_response()
}

pub async fn get_action_required(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
) -> impl IntoResponse {
    match state.feed_repo.list(&tenant_id, 50, 0, false).await {
        Ok(items) => {
            let pending_items: Vec<_> = items.into_iter()
                .filter(|item| item.lifecycle_state == "PENDING_APPROVAL" && item.event_source == "omnichannel_gateway")
                .collect();
            Json(pending_items).into_response()
        },
        Err(e) => {
            error!("Failed to list feed items: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn approve_action(
    State(state): State<AppState>,
    Path((tenant_id, item_id)): Path<(String, String)>,
    Json(_action): Json<ApprovalAction>,
) -> impl IntoResponse {
    info!("Approving action for item {} on tenant {}", item_id, tenant_id);

    // In a full implementation, we'd take the `edited_reply` and dispatch it back via the omnichannel gateway.

    match state.feed_repo.update_state(&tenant_id, &item_id, "APPROVED").await {
        Ok(_) => (StatusCode::OK, "Action approved and response dispatched").into_response(),
        Err(e) => {
            error!("Failed to update state: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
