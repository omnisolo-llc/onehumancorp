use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct WhatsAppCloudWebhookState {
    pub db: Arc<crate::db::DB>,
}

#[derive(Deserialize)]
pub struct VerifyQuery {
    #[serde(rename = "hub.mode")]
    pub mode: Option<String>,
    #[serde(rename = "hub.verify_token")]
    pub verify_token: Option<String>,
    #[serde(rename = "hub.challenge")]
    pub challenge: Option<String>,
}

pub fn whatsapp_cloud_routes() -> Router<WhatsAppCloudWebhookState> {
    Router::new()
        .route("/", get(verify_webhook_handler))
        .route("/", post(handle_webhook_handler))
}

pub async fn verify_webhook_handler(
    Query(query): Query<VerifyQuery>,
) -> impl IntoResponse {
    let expected_token = std::env::var("META_VERIFY_TOKEN").unwrap_or_default();

    if expected_token.is_empty() {
        tracing::error!("META_VERIFY_TOKEN is not set");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if let (Some(mode), Some(token), Some(challenge)) = (query.mode, query.verify_token, query.challenge) {
        if mode == "subscribe" && token == expected_token {
            return (StatusCode::OK, challenge).into_response();
        }
    }

    StatusCode::FORBIDDEN.into_response()
}

fn valid_meta_signature(secret: &str, signature_header: Option<&str>, body: &[u8]) -> bool {
    if secret.trim().is_empty() {
        return false;
    }
    let Some(signature_hex) = signature_header.and_then(|value| value.strip_prefix("sha256=")) else {
        return false;
    };
    let Ok(signature_bytes) = hex::decode(signature_hex) else {
        return false;
    };
    let Ok(mut mac) = Hmac::<Sha256>::new_from_slice(secret.as_bytes()) else {
        return false;
    };
    mac.update(body);
    mac.verify_slice(&signature_bytes).is_ok()
}

pub async fn handle_webhook_handler(
    headers: HeaderMap,
    State(state): State<WhatsAppCloudWebhookState>,
    body_bytes: axum::body::Bytes,
) -> impl IntoResponse {
    let secret = std::env::var("META_APP_SECRET").unwrap_or_default();
    if secret.is_empty() {
        tracing::error!("META_APP_SECRET is not set");
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let signature_header = headers.get("x-hub-signature-256")
        .and_then(|value| value.to_str().ok());

    if !valid_meta_signature(&secret, signature_header, &body_bytes) {
        tracing::warn!("WhatsApp Cloud webhook signature verification failed");
        return StatusCode::UNAUTHORIZED.into_response();
    }

    let payload: Value = match serde_json::from_slice(&body_bytes) {
        Ok(p) => p,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    if let Some(entries) = payload.get("entry").and_then(|e| e.as_array()) {
        for entry in entries {
            if let Some(changes) = entry.get("changes").and_then(|c| c.as_array()) {
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
                                  } else if let Some(audio) = message.get("audio") {
                                      let id = audio.get("id").and_then(|i| i.as_str()).unwrap_or("unknown");
                                      format!("[Audio]({})", id)
                                  } else if let Some(video) = message.get("video") {
                                      let id = video.get("id").and_then(|i| i.as_str()).unwrap_or("unknown");
                                      format!("[Video]({})", id)
                                  } else if let Some(doc) = message.get("document") {
                                      let id = doc.get("id").and_then(|i| i.as_str()).unwrap_or("unknown");
                                      let filename = doc.get("filename").and_then(|c| c.as_str()).unwrap_or("document");
                                      format!("[Document: {}]({})", filename, id)
                                  } else if let Some(interactive) = message.get("interactive") {
                                      if let Some(button_reply) = interactive.get("button_reply") {
                                          button_reply.get("title").and_then(|t| t.as_str()).unwrap_or("Button Clicked").to_string()
                                      } else if let Some(list_reply) = interactive.get("list_reply") {
                                          list_reply.get("title").and_then(|t| t.as_str()).unwrap_or("List Item Selected").to_string()
                                      } else {
                                          "Interactive Message".to_string()
                                      }
                                  } else {
                                      "Unsupported message type".to_string()
                                  };

                                  if !text.is_empty() {
                                      tracing::info!("Received Meta WhatsApp message from {}: {}", sender_id, text);
                                      process_omnichannel_message(&state, display_phone_number.to_string(), sender_id.to_string(), text.to_string()).await;
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

async fn process_omnichannel_message(state: &WhatsAppCloudWebhookState, display_phone_number: String, sender_id: String, text: String) {
    let pool = &state.db.pool;
    let clean_phone_number = display_phone_number.replace("+", "").replace("whatsapp:", "");
    let resolved_tenant_id = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query_scalar::<_, String>(
                "SELECT tenant_id FROM integration_credentials WHERE (from_phone = $1 OR from_phone = $2) AND integration_id = 'whatsapp_cloud_api' LIMIT 1"
            )
            .bind(&display_phone_number)
            .bind(&clean_phone_number)
            .fetch_optional(pool)
            .await.unwrap_or(None)
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query_scalar::<_, String>(
                "SELECT tenant_id FROM integration_credentials WHERE (from_phone = ? OR from_phone = ?) AND integration_id = 'whatsapp_cloud_api' LIMIT 1"
            )
            .bind(&display_phone_number)
            .bind(&clean_phone_number)
            .fetch_optional(sqlite_pool)
            .await.unwrap_or(None)
        }
    };

    let tenant_id = resolved_tenant_id.unwrap_or_else(|| "test_tenant".to_string());
    let source = "whatsapp_cloud_api".to_string();

    let inbox_id = Uuid::new_v4().to_string();

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
}
