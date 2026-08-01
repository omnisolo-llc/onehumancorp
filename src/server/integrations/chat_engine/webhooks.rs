use axum::{extract::State, Json, routing::post, Router};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use super::db;
use super::models::{ChatContact, ChatConversation, ChatMessage};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
}

#[derive(Deserialize)]
pub struct WhatsappWebhookPayload {
    pub from: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct EmailWebhookPayload {
    pub from_email: String,
    pub subject: String,
    pub text: String,
}

pub async fn handle_whatsapp_webhook(
    State(state): State<AppState>,
    Json(payload): Json<WhatsappWebhookPayload>,
) -> Json<&'static str> {
    let pool = &state.pool;
    let tenant_id = state.tenant_id;
    let inbox_id = state.inbox_id;

    // Find or create contact
    let contact = match db::get_contact_by_phone(pool, tenant_id, &payload.from).await.unwrap_or(None) {
        Some(c) => c,
        None => {
            let c = ChatContact {
                id: Uuid::new_v4(),
                tenant_id,
                name: None,
                email: None,
                phone: Some(payload.from.clone()),
            };
            db::create_contact(pool, &c).await.unwrap();
            c
        }
    };

    // Find or create conversation
    let conv = match db::get_conversation(pool, tenant_id, inbox_id, contact.id).await.unwrap_or(None) {
        Some(c) => c,
        None => {
            let c = ChatConversation {
                id: Uuid::new_v4(),
                tenant_id,
                inbox_id,
                contact_id: contact.id,
                assignee_id: None,
                status: "open".to_string(),
            };
            db::create_conversation(pool, &c).await.unwrap();
            c
        }
    };

    // Create message
    let msg = ChatMessage {
        id: Uuid::new_v4(),
        tenant_id,
        conversation_id: conv.id,
        sender_type: "contact".to_string(),
        sender_id: Some(contact.id),
        content: payload.body,
    };
    db::create_message(pool, &msg).await.unwrap();

    // Trigger AI Agent Work Triage (Placeholder for integration)
    tracing::info!("Triggered AI Work Triage for new WhatsApp message in conversation {}", conv.id);

    Json("ok")
}

pub fn webhook_routes(state: AppState) -> Router {
    Router::new()
        .route("/webhooks/whatsapp", post(handle_whatsapp_webhook))
        .with_state(state)
}
