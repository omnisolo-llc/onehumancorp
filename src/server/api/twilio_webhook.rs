use axum::{
    body::Body,
    extract::Request,
    extract::State,
    middleware::Next,
    response::IntoResponse,
    http::StatusCode,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::sync::Arc;
use uuid::Uuid;
use std::collections::HashMap;

use crate::db::DB;
use ::server_utils::url::url_decode;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::hub::Hub;
use crate::orchestration::identity_resolution::IdentityResolver;

#[derive(Clone)]
pub struct TwilioWebhookState {
    pub hub: Arc<Hub>,
    pub db: Arc<DB>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
    pub voice_engine: Arc<crate::voice::VoiceAIEdgeEngine>,
    pub voice_router: Arc<crate::voice::VoiceContextRouter>,
    pub voice_sessions: Arc<dashmap::DashMap<String, String>>,
}

pub(crate) fn valid_twilio_signature(
    auth_token: &str,
    canonical_url: &str,
    form_body: &[u8],
    signature: Option<&str>,
) -> bool {
    if auth_token.trim().is_empty() {
        return false;
    }
    let Some(signature) = signature else {
        return false;
    };
    let Ok(signature) = STANDARD.decode(signature) else {
        return false;
    };

    let mut fields = url::form_urlencoded::parse(form_body).collect::<Vec<_>>();
    fields.sort_by(|left, right| left.0.cmp(&right.0));
    let mut signed = canonical_url.as_bytes().to_vec();
    for (name, value) in fields {
        signed.extend_from_slice(name.as_bytes());
        signed.extend_from_slice(value.as_bytes());
    }

    let Ok(mut mac) = Hmac::<Sha1>::new_from_slice(auth_token.as_bytes()) else {
        return false;
    };
    mac.update(&signed);
    mac.verify_slice(&signature).is_ok()
}

fn canonical_twilio_url(base_url: &str, request_uri: &axum::http::Uri) -> Option<String> {
    let mut url = reqwest::Url::parse(base_url).ok()?;
    if !matches!(url.scheme(), "http" | "https")
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return None;
    }
    url.set_path(request_uri.path());
    url.set_query(request_uri.query());
    Some(url.into())
}

pub async fn twilio_signature_middleware(
    request: Request,
    next: Next,
) -> axum::response::Response {
    const TWILIO_BODY_LIMIT_BYTES: usize = 262_144;

    let auth_token = std::env::var("TWILIO_AUTH_TOKEN").ok();
    let public_base_url = std::env::var("TWILIO_WEBHOOK_BASE_URL").ok();
    let (parts, body) = request.into_parts();
    let signature = parts
        .headers
        .get("x-twilio-signature")
        .and_then(|value| value.to_str().ok());
    let Ok(body) = axum::body::to_bytes(body, TWILIO_BODY_LIMIT_BYTES).await else {
        return StatusCode::PAYLOAD_TOO_LARGE.into_response();
    };
    let valid = auth_token
        .as_deref()
        .zip(public_base_url.as_deref())
        .and_then(|(token, base_url)| {
            canonical_twilio_url(base_url, &parts.uri)
                .map(|url| valid_twilio_signature(token, &url, &body, signature))
        })
        .unwrap_or(false);
    if !valid {
        tracing::warn!("Twilio webhook signature verification failed");
        return StatusCode::UNAUTHORIZED.into_response();
    }

    next.run(Request::from_parts(parts, Body::from(body))).await
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
    let to_number = params.get("To").cloned().unwrap_or_else(|| "unknown".to_string());
    let mut text = params.get("Body").cloned().unwrap_or_else(|| "".to_string());
    let mut has_audio = false;
    let mut audio_url = "".to_string();

    let num_media: usize = params.get("NumMedia").and_then(|s| s.parse().ok()).unwrap_or(0);
    for i in 0..num_media {
        if let Some(media_url) = params.get(&format!("MediaUrl{}", i)) {
            let media_type = params.get(&format!("MediaContentType{}", i)).cloned().unwrap_or_else(|| "unknown".to_string());
            if media_type.starts_with("audio/") || media_type == "application/ogg" {
                has_audio = true;
                audio_url = media_url.clone();
            }
            text.push_str(&format!(" [Media: {} - {}]", media_type, media_url));
        }
    }

    if !text.is_empty() || num_media > 0 {
        if has_audio {
            tracing::info!("Received Twilio WhatsApp Voice Note from {}: {}", sender_id, audio_url);
            text = "Voice order transcribed: 2x Chicken Plates for 1pm. (Mocked transcription via Whisper API)".to_string();
        }
        tracing::info!("Received Twilio message from {}: {}", sender_id, text);

        let pool = &state.db.pool;

        // Find the correct tenant by mapping the `To` number
        let clean_to_number = to_number.replace("whatsapp:", "");
        let tenant_id = match &state.db.store {
            crate::db::DbStore::Postgres => {
                let mut tid = sqlx::query_scalar::<_, String>(
                    "SELECT tenant_id FROM integration_credentials WHERE (from_phone = $1 OR from_phone = $2) AND integration_id IN ('twilio', 'whatsapp', 'whatsapp_cloud_api') LIMIT 1"
                )
                .bind(&to_number)
                .bind(&clean_to_number)
                .fetch_optional(pool)
                .await.unwrap_or(None);

                if tid.is_none() {
                    tid = sqlx::query_scalar::<_, String>(
                        "SELECT tenant_id FROM settings WHERE sms_critical_phone = $1 OR voice_receptionist_number = $1 OR sms_critical_phone = $2 OR voice_receptionist_number = $2 LIMIT 1"
                    )
                    .bind(&to_number)
                    .bind(&clean_to_number)
                    .fetch_optional(pool)
                    .await.unwrap_or(None);
                }

                match tid {
                    Some(id) => id,
                    None => return StatusCode::NOT_FOUND.into_response(),
                }
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let mut tid = sqlx::query_scalar::<_, String>(
                    "SELECT tenant_id FROM integration_credentials WHERE (from_phone = ? OR from_phone = ?) AND integration_id IN ('twilio', 'whatsapp', 'whatsapp_cloud_api') LIMIT 1"
                )
                .bind(&to_number)
                .bind(&clean_to_number)
                .fetch_optional(sqlite_pool)
                .await.unwrap_or(None);

                if tid.is_none() {
                    tid = sqlx::query_scalar::<_, String>(
                        "SELECT tenant_id FROM settings WHERE sms_critical_phone = ? OR voice_receptionist_number = ? OR sms_critical_phone = ? OR voice_receptionist_number = ? LIMIT 1"
                    )
                    .bind(&to_number)
                    .bind(&to_number)
                    .bind(&clean_to_number)
                    .bind(&clean_to_number)
                    .fetch_optional(sqlite_pool)
                    .await.unwrap_or(None);
                }

                match tid {
                    Some(id) => id,
                    None => return StatusCode::NOT_FOUND.into_response(),
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
                    "INSERT INTO chat_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at) VALUES ($1, $2, $3, $4, $5, 'English', '', 'unread', $6, $7, NOW())"
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
                    "INSERT INTO chat_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at) VALUES (?, ?, ?, ?, ?, 'English', '', 'unread', ?, ?, CURRENT_TIMESTAMP)"
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
            tracing::error!("Failed to insert chat_messages: {}", e);
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

pub async fn twilio_voice_webhook_handler(
    State(state): State<TwilioWebhookState>,
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

    let call_sid = params.get("CallSid").cloned().unwrap_or_else(|| "unknown".to_string());
    let sender_id = params.get("From").cloned().unwrap_or_else(|| "unknown".to_string());
    let to_number = params.get("To").cloned().unwrap_or_else(|| "unknown".to_string());
    let speech_result = params.get("SpeechResult").cloned();

    let pool = &state.db.pool;

    let clean_to_number = to_number.replace("whatsapp:", "");
    let tenant_id = match &state.db.store {
        crate::db::DbStore::Postgres => {
            let mut tid = sqlx::query_scalar::<_, String>(
                "SELECT tenant_id FROM integration_credentials WHERE (from_phone = $1 OR from_phone = $2) AND integration_id IN ('twilio', 'whatsapp', 'whatsapp_cloud_api') LIMIT 1"
            )
            .bind(&to_number)
            .bind(&clean_to_number)
            .fetch_optional(pool)
            .await.unwrap_or(None);

            if tid.is_none() {
                tid = sqlx::query_scalar::<_, String>(
                    "SELECT tenant_id FROM settings WHERE sms_critical_phone = $1 OR voice_receptionist_number = $1 OR sms_critical_phone = $2 OR voice_receptionist_number = $2 LIMIT 1"
                )
                .bind(&to_number)
                .bind(&clean_to_number)
                .fetch_optional(pool)
                .await.unwrap_or(None);
            }

            match tid {
                Some(id) => id,
                None => return StatusCode::NOT_FOUND.into_response(),
            }
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let mut tid = sqlx::query_scalar::<_, String>(
                "SELECT tenant_id FROM integration_credentials WHERE (from_phone = ? OR from_phone = ?) AND integration_id IN ('twilio', 'whatsapp', 'whatsapp_cloud_api') LIMIT 1"
            )
            .bind(&to_number)
            .bind(&clean_to_number)
            .fetch_optional(sqlite_pool)
            .await.unwrap_or(None);

            if tid.is_none() {
                tid = sqlx::query_scalar::<_, String>(
                    "SELECT tenant_id FROM settings WHERE sms_critical_phone = ? OR voice_receptionist_number = ? OR sms_critical_phone = ? OR voice_receptionist_number = ? LIMIT 1"
                )
                .bind(&to_number)
                .bind(&to_number)
                .bind(&clean_to_number)
                .bind(&clean_to_number)
                .fetch_optional(sqlite_pool)
                .await.unwrap_or(None);
            }

            match tid {
                Some(id) => id,
                None => return StatusCode::NOT_FOUND.into_response(),
            }
        }
    };

    let ai_response = if let Some(user_text) = speech_result {
        // Continuing call
        let session_id = state.voice_sessions.get(&call_sid).map(|r| r.value().clone()).unwrap_or_else(|| call_sid.clone());

        // Log the user's speech
        let inbox_id = Uuid::new_v4().to_string();
        let clean_sender_id = sender_id.replace("whatsapp:", "");

        let resolver = IdentityResolver::new(state.db.clone());
        let customer_id_result = resolver.resolve_or_create_customer(&tenant_id, &clean_sender_id, "voice").await;
        let customer_id = customer_id_result.as_ref().ok().map(|s| s.as_str());

        let insert_result = match &state.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query(
                    "INSERT INTO chat_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at) VALUES ($1, $2, 'voice', $3, $4, 'English', '', 'unread', $5, $6, NOW())"
                )
                .bind(&inbox_id)
                .bind(&tenant_id)
                .bind(&user_text)
                .bind(&user_text)
                .bind(&clean_sender_id)
                .bind(&customer_id)
                .execute(pool)
                .await.map(|_| ())
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    "INSERT INTO chat_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at) VALUES (?, ?, 'voice', ?, ?, 'English', '', 'unread', ?, ?, CURRENT_TIMESTAMP)"
                )
                .bind(&inbox_id)
                .bind(&tenant_id)
                .bind(&user_text)
                .bind(&user_text)
                .bind(&clean_sender_id)
                .bind(&customer_id)
                .execute(sqlite_pool)
                .await.map(|_| ())
            }
        };

        if let Err(e) = insert_result {
            tracing::error!("Failed to insert chat_messages for voice: {}", e);
        }

        // Enqueue job
        let job_id = Uuid::new_v4().to_string();
        let mut payload_json = serde_json::json!({
            "message_id": inbox_id,
            "inbox_message_id": inbox_id,
            "source": "voice",
            "content": user_text,
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
            tracing::error!("Failed to enqueue voice message_triage job: {}", e);
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

        state.voice_router.process_user_input(&session_id, &user_text, &to_number).await
    } else {
        // New call
        let session_id = state.voice_engine.handle_incoming_call(&tenant_id, &sender_id).await;
        state.voice_sessions.insert(call_sid.clone(), session_id);
        "Hello! Thank you for calling. How can I help you today?".to_string()
    };

    let twiml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><Response><Say>{}</Say><Gather input="speech" action="/api/v1/webhooks/twilio/voice" speechTimeout="auto"></Gather></Response>"#,
        ai_response
    );

    ([(axum::http::header::CONTENT_TYPE, "text/xml")], twiml).into_response()
}
