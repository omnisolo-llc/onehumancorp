use axum::{
    extract::State,
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use uuid::Uuid;
use std::collections::HashMap;

use crate::db::DB;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::hub::Hub;
use crate::orchestration::identity_resolution::IdentityResolver;

#[derive(Clone)]
pub struct TwilioWebhookState {
    pub hub: Arc<Hub>,
    pub db: Arc<DB>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
    pub voice_engine: Option<Arc<crate::voice::VoiceAIEdgeEngine>>,
    pub voice_router: Option<Arc<crate::voice::VoiceContextRouter>>,
}

pub async fn twilio_webhook_post_handler(
    State(state): State<TwilioWebhookState>,
    body_bytes: axum::body::Bytes,
) -> impl IntoResponse {
    let body_str = String::from_utf8_lossy(&body_bytes);

    // Parse form url-encoded body manually (split by & and =)
    let mut params = HashMap::new();
    for pair in body_str.split('&') {
        let mut parts = pair.split('=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            let decoded_key = url_decode(key);
            let decoded_val = url_decode(value);
            params.insert(decoded_key, decoded_val);
        }
    }

    let sender_id = params.get("From").cloned().unwrap_or_else(|| "unknown".to_string());
    let _to_number = params.get("To").cloned().unwrap_or_else(|| "unknown".to_string());
    let mut text = params.get("Body").cloned().unwrap_or_else(|| "".to_string());

    let num_media: usize = params.get("NumMedia").and_then(|s| s.parse().ok()).unwrap_or(0);
    for i in 0..num_media {
        if let Some(media_url) = params.get(&format!("MediaUrl{}", i)) {
            let media_type = params.get(&format!("MediaContentType{}", i)).cloned().unwrap_or_else(|| "unknown".to_string());
            text.push_str(&format!(" [Media: {} - {}]", media_type, media_url));
        }
    }

    if !text.is_empty() || num_media > 0 {
        tracing::info!("Received Twilio message from {}: {}", sender_id, text);

        let pool = &state.db.pool;

        // Find the correct tenant by mapping the `To` number
        let tenant_id = match &state.db.store {
            crate::db::DbStore::Postgres => {
                match sqlx::query_scalar::<_, String>(
                    "SELECT tenant_id FROM settings WHERE sms_critical_phone = $1 OR voice_receptionist_number = $1 LIMIT 1"
                )
                .bind(&_to_number)
                .fetch_optional(pool)
                .await {
                    Ok(Some(id)) => id,
                    _ => "test_tenant".to_string(), // Fallback if no specific tenant is found
                }
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                match sqlx::query_scalar::<_, String>(
                    "SELECT tenant_id FROM settings WHERE sms_critical_phone = ? OR voice_receptionist_number = ? LIMIT 1"
                )
                .bind(&_to_number)
                .bind(&_to_number)
                .fetch_optional(sqlite_pool)
                .await {
                    Ok(Some(id)) => id,
                    _ => "test_tenant".to_string(),
                }
            }
        };

        let inbox_id = Uuid::new_v4().to_string();

        // Twilio sends whatsapp messages with "whatsapp:" prefix in the From/To fields
        // but we can also just hardcode the source to whatsapp for this specific webhook if it's meant exclusively for whatsapp
        let source = if sender_id.starts_with("whatsapp:") { "whatsapp".to_string() } else { "sms".to_string() };

        // Identity Resolution
        let resolver = IdentityResolver::new(state.db.clone());
        let clean_sender_id = sender_id.replace("whatsapp:", "");
        let customer_id_result = resolver.resolve_or_create_customer(&tenant_id, &clean_sender_id, &source).await;
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
                .bind(&clean_sender_id)
                .bind(&customer_id)
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
                .bind(&clean_sender_id)
                .bind(&customer_id)
                .execute(sqlite_pool)
                .await.map(|_| ())
            }
        };

        if let Err(e) = insert_result {
            tracing::error!("Failed to insert omni_inbox_messages: {}", e);
        }

        // Enqueue to ohc_job_queue
        let job_id = Uuid::new_v4().to_string();
        let mut payload_json = serde_json::json!({
            "message_id": inbox_id,
            "inbox_message_id": inbox_id,
            "source": source,
            "content": text,
            "sender_id": clean_sender_id
        });

        if let Ok(c_id) = &customer_id_result {
            payload_json["customer_id"] = serde_json::json!(c_id);
        }

        let enqueue_result = match &state.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'message_triage', $3, 'PENDING')")
                    .bind(&job_id)
                    .bind(&tenant_id)
                    .bind(payload_json.to_string())
                    .execute(&state.db.pool)
                    .await
                    .map(|_| ())
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'message_triage', ?, 'PENDING')")
                    .bind(&job_id)
                    .bind(&tenant_id)
                    .bind(payload_json.to_string())
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
            payload: payload_json,
        };

        let orchestrator_clone = state.orchestrator.clone();
        tokio::spawn(async move {
            let _ = orchestrator_clone.dispatch_event(event).await;
        });
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_decode() {
        assert_eq!(url_decode("Hello+World"), "Hello World");
        assert_eq!(url_decode("Hello%20World"), "Hello World");
        assert_eq!(url_decode("whatsapp%3A%2B1234567890"), "whatsapp:+1234567890");
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct TwilioVoicePayload {
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "To")]
    pub to: String,
    #[serde(rename = "CallSid")]
    pub call_sid: String,
    #[serde(rename = "CallStatus")]
    pub call_status: Option<String>,
    #[serde(rename = "SpeechResult")]
    pub speech_result: Option<String>,
}

pub async fn twilio_voice_post_handler(
    State(state): State<TwilioWebhookState>,
    axum::extract::Form(payload): axum::extract::Form<TwilioVoicePayload>,
) -> impl IntoResponse {
    let call_status = payload.call_status.unwrap_or_else(|| "ringing".to_string());

    let tenant_id = match &state.db.store {
        crate::db::DbStore::Postgres => {
            match sqlx::query_scalar::<_, String>(
                "SELECT tenant_id FROM settings WHERE sms_critical_phone = $1 OR voice_receptionist_number = $1 LIMIT 1"
            )
            .bind(&payload.to)
            .fetch_optional(&state.db.pool)
            .await {
                Ok(Some(id)) => id,
                _ => "test_tenant".to_string(),
            }
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            match sqlx::query_scalar::<_, String>(
                "SELECT tenant_id FROM settings WHERE sms_critical_phone = ? OR voice_receptionist_number = ? LIMIT 1"
            )
            .bind(&payload.to)
            .bind(&payload.to)
            .fetch_optional(sqlite_pool)
            .await {
                Ok(Some(id)) => id,
                _ => "test_tenant".to_string(),
            }
        }
    };

    if call_status == "completed" || call_status == "failed" || call_status == "canceled" || call_status == "no-answer" {
        if let Some(engine) = &state.voice_engine {
            engine.end_call(&payload.call_sid).await;

            let transcript = engine.get_call_transcript(&payload.call_sid).await;

            let inbox_id = Uuid::new_v4().to_string();
            let source = "voice".to_string();
            let clean_sender_id = payload.from.clone();

            let insert_result = match &state.db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query(
                        "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, created_at) VALUES ($1, $2, $3, $4, $5, 'English', '', 'unread', $6, NOW())"
                    )
                    .bind(&inbox_id)
                    .bind(&tenant_id)
                    .bind(&source)
                    .bind(&transcript)
                    .bind(&transcript)
                    .bind(&clean_sender_id)
                    .execute(&state.db.pool)
                    .await.map(|_| ())
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    sqlx::query(
                        "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, created_at) VALUES (?, ?, ?, ?, ?, 'English', '', 'unread', ?, CURRENT_TIMESTAMP)"
                    )
                    .bind(&inbox_id)
                    .bind(&tenant_id)
                    .bind(&source)
                    .bind(&transcript)
                    .bind(&transcript)
                    .bind(&clean_sender_id)
                    .execute(sqlite_pool)
                    .await.map(|_| ())
                }
            };

            if let Err(e) = insert_result {
                tracing::error!("Failed to insert omni_inbox_messages for voice call: {}", e);
            }

            let job_id = Uuid::new_v4().to_string();
            let payload_json = serde_json::json!({
                "message_id": inbox_id,
                "inbox_message_id": inbox_id,
                "source": source,
                "content": transcript,
                "sender_id": clean_sender_id
            });

            let enqueue_result = match &state.db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'message_triage', $3, 'PENDING')")
                        .bind(&job_id)
                        .bind(&tenant_id)
                        .bind(payload_json.to_string())
                        .execute(&state.db.pool)
                        .await
                        .map(|_| ())
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'message_triage', ?, 'PENDING')")
                        .bind(&job_id)
                        .bind(&tenant_id)
                        .bind(payload_json.to_string())
                        .execute(sqlite_pool)
                        .await
                        .map(|_| ())
                }
            };
            if let Err(e) = enqueue_result {
                tracing::error!("Failed to enqueue message_triage job for voice: {}", e);
            }
        }

        let twiml = "<Response></Response>";
        return (StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "text/xml")], twiml.to_string()).into_response();
    }

    let mut ai_reply = "Hello! Thank you for calling. How can I help you today?".to_string();

    if let Some(engine) = &state.voice_engine {
        if let Some(speech) = payload.speech_result {
            if let Some(router) = &state.voice_router {
                ai_reply = router.process_user_input(&payload.call_sid, &speech, &payload.to).await;
            }
        } else {
            engine.handle_incoming_call("merchant_123", &payload.from, Some(payload.call_sid.clone())).await;
        }
    }

    let twiml = format!(
        "<Response><Say>{}</Say><Gather input=\"speech\" action=\"/api/v1/webhooks/twilio/voice\" speechTimeout=\"auto\"/></Response>",
        ai_reply
    );

    (StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "text/xml")], twiml).into_response()
}
