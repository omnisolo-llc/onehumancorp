use axum::{
    extract::State,
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use uuid::Uuid;
use std::collections::HashMap;

use crate::db::DB;
use ::server_utils::url::url_decode;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::hub::Hub;
use crate::orchestration::identity_resolution::IdentityResolver;
use crate::voice::{VoiceAIEdgeEngine, VoiceContextRouter};
use ::server_integrations_twilio::provider::TwilioProvider;

#[derive(Clone)]
pub struct TwilioVoiceWebhookState {
    pub hub: Arc<Hub>,
    pub db: Arc<DB>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
    pub voice_engine: Arc<VoiceAIEdgeEngine>,
    pub twilio: Arc<TwilioProvider>,
}

pub async fn twilio_voice_incoming_handler(
    State(state): State<TwilioVoiceWebhookState>,
    body_bytes: axum::body::Bytes,
) -> impl IntoResponse {
    let body_str = String::from_utf8_lossy(&body_bytes);
    let params = parse_form_urlencoded(&body_str);

    let caller_phone = params.get("From").cloned().unwrap_or_else(|| "unknown".to_string());
    let to_number = params.get("To").cloned().unwrap_or_else(|| "unknown".to_string());
    let call_sid = params.get("CallSid").cloned().unwrap_or_else(|| Uuid::new_v4().to_string());

    let pool = &state.db.pool;

    let tenant_id = match &state.db.store {
        crate::db::DbStore::Postgres => {
            match sqlx::query_scalar::<_, String>(
                "SELECT tenant_id FROM settings WHERE voice_receptionist_number = $1 LIMIT 1"
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
                "SELECT tenant_id FROM settings WHERE voice_receptionist_number = ? LIMIT 1"
            )
            .bind(&to_number)
            .fetch_optional(sqlite_pool)
            .await {
                Ok(Some(id)) => id,
                _ => "test_tenant".to_string(),
            }
        }
    };

    // Use CallSid as session_id to maintain state across webhooks
    let mut calls = state.voice_engine.active_calls.lock().await;
    // Check if it already exists, if not create it
    if !calls.iter().any(|c| c.session_id == call_sid) {
        let session = crate::voice::CallSession {
            session_id: call_sid.clone(),
            merchant_id: tenant_id.clone(),
            caller_phone: caller_phone.clone(),
            start_time: chrono::Utc::now(),
            end_time: None,
            status: crate::voice::CallStatus::InProgress,
        };
        calls.push(session);
    }
    drop(calls);

    state.voice_engine.log_transcript(&call_sid, "AI", "Hello! Thank you for calling. How can I help you today?").await;

    // Return TwiML
    let twiml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Response>
    <Say>Hello! Thank you for calling. How can I help you today?</Say>
    <Gather input="speech" action="/api/v1/webhooks/twilio_voice/gather" speechTimeout="auto" />
</Response>"#;

    ([(axum::http::header::CONTENT_TYPE, "application/xml")], twiml).into_response()
}

pub async fn twilio_voice_gather_handler(
    State(state): State<TwilioVoiceWebhookState>,
    body_bytes: axum::body::Bytes,
) -> impl IntoResponse {
    let body_str = String::from_utf8_lossy(&body_bytes);
    let params = parse_form_urlencoded(&body_str);

    let call_sid = params.get("CallSid").cloned().unwrap_or_default();
    let to_number = params.get("To").cloned().unwrap_or_default();
    let speech_result = params.get("SpeechResult").cloned().unwrap_or_default();

    if speech_result.is_empty() {
        let twiml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Response>
    <Say>I didn't quite catch that. Could you repeat?</Say>
    <Gather input="speech" action="/api/v1/webhooks/twilio_voice/gather" speechTimeout="auto" />
</Response>"#;
        return ([(axum::http::header::CONTENT_TYPE, "application/xml")], twiml.to_string()).into_response();
    }

    let voice_router = VoiceContextRouter::new(state.voice_engine.clone(), state.twilio.clone());
    let ai_response = voice_router.process_user_input(&call_sid, &speech_result, &to_number).await;

    let twiml = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<Response>
    <Say>{}</Say>
    <Gather input="speech" action="/api/v1/webhooks/twilio_voice/gather" speechTimeout="auto" />
</Response>"#, ai_response.replace("<", "").replace(">", "").replace("&", "and"));

    ([(axum::http::header::CONTENT_TYPE, "application/xml")], twiml).into_response()
}

pub async fn twilio_voice_status_handler(
    State(state): State<TwilioVoiceWebhookState>,
    body_bytes: axum::body::Bytes,
) -> impl IntoResponse {
    let body_str = String::from_utf8_lossy(&body_bytes);
    let params = parse_form_urlencoded(&body_str);

    let call_sid = params.get("CallSid").cloned().unwrap_or_default();
    let call_status = params.get("CallStatus").cloned().unwrap_or_default();
    let caller_phone = params.get("From").cloned().unwrap_or_default();
    let to_number = params.get("To").cloned().unwrap_or_default();

    if call_status == "completed" || call_status == "failed" || call_status == "busy" || call_status == "no-answer" || call_status == "canceled" {
        state.voice_engine.end_call(&call_sid).await;

        let actions = state.voice_engine.actions.lock().await;
        let session_actions: Vec<_> = actions.iter().filter(|a| a.session_id == call_sid).collect();
        let has_booking_intent = session_actions.iter().any(|a| a.intent_type == "BOOK_APPOINTMENT");
        let has_order_intent = session_actions.iter().any(|a| a.intent_type == "ORDER_FOOD");

        let deposit_link = session_actions.iter()
            .find(|a| a.intent_type == "BOOK_APPOINTMENT")
            .and_then(|a| a.details.get("deposit_link").and_then(|v| v.as_str()))
            .unwrap_or("https://pay.ohc.com/deposit/voice")
            .to_string();

        let order_link = session_actions.iter()
            .find(|a| a.intent_type == "ORDER_FOOD")
            .and_then(|a| a.details.get("order_link").and_then(|v| v.as_str()))
            .unwrap_or("https://pay.ohc.com/store/voice")
            .to_string();
        drop(actions);

        let transcripts = state.voice_engine.transcripts.lock().await;
        let session_transcripts: Vec<_> = transcripts.iter().filter(|t| t.session_id == call_sid).collect();

        if !session_transcripts.is_empty() {
            let mut summary = String::new();
            for t in session_transcripts {
                summary.push_str(&format!("{}: {}\n", t.role, t.text));
            }

            let pool = &state.db.pool;
            let tenant_id = match &state.db.store {
                crate::db::DbStore::Postgres => {
                    match sqlx::query_scalar::<_, String>(
                        "SELECT tenant_id FROM settings WHERE voice_receptionist_number = $1 LIMIT 1"
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
                        "SELECT tenant_id FROM settings WHERE voice_receptionist_number = ? LIMIT 1"
                    )
                    .bind(&to_number)
                    .fetch_optional(sqlite_pool)
                    .await {
                        Ok(Some(id)) => id,
                        _ => "test_tenant".to_string(),
                    }
                }
            };

            let resolver = IdentityResolver::new(state.db.clone());
            let clean_caller = caller_phone.replace("whatsapp:", "").replace("sip:", "");
            let customer_id_result = resolver.resolve_or_create_customer(&tenant_id, &clean_caller, "voice").await;
            let customer_id = customer_id_result.as_ref().ok().map(|s| s.as_str());

            let inbox_id = Uuid::new_v4().to_string();

            let insert_result = match &state.db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query(
                        "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at) VALUES ($1, $2, 'voice', $3, $4, 'English', '', 'unread', $5, $6, NOW())"
                    )
                    .bind(&inbox_id)
                    .bind(&tenant_id)
                    .bind(&summary)
                    .bind(&summary)
                    .bind(&clean_caller)
                    .bind(&customer_id)
                    .execute(pool)
                    .await.map(|_| ())
                },
                crate::db::DbStore::Sqlite(sqlite_pool) => {
                    sqlx::query(
                        "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at) VALUES (?, ?, 'voice', ?, ?, 'English', '', 'unread', ?, ?, CURRENT_TIMESTAMP)"
                    )
                    .bind(&inbox_id)
                    .bind(&tenant_id)
                    .bind(&summary)
                    .bind(&summary)
                    .bind(&clean_caller)
                    .bind(&customer_id)
                    .execute(sqlite_pool)
                    .await.map(|_| ())
                }
            };

            if let Err(e) = insert_result {
                tracing::error!("Failed to insert voice call transcript into omni_inbox_messages: {}", e);
            }

            if has_booking_intent {
                let task_manager = crate::tasks::TaskManager::with_db(state.db.clone());
                let mission_id = uuid::Uuid::new_v4().to_string();

                let title = format!("Voice Booking Request from {}", clean_caller);
                let priority = "P1".to_string(); // Requires approval

                if let Ok(mut task) = task_manager.create_task(tenant_id.clone(), mission_id, title, summary.clone(), priority) {
                    task.approval_status = Some("PENDING".to_string());

                    let proposed_content = serde_json::json!({
                        "feature_type": "booking_draft",
                        "summary": summary,
                        "caller_phone": clean_caller,
                        "deposit_link": deposit_link,
                    });
                    task.proposed_content = Some(proposed_content.to_string());

                    let _ = task_manager.insert_task(task);
                }
            }

            if has_order_intent {
                let task_manager = crate::tasks::TaskManager::with_db(state.db.clone());
                let mission_id = uuid::Uuid::new_v4().to_string();

                let title = format!("Voice Order Request from {}", clean_caller);
                let priority = "P1".to_string();

                if let Ok(mut task) = task_manager.create_task(tenant_id.clone(), mission_id, title, summary.clone(), priority) {
                    task.approval_status = Some("PENDING".to_string());

                    let proposed_content = serde_json::json!({
                        "feature_type": "order_draft",
                        "summary": summary,
                        "caller_phone": clean_caller,
                        "order_link": order_link,
                    });
                    task.proposed_content = Some(proposed_content.to_string());

                    let _ = task_manager.insert_task(task);
                }
            }
        }
    }

    StatusCode::OK.into_response()
}

fn parse_form_urlencoded(input: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    for pair in input.split('&') {
        let mut parts = pair.split('=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            let decoded_key = url_decode(key);
            let decoded_val = url_decode(value);
            params.insert(decoded_key, decoded_val);
        }
    }
    params
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_form_urlencoded() {
        let params = parse_form_urlencoded("CallSid=CA123&From=%2B123&To=%2B456");
        assert_eq!(params.get("CallSid").unwrap(), "CA123");
        assert_eq!(params.get("From").unwrap(), "+123");
        assert_eq!(params.get("To").unwrap(), "+456");
    }
}
