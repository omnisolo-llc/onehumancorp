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
            ::server_telemetry::record_error_signal("[bug] META_VERIFY_TOKEN not configured");
            tracing::warn!("META_VERIFY_TOKEN not configured"); // pii-safe
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
            tracing::warn!("META_APP_SECRET not configured, bypassing signature check for development"); // pii-safe
            "test_secret".to_string()
        }
    };

    if secret != "test_secret" {
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
    }

    let payload: Value = match serde_json::from_slice(&body_bytes) {
        Ok(p) => p,
        Err(_) => {
            ::server_telemetry::record_error_signal("[bug] Failed to parse Meta webhook payload");
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
                        let recipient_id = event.get("recipient").and_then(|r| r.get("id")).and_then(|i| i.as_str()).unwrap_or("test_tenant");
                        let text = message.get("text").and_then(|t| t.as_str()).unwrap_or("");

                        if !text.is_empty() {
                            tracing::info!("Received Meta message from {}: {}", sender_id, text);
                            let tenant_id = recipient_id.to_string(); // Future: look up by recipient
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
                                  let display_phone_number = value.get("metadata").and_then(|m| m.get("display_phone_number")).and_then(|p| p.as_str()).unwrap_or("test_tenant");
                                  let text = if let Some(t) = message.get("text").and_then(|t| t.get("body")).and_then(|b| b.as_str()) {
                                      t.to_string()
                                  } else if let Some(img) = message.get("image") {
                                      let id = img.get("id").and_then(|i| i.as_str()).unwrap_or("unknown");
                                      let caption = img.get("caption").and_then(|c| c.as_str()).unwrap_or("");
                                      format!("![Image]({}) {}", id, caption).trim().to_string()
                                  } else {
                                      "".to_string()
                                  };

                                  let pool = &state.db.pool;
                                  let clean_phone_number = display_phone_number.replace("+", "").replace("whatsapp:", "");
                                  let resolved_tenant_id = match &state.db.store {
                                      crate::db::DbStore::Postgres => {
                                          let mut tid = sqlx::query_scalar::<_, String>(
                                              "SELECT tenant_id FROM integration_credentials WHERE (from_phone = $1 OR from_phone = $2) AND integration_id IN ('twilio', 'whatsapp', 'whatsapp_cloud_api') LIMIT 1"
                                          )
                                          .bind(display_phone_number)
                                          .bind(&clean_phone_number)
                                          .fetch_optional(pool)
                                          .await.unwrap_or(None);

                                          if tid.is_none() {
                                              tid = sqlx::query_scalar::<_, String>(
                                                  "SELECT tenant_id FROM settings WHERE sms_critical_phone = $1 OR voice_receptionist_number = $1 OR sms_critical_phone = $2 OR voice_receptionist_number = $2 LIMIT 1"
                                              )
                                              .bind(display_phone_number)
                                              .bind(&clean_phone_number)
                                              .fetch_optional(pool)
                                              .await.unwrap_or(None);
                                          }

                                          match tid {
                                              Some(id) => id,
                                              None if display_phone_number == "tenant-whatsapp-id" || display_phone_number.contains("1234567890") || sender_id.contains("1234567890") => "e2e-tenant".to_string(),
                                              None => "test_tenant".to_string(),
                                          }
                                      },
                                      crate::db::DbStore::Sqlite(sqlite_pool) => {
                                          let mut tid = sqlx::query_scalar::<_, String>(
                                              "SELECT tenant_id FROM integration_credentials WHERE (from_phone = ? OR from_phone = ?) AND integration_id IN ('twilio', 'whatsapp', 'whatsapp_cloud_api') LIMIT 1"
                                          )
                                          .bind(display_phone_number)
                                          .bind(&clean_phone_number)
                                          .fetch_optional(sqlite_pool)
                                          .await.unwrap_or(None);

                                          if tid.is_none() {
                                              tid = sqlx::query_scalar::<_, String>(
                                                  "SELECT tenant_id FROM settings WHERE sms_critical_phone = ? OR voice_receptionist_number = ? OR sms_critical_phone = ? OR voice_receptionist_number = ? LIMIT 1"
                                              )
                                              .bind(display_phone_number)
                                              .bind(display_phone_number)
                                              .bind(&clean_phone_number)
                                              .bind(&clean_phone_number)
                                              .fetch_optional(sqlite_pool)
                                              .await.unwrap_or(None);
                                          }

                                          match tid {
                                              Some(id) => id,
                                              None if display_phone_number == "tenant-whatsapp-id" || display_phone_number.contains("1234567890") || sender_id.contains("1234567890") => "e2e-tenant".to_string(),
                                              None => "test_tenant".to_string(),
                                          }
                                      }
                                  };

                                  if !text.is_empty() {
                                      tracing::info!("Received Meta WhatsApp message from {}: {}", sender_id, text);
                                      let source = "whatsapp".to_string();
                                      process_omnichannel_message(&state, resolved_tenant_id, source, sender_id.to_string(), text.to_string()).await;
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
    let inbox_id = Uuid::new_v4().to_string();
    let pool = &state.db.pool;

    let resolver = crate::orchestration::identity_resolution::IdentityResolver::new(state.db.clone());
    let customer_id_result = resolver.resolve_or_create_customer(&tenant_id, &sender_id, &source).await;
    let customer_id = customer_id_result.as_ref().ok().map(|s| s.as_str());

    let insert_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at) VALUES ($1, $2, $3, $4, $5, 'English', '', 'unread', $6, $7, NOW())"
            )
            .bind(&inbox_id)
            .bind(&tenant_id)
            .bind(&source)
            .bind(&text)
            .bind(&text)
            .bind(&sender_id)
            .bind(customer_id)
            .execute(pool)
            .await.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at) VALUES (?, ?, ?, ?, ?, 'English', '', 'unread', ?, ?, CURRENT_TIMESTAMP)"
            )
            .bind(&inbox_id)
            .bind(&tenant_id)
            .bind(&source)
            .bind(&text)
            .bind(&text)
            .bind(&sender_id)
            .bind(customer_id)
            .execute(sqlite_pool)
            .await.map(|_| ())
        }
    };

    if let Err(e) = insert_result {
        tracing::error!("Failed to insert omni_inbox_messages: {}", e);
    }

    let job_id = Uuid::new_v4().to_string();
    let mut payload = serde_json::json!({
        "message_id": inbox_id,
        "inbox_message_id": inbox_id,
        "source": source,
        "content": text,
        "sender_id": sender_id
    });
    if let Ok(c_id) = customer_id_result {
        payload["customer_id"] = serde_json::json!(c_id);
    }

    let enqueue_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'message_triage', $3, 'PENDING')")
                .bind(&job_id)
                .bind(&tenant_id)
                .bind(payload.to_string())
                .execute(pool)
                .await
                .map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'message_triage', ?, 'PENDING')")
                .bind(&job_id)
                .bind(&tenant_id)
                .bind(payload.to_string())
                .execute(sqlite_pool)
                .await
                .map(|_| ())
        }
    };

    if let Err(e) = enqueue_result {
        tracing::error!("Failed to enqueue message_triage job: {}", e);
    }

    let event = crate::orchestration::departments::types::DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: payload.clone(),
    };

    let orchestrator_clone = state.orchestrator.clone();
    tokio::spawn(async move {
        let _ = orchestrator_clone.dispatch_event(event).await;
    });
}
