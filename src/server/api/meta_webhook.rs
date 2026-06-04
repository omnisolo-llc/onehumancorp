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
use crate::hub::{Hub, HubEvent};
use crate::orchestration::departments::types::DepartmentType;
use crate::orchestration::departments::types::ActionRisk;
use uuid::Uuid;
use crate::db::get_pool;
use chrono::Utc;

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
            tracing::error!("META_VERIFY_TOKEN not configured");
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
    State(orchestrator): State<Arc<crate::orchestration::departments::orchestrator::DepartmentOrchestrator>>,
    headers: HeaderMap,
    body_bytes: axum::body::Bytes,
) -> impl IntoResponse {
    // 1. Verify Signature
    let secret = match std::env::var("META_APP_SECRET") {
        Ok(s) if !s.is_empty() => s,
        _ => {
            ::server_telemetry::record_error_signal("META_APP_SECRET not configured, refusing to process webhook");
            tracing::error!("META_APP_SECRET not configured, refusing to process webhook");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
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
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    mac.update(&body_bytes);
    if mac.verify_slice(&signature_bytes).is_err() {
        tracing::warn!("Meta webhook signature verification failed");
        return StatusCode::UNAUTHORIZED.into_response();
    }

    // 2. Parse Payload
    let payload: Value = match serde_json::from_slice(&body_bytes) {
        Ok(p) => p,
        Err(_) => {
            ::server_telemetry::record_error_signal("Failed to parse Meta webhook payload");
            tracing::error!("Failed to parse Meta webhook payload");
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    // 3. Extract Message & Emits HubEvent
    if let Some(entries) = payload.get("entry").and_then(|e| e.as_array()) {
        for entry in entries {
            if let Some(messaging) = entry.get("messaging").and_then(|m| m.as_array()) {
                for event in messaging {
                    if let Some(message) = event.get("message") {
                        let sender_id = event.get("sender").and_then(|s| s.get("id")).and_then(|i| i.as_str()).unwrap_or("unknown");
                        let text = message.get("text").and_then(|t| t.as_str()).unwrap_or("");

                        if !text.is_empty() {
                            tracing::info!("Received Meta message from {}: {}", sender_id, text);
                            let payload_tenant_id = "default".to_string(); // In a real app we map sender_id/page_id to tenant_id
                            let source = "instagram".to_string();
                            let message = text.to_string();

                            // Generate draft reply (same as in webhook.rs)
                            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                            let draft_reply = if !api_key.is_empty() {
                                let business_context = "A friendly bakery that sells vegan celebration cakes and classes."; // mocked context
                                let prompt = format!(
                                    "Write one concise, warm customer-service reply. Business context: {} Customer message: {}",
                                    business_context, message
                                );
                                let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);
                                let client = crate::minimax::MinimaxClient::new(api_key);
                                client.reason(&compressed_prompt).await.unwrap_or_else(|_| "Draft generation failed.".to_string())
                            } else {
                                "Thank you for reaching out! We will get back to you shortly.".to_string()
                            };

                            let id = Uuid::new_v4().to_string();
                            let status = "pending";
                            let pool = get_pool();
                            if let Ok(mut tx) = pool.begin().await {
                                let _ = crate::common::auth_utils::set_org_context(&mut *tx, &payload_tenant_id).await;
                                let _ = sqlx::query(
                                    "INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status) VALUES ($1, $2, $3, $4, $5, $6)"
                                )
                                .bind(&id)
                                .bind(&payload_tenant_id)
                                .bind(&source)
                                .bind(&message)
                                .bind(&draft_reply)
                                .bind(&status)
                                .execute(&mut *tx)
                                .await;
                                let _ = tx.commit().await;
                            }

                            let description = format!("Incoming message from {}: {}", source, message);
                            let _ = orchestrator.execute_action(
                                DepartmentType::CustomerSuccess,
                                description,
                                payload_tenant_id,
                                ActionRisk::DraftForReview,
                                serde_json::json!({
                                    "source": source,
                                    "message": message,
                                    "draft_reply": draft_reply,
                                    "inbox_message_id": id,
                                }),
                            ).await;
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
                                      let payload_tenant_id = "default".to_string(); // In a real app we map sender_id/page_id to tenant_id
                                      let source = "whatsapp".to_string();
                                      let message = text.to_string();

                                      // Generate draft reply
                                      let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                                      let draft_reply = if !api_key.is_empty() {
                                          let business_context = "A friendly bakery that sells vegan celebration cakes and classes."; // mocked context
                                          let prompt = format!(
                                              "Write one concise, warm customer-service reply. Business context: {} Customer message: {}",
                                              business_context, message
                                          );
                                          let compressed_prompt = crate::pricing::compression::reduce_tokens(&prompt);
                                          let client = crate::minimax::MinimaxClient::new(api_key);
                                          client.reason(&compressed_prompt).await.unwrap_or_else(|_| "Draft generation failed.".to_string())
                                      } else {
                                          "Thank you for reaching out! We will get back to you shortly.".to_string()
                                      };

                                      let id = Uuid::new_v4().to_string();
                                      let status = "pending";
                                      let pool = get_pool();
                                      if let Ok(mut tx) = pool.begin().await {
                                          let _ = crate::common::auth_utils::set_org_context(&mut *tx, &payload_tenant_id).await;
                                          let _ = sqlx::query(
                                              "INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status) VALUES ($1, $2, $3, $4, $5, $6)"
                                          )
                                          .bind(&id)
                                          .bind(&payload_tenant_id)
                                          .bind(&source)
                                          .bind(&message)
                                          .bind(&draft_reply)
                                          .bind(&status)
                                          .execute(&mut *tx)
                                          .await;
                                          let _ = tx.commit().await;
                                      }

                                      let description = format!("Incoming message from {}: {}", source, message);
                                      let _ = orchestrator.execute_action(
                                          DepartmentType::CustomerSuccess,
                                          description,
                                          payload_tenant_id,
                                          ActionRisk::DraftForReview,
                                          serde_json::json!({
                                              "source": source,
                                              "message": message,
                                              "draft_reply": draft_reply,
                                              "inbox_message_id": id,
                                          }),
                                      ).await;
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
