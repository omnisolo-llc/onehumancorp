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

#[derive(Clone)]
pub struct TwilioWebhookState {
    pub hub: Arc<Hub>,
    pub db: Arc<DB>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
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

        let insert_result = match &state.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query(
                    "INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status) VALUES ($1, $2, $3, $4, '', 'pending')"
                )
                .bind(&inbox_id)
                .bind(&tenant_id)
                .bind(&source)
                .bind(&text)
                .execute(pool)
                .await.map(|_| ())
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    "INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status) VALUES (?, ?, ?, ?, '', 'pending')"
                )
                .bind(&inbox_id)
                .bind(&tenant_id)
                .bind(&source)
                .bind(&text)
                .execute(sqlite_pool)
                .await.map(|_| ())
            }
        };

        if let Err(e) = insert_result {
            tracing::error!("Failed to insert inbox message: {}", e);
        }

        let event = crate::orchestration::departments::types::DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.message.received".to_string(),
            payload: serde_json::json!({
                "source": source,
                "message": text,
                "sender_id": sender_id.replace("whatsapp:", ""),
                "inbox_message_id": inbox_id,
            }),
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
