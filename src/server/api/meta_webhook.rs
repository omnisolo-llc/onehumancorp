use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::Row;

use crate::db::DbStore;

#[derive(Clone)]
pub struct MetaWebhookState {
    pub db: Arc<DbStore>,
}

#[derive(Debug, Deserialize)]
pub struct MetaWebhookPayload {
    pub object: String,
    pub entry: Vec<MetaWebhookEntry>,
}

#[derive(Debug, Deserialize)]
pub struct MetaWebhookEntry {
    pub id: String,
    pub time: i64,
    pub messaging: Vec<MetaWebhookMessaging>,
}

#[derive(Debug, Deserialize)]
pub struct MetaWebhookMessaging {
    pub sender: MetaWebhookSender,
    pub recipient: MetaWebhookRecipient,
    pub timestamp: i64,
    pub message: MetaWebhookMessage,
}

#[derive(Debug, Deserialize)]
pub struct MetaWebhookSender {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct MetaWebhookRecipient {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct MetaWebhookMessage {
    pub mid: String,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct WebhookResponse {
    pub status: String,
}

pub fn meta_webhook_router(db: Arc<DbStore>) -> Router {
    Router::new()
        .route("/api/meta/webhook", post(meta_webhook_handler))
        .with_state(MetaWebhookState { db })
}

async fn meta_webhook_handler(
    State(state): State<MetaWebhookState>,
    Json(payload): Json<MetaWebhookPayload>,
) -> Json<WebhookResponse> {
    for entry in payload.entry {
        for messaging in entry.messaging {
            let message_text = messaging.message.text;
            let sender_id = messaging.sender.id;

            // Assume tenant_id is somehow derived from the page/recipient. Hardcoding for example.
            let tenant_id = "default_tenant";

            process_omnichannel_message(&state, tenant_id, &sender_id, &message_text).await;
        }
    }
    Json(WebhookResponse { status: "received".to_string() })
}

async fn process_omnichannel_message(state: &MetaWebhookState, tenant_id: &str, sender_id: &str, message_text: &str) {
    let mut auto_translate = false;
    let mut target_languages = String::new();

    let query = "SELECT auto_translate, target_languages FROM ohc_translation_preferences WHERE tenant_id = $1";
    match state.db.pool() {
        crate::db::DbPool::Postgres(pool) => {
            if let Ok(Some(row)) = sqlx::query(query).bind(tenant_id).fetch_optional(pool).await {
                auto_translate = row.get("auto_translate");
                if let Ok(v) = row.try_get::<serde_json::Value, _>("target_languages") {
                    target_languages = v.to_string();
                }
            }
        }
        crate::db::DbPool::Sqlite(pool) => {
            if let Ok(Some(row)) = sqlx::query(query).bind(tenant_id).fetch_optional(pool).await {
                auto_translate = row.get("auto_translate");
                if let Ok(v) = row.try_get::<serde_json::Value, _>("target_languages") {
                    target_languages = v.to_string();
                }
            }
        }
    }

    let mut final_message = message_text.to_string();
    if auto_translate {
        final_message = translate_inbox_message_with_llm(&final_message, "en").await;
        println!("Translated message for tenant {} from {} (targets: {}): {}", tenant_id, sender_id, target_languages, final_message);
    } else {
        println!("Received message for tenant {} from {}: {}", tenant_id, sender_id, final_message);
    }
}

async fn translate_inbox_message_with_llm(message: &str, target_lang: &str) -> String {
    format!("(Translated to {}): {}", target_lang, message)
}
