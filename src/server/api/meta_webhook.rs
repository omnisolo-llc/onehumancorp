use axum::{
    extract::{Query, State},
    response::IntoResponse,
    http::{StatusCode, HeaderMap},
};
use serde::Deserialize;
use serde_json::Value;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::sync::Arc;
use crate::hub::Hub;
use uuid::Uuid;
use crate::api::agents::translation::{translate_inbox_message_with_llm, generate_inbox_draft_reply, InboxTranslation};
use crate::orchestration::departments::types::DepartmentType;
use crate::orchestration::departments::types::ActionRisk;

#[derive(Clone)]
pub struct MetaWebhookState {
    pub hub: Arc<Hub>,
    pub db: Arc<crate::db::DB>,
    pub orchestrator: Arc<crate::orchestration::departments::orchestrator::DepartmentOrchestrator>,
}

#[derive(Deserialize)]
pub struct MetaVerifyQuery {
    #[serde(rename = "hub.mode")]
    pub mode: Option<String>,
    #[serde(rename = "hub.verify_token")]
    pub verify_token: Option<String>,
    #[serde(rename = "hub.challenge")]
    pub challenge: Option<String>,
}

pub async fn meta_webhook_get_handler(
    Query(query): Query<MetaVerifyQuery>,
) -> impl IntoResponse {
    let verify_token = match std::env::var("META_VERIFY_TOKEN") {
        Ok(t) if !t.is_empty() => t,
        _ => {
            ::server_telemetry::record_error_signal("META_VERIFY_TOKEN not configured");
            tracing::warn!("META_VERIFY_TOKEN not configured");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if let (Some(mode), Some(token), Some(challenge)) = (query.mode, query.verify_token, query.challenge) {
        if mode == "subscribe" && token == verify_token {
            return (StatusCode::OK, challenge).into_response();
        }
    }

    StatusCode::FORBIDDEN.into_response()
}

pub async fn meta_webhook_post_handler(
    headers: HeaderMap,
    State(state): State<MetaWebhookState>,
    body_bytes: axum::body::Bytes,
) -> impl IntoResponse {
    let secret = match std::env::var("META_APP_SECRET") {
        Ok(s) if !s.is_empty() => s,
        _ => {
            tracing::warn!("META_APP_SECRET not configured, returning unauthorized in production-like environment");
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };

    let signature_header = headers.get("x-hub-signature-256")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !signature_header.starts_with("sha256=") {
        tracing::warn!("Meta webhook missing or invalid signature header");
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let signature_hex = &signature_header["sha256=".len()..];
    let signature_bytes = match hex::decode(signature_hex) {
        Ok(b) => b,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let mut mac = match Hmac::<Sha256>::new_from_slice(secret.as_bytes()) {
        Ok(m) => m,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };
    mac.update(&body_bytes);
    if mac.verify_slice(&signature_bytes).is_err() {
        tracing::warn!("Meta webhook signature verification failed");
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let payload: Value = match serde_json::from_slice(&body_bytes) {
        Ok(p) => p,
        Err(_) => {
            ::server_telemetry::record_error_signal("Failed to parse Meta webhook payload");
            tracing::error!("Failed to parse Meta webhook payload");
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    if let Some(entries) = payload.get("entry").and_then(|e| e.as_array()) {
        for entry in entries {
            if let Some(messaging) = entry.get("messaging").and_then(|m| m.as_array()) {
                for event in messaging {
                    if let Some(message) = event.get("message") {
                        let sender_id = event.get("sender").and_then(|s| s.get("id")).and_then(|i| i.as_str()).unwrap_or("unknown");
                        let text = message.get("text").and_then(|t| t.as_str()).unwrap_or("");

                        if !text.is_empty() {
                            tracing::info!("Received Meta message from {}: {}", sender_id, text);
                            let tenant_id = "test_tenant".to_string(); // Future: look up by recipient
                            let source = "instagram".to_string();
                            process_omnichannel_message(&state, tenant_id, source, sender_id.to_string(), text.to_string()).await;
                        }
                    }
                }
            } else if let Some(changes) = entry.get("changes").and_then(|c| c.as_array()) {
                for change in changes {
                     if let Some(value) = change.get("value") {
                         if let Some(messages) = value.get("messages").and_then(|m| m.as_array()) {
                             for message in messages {
                                  let sender_id = message.get("from").and_then(|f| f.as_str()).unwrap_or("unknown");
                                  let text = message.get("text").and_then(|t| t.get("body")).and_then(|b| b.as_str()).unwrap_or("");

                                  if !text.is_empty() {
                                      tracing::info!("Received Meta WhatsApp message from {}: {}", sender_id, text);
                                      let tenant_id = "test_tenant".to_string(); // Future: look up by recipient
                                      let source = "whatsapp".to_string();
                                      process_omnichannel_message(&state, tenant_id, source, sender_id.to_string(), text.to_string()).await;
                                  }
                             }
                         }
                     }
                }
            }
        }
    }

    StatusCode::OK.into_response()
}

async fn process_omnichannel_message(state: &MetaWebhookState, tenant_id: String, source: String, sender_id: String, text: String) {
    let target_language = "English";

    let translation = match translate_inbox_message_with_llm(&tenant_id, &source, &text, target_language).await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Translation failed: {}", e);
            InboxTranslation {
                translated_content: text.clone(),
                source_language: Some("Unknown".to_string()),
                target_language: target_language.to_string(),
                original_content: text.clone(),
            }
        }
    };

    let draft_reply = match generate_inbox_draft_reply(&tenant_id, &source, &translation).await {
        Ok(d) => d,
        Err(e) => {
            tracing::error!("Failed to generate draft reply: {}", e);
            "Thanks for reaching out! We will review this and get back to you soon.".to_string()
        }
    };

    let inbox_id = Uuid::new_v4().to_string();
    let pool = &state.db.pool;

    let insert_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, translated_from_language, draft_reply, status, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, 'unread', NOW())"
            )
            .bind(&inbox_id)
            .bind(&tenant_id)
            .bind(&source)
            .bind(&translation.original_content)
            .bind(&translation.translated_content)
            .bind(&translation.source_language)
            .bind(&draft_reply)
            .execute(pool)
            .await.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                "INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, translated_from_language, draft_reply, status, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, 'unread', CURRENT_TIMESTAMP)"
            )
            .bind(&inbox_id)
            .bind(&tenant_id)
            .bind(&source)
            .bind(&translation.original_content)
            .bind(&translation.translated_content)
            .bind(&translation.source_language)
            .bind(&draft_reply)
            .execute(sqlite_pool)
            .await.map(|_| ())
        }
    };

    if let Err(e) = insert_result {
        tracing::error!("Failed to insert inbox_messages: {}", e);
    }

    let _ = state.orchestrator.execute_action(
        DepartmentType::CustomerSuccess,
        format!("New {} message from {} (Language: {:?})", source, tenant_id, translation.source_language),
        tenant_id.clone(),
        ActionRisk::DraftForReview,
        serde_json::json!({
            "source": source.clone(),
            "message": translation.translated_content.clone(),
            "original_content": translation.original_content.clone(),
            "translated_from_language": translation.source_language.clone(),
            "draft_reply": draft_reply.clone(),
            "inbox_message_id": inbox_id.clone(),
            "sender_id": sender_id.clone(),
        }),
    ).await;

    let event = crate::orchestration::departments::types::DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: serde_json::json!({
            "source": source,
            "message": translation.translated_content,
            "original_message": translation.original_content,
            "translated_from_language": translation.source_language,
            "generated_response": draft_reply,
            "feature_type": "ambassador_reply",
            "sender_id": sender_id,
            "inbox_message_id": inbox_id,
        }),
    };

    let orchestrator_clone = state.orchestrator.clone();
    tokio::spawn(async move {
        let _ = orchestrator_clone.dispatch_event(event).await;
    });
}
