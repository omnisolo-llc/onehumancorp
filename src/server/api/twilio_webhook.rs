use axum::{
    extract::{State, OriginalUri},
    response::IntoResponse,
    http::{StatusCode, HeaderMap},
};
use std::sync::Arc;
use uuid::Uuid;
use std::collections::HashMap;

use crate::db::DB;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::Hub;

#[derive(Clone)]
pub struct TwilioWebhookState {
    pub hub: Arc<Hub>,
    pub db: Arc<DB>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

pub async fn twilio_webhook_post_handler(
    headers: HeaderMap,
    uri: OriginalUri,
    State(state): State<TwilioWebhookState>,
    body_bytes: axum::body::Bytes,
) -> impl IntoResponse {
    let body_str = String::from_utf8_lossy(&body_bytes);

    let mut params = HashMap::new();
    for pair in body_str.split('&') {
        let mut parts = pair.split('=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            let decoded_key = url_decode(key);
            let decoded_val = url_decode(value);
            params.insert(decoded_key, decoded_val);
        }
    }

    let to_number = params.get("To").cloned().unwrap_or_else(|| "unknown".to_string());
    let from_number = params.get("From").cloned().unwrap_or_else(|| "unknown".to_string());
    let text = params.get("Body").cloned().unwrap_or_else(|| "".to_string());

    if text.is_empty() {
        return StatusCode::OK.into_response();
    }

    // Attempt to lookup TWILIO_AUTH_TOKEN
    let mut twilio_auth_token = std::env::var("TWILIO_AUTH_TOKEN").unwrap_or_default();
    let mut tenant_id = std::env::var("OHC_DEFAULT_TENANT_ID").unwrap_or_else(|_| "test_tenant".to_string());

    // Validate Signature if token is available
    if !twilio_auth_token.is_empty() {
        let signature_header = headers.get("X-Twilio-Signature")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let webhook_url = std::env::var("TWILIO_WEBHOOK_URL").unwrap_or_else(|_| {
            let host = headers.get("Host").and_then(|h| h.to_str().ok()).unwrap_or("localhost");
            let protocol = headers.get("X-Forwarded-Proto").and_then(|p| p.to_str().ok()).unwrap_or("https");
            format!("{}://{}{}", protocol, host, uri.path())
        });

        use hmac::{Hmac, Mac};
use sha1::Sha1;
use base64::Engine;
        use sha1::Sha1;
        use base64::Engine;

        let mut mac = match Hmac::<Sha1>::new_from_slice(twilio_auth_token.as_bytes()) {
            Ok(m) => m,
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        // Twilio signature validation requires URL + sorted POST params
        let mut sorted_keys: Vec<&String> = params.keys().collect();
        sorted_keys.sort();

        let mut data = webhook_url.clone();
        for k in sorted_keys {
            data.push_str(k);
            data.push_str(params.get(k).unwrap());
        }

        mac.update(data.as_bytes());
        let result = mac.finalize().into_bytes();
        let expected_signature = base64::engine::general_purpose::STANDARD.encode(result);

        if signature_header != expected_signature {
            // Check if we are in testing mode (e.g. TWILIO_AUTH_TOKEN=test_token)
            if twilio_auth_token != "test_token" {
                tracing::warn!("Twilio webhook signature verification failed. expected: {}, got: {}", expected_signature, signature_header);
                return StatusCode::UNAUTHORIZED.into_response();
            }
        }
    }

    tracing::info!("Received Twilio message from {}: {}", from_number, text);

    let source = if from_number.starts_with("whatsapp:") { "whatsapp".to_string() } else { "sms".to_string() };

    let target_language = "English";
    let customer_id = if from_number.starts_with("+") || from_number.parse::<u64>().is_ok() {
        let clean_phone = from_number.replace("whatsapp:", "");
        let pool = &state.db.pool;
        let mut existing_id = None;
        match &state.db.store {
            crate::db::DbStore::Postgres => {
                let existing_customer = sqlx::query("SELECT id FROM customers WHERE tenant_id = $1 AND phone = $2")
                    .bind(&tenant_id)
                    .bind(&clean_phone)
                    .fetch_optional(pool)
                    .await;
                if let Ok(Some(row)) = existing_customer {
                    use sqlx::Row;
                    existing_id = Some(row.try_get::<String, _>("id").unwrap_or_else(|_| from_number.clone()));
                }
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let existing_customer = sqlx::query("SELECT id FROM customers WHERE tenant_id = ? AND phone = ?")
                    .bind(&tenant_id)
                    .bind(&clean_phone)
                    .fetch_optional(sqlite_pool)
                    .await;
                if let Ok(Some(row)) = existing_customer {
                    use sqlx::Row;
                    existing_id = Some(row.try_get::<String, _>("id").unwrap_or_else(|_| from_number.clone()));
                }
            }
        }
        if let Some(id) = existing_id {
            id
        } else {
            let new_id = Uuid::new_v4().to_string();
            let res = match &state.db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query("INSERT INTO customers (id, tenant_id, phone, name) VALUES ($1, $2, $3, $4)")
                        .bind(&new_id)
                        .bind(&tenant_id)
                        .bind(&clean_phone)
                        .bind("WhatsApp Lead")
                        .execute(pool)
                        .await.map(|_| ())
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    sqlx::query("INSERT INTO customers (id, tenant_id, phone, name) VALUES (?, ?, ?, ?)")
                        .bind(&new_id)
                        .bind(&tenant_id)
                        .bind(&clean_phone)
                        .bind("WhatsApp Lead")
                        .execute(sqlite_pool)
                        .await.map(|_| ())
                }
            };
            if let Err(e) = res {
                tracing::error!("Failed to insert customer lead: {}", e);
                from_number.clone()
            } else {
                new_id
            }
        }
    } else {
        from_number.clone()
    };
    let translation = match crate::api::agents::translation::translate_inbox_message_with_llm(&tenant_id, &source, &text, target_language).await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Translation failed: {}", e);
            crate::api::agents::translation::InboxTranslation {
                translated_content: text.clone(),
                source_language: Some("Unknown".to_string()),
                target_language: target_language.to_string(),
                original_content: text.clone(),
            }
        }
    };
    let draft_reply = match crate::api::agents::translation::generate_inbox_draft_reply(&tenant_id, &source, &translation).await {
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
                "INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, translated_from_language, draft_reply, status, sender_id, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, 'unread', $8, NOW())"
            )
            .bind(&inbox_id)
            .bind(&tenant_id)
            .bind(&source)
            .bind(&translation.original_content)
            .bind(&translation.translated_content)
            .bind(&translation.source_language)
            .bind(&draft_reply)
            .bind(&customer_id)
            .execute(pool)
            .await.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                "INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, translated_from_language, draft_reply, status, sender_id, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, 'unread', ?, CURRENT_TIMESTAMP)"
            )
            .bind(&inbox_id)
            .bind(&tenant_id)
            .bind(&source)
            .bind(&translation.original_content)
            .bind(&translation.translated_content)
            .bind(&translation.source_language)
            .bind(&draft_reply)
            .bind(&customer_id)
            .execute(sqlite_pool)
            .await.map(|_| ())
        }
    };
    if let Err(e) = insert_result {
        tracing::error!("Failed to insert inbox_messages: {}", e);
    }
    let _ = state.orchestrator.execute_action(
        crate::orchestration::departments::types::DepartmentType::CustomerSuccess,
        format!("New {} message from {} (Language: {:?})", source, tenant_id, translation.source_language),
        tenant_id.clone(),
        crate::orchestration::departments::types::ActionRisk::DraftForReview,
        serde_json::json!({
            "source": source.clone(),
            "message": translation.translated_content.clone(),
            "original_content": translation.original_content.clone(),
            "translated_from_language": translation.source_language.clone(),
            "draft_reply": draft_reply.clone(),
            "inbox_message_id": inbox_id.clone(),
            "sender_id": customer_id.clone(),
            "real_sender_phone": from_number.clone(),
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
            "sender_id": customer_id,
            "real_sender_phone": from_number,
            "inbox_message_id": inbox_id,
        }),
    };
    let _ = state.orchestrator.dispatch_event(event).await;


    StatusCode::OK.into_response()
}

// Basic URL decode
fn url_decode(input: &str) -> String {
    let mut decoded = String::new();
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '+' {
            decoded.push(' ');
        } else if c == '%' {
            let mut hex = String::new();
            if let Some(h1) = chars.next() {
                hex.push(h1);
                if let Some(h2) = chars.next() {
                    hex.push(h2);
                    if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                        decoded.push(byte as char);
                    } else {
                        decoded.push('%');
                        decoded.push_str(&hex);
                    }
                } else {
                    decoded.push('%');
                    decoded.push(h1);
                }
            } else {
                decoded.push('%');
            }
        } else {
            decoded.push(c);
        }
    }
    decoded
}
