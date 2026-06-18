use axum::{
    extract::{State, Form},
    response::{IntoResponse},
    http::{StatusCode, header},
};
use std::sync::Arc;
use std::collections::HashMap;

use crate::db::DB;
use crate::voice::VoiceContextRouter;
use crate::voice::VoiceAIEdgeEngine;
use ::server_integrations_twilio::provider::TwilioProvider;
use uuid::Uuid;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::identity_resolution::IdentityResolver;

#[derive(Clone)]
pub struct TwilioVoiceWebhookState {
    pub db: Arc<DB>,
    pub voice_engine: Arc<VoiceAIEdgeEngine>,
    pub twilio_provider: Arc<TwilioProvider>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

pub async fn twilio_voice_webhook_post_handler(
    State(state): State<TwilioVoiceWebhookState>,
    Form(params): Form<HashMap<String, String>>,
) -> impl IntoResponse {
    let call_sid = params.get("CallSid").cloned().unwrap_or_else(|| Uuid::new_v4().to_string());
    let from_number = params.get("From").cloned().unwrap_or_else(|| "unknown".to_string());
    let to_number = params.get("To").cloned().unwrap_or_else(|| "unknown".to_string());
    let speech_result = params.get("SpeechResult").cloned().unwrap_or_else(|| "".to_string());
    let call_status = params.get("CallStatus").cloned().unwrap_or_else(|| "".to_string());

    tracing::info!("Received Twilio Voice Webhook: CallSid: {}, From: {}, Status: {}, Speech: {}", call_sid, from_number, call_status, speech_result);

    let pool = &state.db.pool;

    // Find the correct tenant
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

    let router = VoiceContextRouter::new(state.voice_engine.clone(), state.twilio_provider.clone());

    if call_status == "completed" || call_status == "failed" || call_status == "busy" || call_status == "no-answer" || call_status == "canceled" {
        state.voice_engine.end_call(&call_sid).await;

        // At the end of the call, we save the transcript summary to the omni inbox.
        // We will fetch transcriptions associated to the call and build a summary
        let mut full_transcript = String::new();
        {
            let transcripts = state.voice_engine.transcripts.lock().await;
            for t in transcripts.iter().filter(|t| t.session_id == call_sid) {
                 full_transcript.push_str(&format!("{}: {}\n", t.role, t.text));
            }
        }

        if !full_transcript.is_empty() {
            let resolver = IdentityResolver::new(state.db.clone());
            let customer_id_result = resolver.resolve_or_create_customer(&tenant_id, &from_number, "voice").await;
            let customer_id = customer_id_result.as_ref().ok().map(|s| s.as_str());

            let inbox_id = Uuid::new_v4().to_string();
            let summary_text = format!("Voice Call Summary (CallSid: {}):\n{}", call_sid, full_transcript);

            let insert_result = match &state.db.store {
                crate::db::DbStore::Postgres => {
                    sqlx::query(
                        "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at) VALUES ($1, $2, 'voice', $3, $4, 'English', '', 'unread', $5, $6, NOW())"
                    )
                    .bind(&inbox_id)
                    .bind(&tenant_id)
                    .bind(&summary_text)
                    .bind(&summary_text)
                    .bind(&from_number)
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
                    .bind(&summary_text)
                    .bind(&summary_text)
                    .bind(&from_number)
                    .bind(&customer_id)
                    .execute(sqlite_pool)
                    .await.map(|_| ())
                }
            };

            if let Err(e) = insert_result {
                tracing::error!("Failed to insert omni_inbox_messages for voice summary: {}", e);
            }

            // Enqueue to ohc_job_queue
            let job_id = Uuid::new_v4().to_string();
            let mut payload_json = serde_json::json!({
                "message_id": inbox_id,
                "inbox_message_id": inbox_id,
                "source": "voice",
                "content": summary_text,
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
                tracing::error!("Failed to enqueue message_triage job for voice summary: {}", e);
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

        return (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/xml")],
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Response></Response>",
        ).into_response();
    }

    let reply = if speech_result.is_empty() {
        // Initial greeting
        state.voice_engine.handle_incoming_call(&tenant_id, &from_number).await; // In real app, we'd use CallSid as session_id or map it. For now engine generates one, but we should use CallSid.

        let greeting = match &state.db.store {
            crate::db::DbStore::Postgres => {
                match sqlx::query_scalar::<_, String>(
                    "SELECT voice_receptionist_persona FROM settings WHERE tenant_id = $1 LIMIT 1"
                )
                .bind(&tenant_id)
                .fetch_optional(pool)
                .await {
                    Ok(Some(persona)) => format!("Hello! This is the {} AI receptionist. How can I help you today?", persona),
                    _ => "Hello! This is the AI receptionist. How can I help you today?".to_string(),
                }
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                match sqlx::query_scalar::<_, String>(
                    "SELECT voice_receptionist_persona FROM settings WHERE tenant_id = ? LIMIT 1"
                )
                .bind(&tenant_id)
                .fetch_optional(sqlite_pool)
                .await {
                    Ok(Some(persona)) => format!("Hello! This is the {} AI receptionist. How can I help you today?", persona),
                    _ => "Hello! This is the AI receptionist. How can I help you today?".to_string(),
                }
            }
        };

        greeting
    } else {
        router.process_user_input(&call_sid, &speech_result, &to_number).await
    };

    let twiml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
        <Response>
            <Say>{}</Say>
            <Gather input=\"speech\" action=\"/api/v1/webhooks/twilio/voice\" method=\"POST\" timeout=\"3\" speechTimeout=\"auto\"/>
        </Response>",
        reply
    );

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/xml")],
        twiml,
    ).into_response()
}
