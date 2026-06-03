use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::repository::support_ticket_repo::SupportTicketRepository;
use crate::orchestration::departments::customer_success_agent::CustomerSuccessAgent;

pub struct SupportEngineState {
    pub ticket_repo: Arc<SupportTicketRepository>,
    pub cs_agent: Arc<RwLock<CustomerSuccessAgent>>,
}

#[derive(Deserialize)]
pub struct OmnichannelWebhookPayload {
    pub tenant_id: String,
    pub channel: String,
    pub external_message_id: Option<String>,
    pub customer_id: Option<String>,
    pub content: String,
    pub signature: Option<String>,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub status: String,
    pub message: String,
}

pub async fn handle_webhook(
    State(state): State<Arc<SupportEngineState>>,
    Json(payload): Json<OmnichannelWebhookPayload>,
) -> impl IntoResponse {
    match state.ticket_repo.create_ticket(
        &payload.tenant_id,
        &payload.channel,
        payload.external_message_id.as_deref(),
        payload.customer_id.as_deref(),
    ).await {
        Ok(ticket) => {
            match state.ticket_repo.add_message(
                &ticket.id,
                "customer",
                &payload.content,
                None,
            ).await {
                Ok(_) => {
                    let cs_agent = state.cs_agent.read().await;
                    let _ = cs_agent.process_support_ticket(&payload.tenant_id, &ticket.id, &payload.content).await;
                    (StatusCode::OK, Json(WebhookResponse { status: "success".into(), message: "Ticket created".into() }))
                }
                Err(e) => {
                    eprintln!("Failed to add message: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { status: "error".into(), message: "Failed to create message".into() }))
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create ticket: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { status: "error".into(), message: "Failed to create ticket".into() }))
        }
    }
}

pub async fn get_open_tickets(
    State(state): State<Arc<SupportEngineState>>,
    Path(tenant_id): Path<String>,
) -> impl IntoResponse {
    match state.ticket_repo.list_open_tickets(&tenant_id).await {
        Ok(tickets) => (StatusCode::OK, Json(tickets)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![] as Vec<String>)).into_response(),
    }
}

pub async fn get_ticket_details(
    State(state): State<Arc<SupportEngineState>>,
    Path((_tenant_id, ticket_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match state.ticket_repo.get_ticket_messages(&ticket_id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![] as Vec<String>)).into_response(),
    }
}

#[derive(Deserialize)]
pub struct UpdateTicketStatusPayload {
    pub status: String,
}

pub async fn update_ticket_status(
    State(state): State<Arc<SupportEngineState>>,
    Path((_tenant_id, ticket_id)): Path<(String, String)>,
    Json(payload): Json<UpdateTicketStatusPayload>,
) -> impl IntoResponse {
    match state.ticket_repo.update_ticket_status(&ticket_id, &payload.status).await {
        Ok(_) => (StatusCode::OK, Json(WebhookResponse { status: "success".into(), message: "Updated".into() })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { status: "error".into(), message: "Failed".into() })).into_response(),
    }
}

pub fn router(ticket_repo: Arc<SupportTicketRepository>, cs_agent: Arc<RwLock<CustomerSuccessAgent>>) -> Router {
    let state = Arc::new(SupportEngineState { ticket_repo, cs_agent });
    Router::new()
        .route("/webhook", post(handle_webhook))
        .route("/:tenant_id/tickets", get(get_open_tickets))
        .route("/:tenant_id/tickets/:ticket_id", get(get_ticket_details))
        .route("/:tenant_id/tickets/:ticket_id/status", put(update_ticket_status))
        .with_state(state)
}
