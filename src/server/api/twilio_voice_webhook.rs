use axum::{
    extract::State,
    response::IntoResponse,
    http::StatusCode,
    http::header,
};
use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;

use crate::db::DB;
use crate::voice::VoiceAIEdgeEngine;
use crate::voice::VoiceContextRouter;
use crate::hub::Hub;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::identity_resolution::IdentityResolver;

#[derive(Clone)]
pub struct TwilioVoiceWebhookState {
    pub db: Arc<DB>,
    pub engine: Arc<VoiceAIEdgeEngine>,
    pub router: Arc<VoiceContextRouter>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
    pub hub: Arc<Hub>,
}

pub async fn twilio_voice_webhook_handler(
    State(state): State<TwilioVoiceWebhookState>,
    body_bytes: axum::body::Bytes,
) -> impl IntoResponse {
    let body_str = String::from_utf8_lossy(&body_bytes);

    let mut params = HashMap::new();
    for pair in body_str.split('&') {
        let mut parts = pair.split('=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            params.insert(url_decode(key), url_decode(value));
        }
    }

    let call_sid = params.get("CallSid").cloned().unwrap_or_else(|| Uuid::new_v4().to_string());
    let from_number = params.get("From").cloned().unwrap_or_else(|| "unknown".to_string());
    let to_number = params.get("To").cloned().unwrap_or_else(|| "unknown".to_string());
    let speech_result = params.get("SpeechResult").cloned().unwrap_or_default();

    let reply = if speech_result.is_empty() {
        // New call
        let _ = state.engine.handle_incoming_call(&to_number, &from_number).await;
        // In handle_incoming_call, it generates "Hello! Thank you for calling. How can I help you today?"
        // We'll just return it directly here to Twilio since handle_incoming_call doesn't return the text
        "Hello! Thank you for calling. How can I help you today?".to_string()
    } else {
        // Ongoing call
        state.router.process_user_input(&call_sid, &speech_result, &to_number).await
    };

    let twiml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
        <Response>\n\
            <Say>{}</Say>\n\
            <Gather input=\"speech\" action=\"/api/v1/webhooks/twilio/voice\" speechTimeout=\"auto\">\n\
                <Say>Are you still there?</Say>\n\
            </Gather>\n\
        </Response>",
        reply
    );

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/xml")],
        twiml
    ).into_response()
}

pub async fn twilio_voice_status_webhook_handler(
    State(state): State<TwilioVoiceWebhookState>,
    body_bytes: axum::body::Bytes,
) -> impl IntoResponse {
    let body_str = String::from_utf8_lossy(&body_bytes);

    let mut params = HashMap::new();
    for pair in body_str.split('&') {
        let mut parts = pair.split('=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            params.insert(url_decode(key), url_decode(value));
        }
    }

    let call_sid = params.get("CallSid").cloned().unwrap_or_default();
    let call_status = params.get("CallStatus").cloned().unwrap_or_default();
    let from_number = params.get("From").cloned().unwrap_or_else(|| "unknown".to_string());
    let to_number = params.get("To").cloned().unwrap_or_else(|| "unknown".to_string());

    if call_status == "completed" || call_status == "failed" || call_status == "no-answer" || call_status == "canceled" || call_status == "busy" {
        state.engine.end_call(&call_sid).await;

        let transcripts = state.engine.get_transcripts(&call_sid).await;

        // We only create an inbox message if we actually had a conversation
        if transcripts.len() > 1 {
            let pool = &state.db.pool;

            // Find the correct tenant
            let tenant_id = match &state.db.store {
                crate::db::DbStore::Postgres => {
                    match sqlx::query_scalar::<_, String>(
                        "SELECT tenant_id FROM settings WHERE sms_critical_phone = $1 OR voice_receptionist_number = $1 LIMIT 1"
                    )
                    .bind(&to_number)
                    .fetch_optional(pool)
                    .await {
                        Ok(Some(id)) => id,
                        _ => "test_tenant".to_string(),
                    }
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    match sqlx::query_scalar::<_, String>(
                        "SELECT tenant_id FROM settings WHERE sms_critical_phone = ? OR voice_receptionist_number = ? LIMIT 1"
                    )
                    .bind(&to_number)
                    .bind(&to_number)
                    .fetch_optional(sqlite_pool)
                    .await {
                        Ok(Some(id)) => id,
                        _ => "test_tenant".to_string(),
                    }
                }
            };

            let mut full_transcript = String::new();
            for t in transcripts {
                full_transcript.push_str(&format!("{}: {}\n", t.role, t.text));
            }

            let inbox_id = Uuid::new_v4().to_string();
            let source = "voice".to_string();

            // Identity Resolution
            let resolver = IdentityResolver::new(state.db.clone());
            let customer_id_result = resolver.resolve_or_create_customer(&tenant_id, &from_number, &source).await;
            let customer_id = customer_id_result.as_ref().ok().map(|s| s.as_str());

            let insert_result = match &state.db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query(
                        "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at) VALUES ($1, $2, $3, $4, $5, 'English', '', 'unread', $6, $7, NOW())"
                    )
                    .bind(&inbox_id)
                    .bind(&tenant_id)
                    .bind(&source)
                    .bind(&full_transcript)
                    .bind(&full_transcript)
                    .bind(&from_number)
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
                    .bind(&full_transcript)
                    .bind(&full_transcript)
                    .bind(&from_number)
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
                "content": full_transcript,
                "sender_id": from_number
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
    }

    StatusCode::OK.into_response()
}

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
