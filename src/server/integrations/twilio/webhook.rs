use axum::{
    extract::{Form, State, Request},
    http::StatusCode,
    response::IntoResponse,
    middleware::Next,
};
use std::collections::HashMap;

use uuid::Uuid;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use ::server_utils::url::url_decode;
use sqlx::PgPool;

pub async fn handle_twilio_webhook(
    State(pool): State<PgPool>,
    Form(payload): Form<HashMap<String, String>>,
) -> impl IntoResponse {
    let to = payload.get("To");
    let from = payload.get("From");
    let body = payload.get("Body");
    let message_sid = payload.get("MessageSid");

    if to.is_none() || from.is_none() || body.is_none() || message_sid.is_none() {
        return (StatusCode::BAD_REQUEST, "Missing required fields").into_response();
    }
    let to = to.unwrap();
    let from = from.unwrap();
    let body = body.unwrap();

    let tenant_id: Option<String> = sqlx::query_scalar(
        "SELECT tenant_id FROM channel_twilio_sms WHERE phone_number = $1 LIMIT 1"
    )
    .bind(&to)
    .fetch_optional(&pool)
    .await.unwrap_or(None);

    let tenant_id = match tenant_id {
        Some(t) => t,
        None => {
            tracing::warn!("Could not find tenant for twilio webhook receiver {}", to);
            return (StatusCode::NOT_FOUND, "Tenant not found").into_response();
        }
    };

    let inbox_id = Uuid::new_v4().to_string();
    let result = sqlx::query(
        "INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, draft_reply, status, sender_id, created_at) VALUES ($1, $2, $3, $4, $5, '', 'unread', $6, NOW()) ON CONFLICT DO NOTHING"
    )
    .bind(&inbox_id)
    .bind(&tenant_id)
    .bind("twilio_sms")
    .bind(body)
    .bind(body)
    .bind(from)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            let job_id = Uuid::new_v4().to_string();
            let payload_json = serde_json::json!({
                "message_id": inbox_id,
                "inbox_message_id": inbox_id,
                "source": "twilio_sms",
                "content": body,
                "sender_id": from
            });
            let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'message_triage', $3, 'PENDING')")
                .bind(&job_id)
                .bind(&tenant_id)
                .bind(payload_json.to_string())
                .execute(&pool)
                .await;
            (StatusCode::OK, "Message received").into_response()
        },
        Err(e) => {
            tracing::error!("Failed to save incoming Twilio message: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

pub async fn twilio_connector_signature_middleware(
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let auth_token = std::env::var("TWILIO_AUTH_TOKEN").unwrap_or_default();
    if auth_token.is_empty() {
        let res = next.run(req).await;
        return Ok(res);
    }

    let base_url = std::env::var("TWILIO_WEBHOOK_BASE_URL").unwrap_or_else(|_| "https://example.com".to_string());

    let twilio_signature = req.headers().get("X-Twilio-Signature").map(|v| v.to_str().unwrap_or("").to_string());
    if twilio_signature.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let (parts, body) = req.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX).await.map_err(|_| StatusCode::BAD_REQUEST)?;

    let uri = parts.uri.path();
    let query = parts.uri.query().map(|q| format!("?{}", q)).unwrap_or_default();
    let full_url = format!("{}{}{}", base_url, uri, query);

    let body_str = String::from_utf8_lossy(&bytes);

    let mut params = HashMap::new();
    for pair in body_str.split('&') {
        let mut parts = pair.split('=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            params.insert(url_decode(key), url_decode(value));
        }
    }

    let mut keys: Vec<&String> = params.keys().collect();
    keys.sort();

    let mut data = full_url;
    for k in keys {
        data.push_str(k);
        data.push_str(params.get(k).unwrap());
    }

    let mut mac = Hmac::<Sha1>::new_from_slice(auth_token.as_bytes()).unwrap();
    mac.update(data.as_bytes());
    let result = mac.finalize().into_bytes();
    let expected_sig = STANDARD.encode(result);

    if twilio_signature.unwrap() == expected_sig {
        let new_req = Request::from_parts(parts, axum::body::Body::from(bytes));
        let res = next.run(new_req).await;
        Ok(res)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
