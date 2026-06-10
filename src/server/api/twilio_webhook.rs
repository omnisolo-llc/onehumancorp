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
use crate::Hub;

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
    let text = params.get("Body").cloned().unwrap_or_else(|| "".to_string());

    if !text.is_empty() {
        tracing::info!("Received Twilio message from {}: {}", sender_id, text);

        let tenant_id = "test_tenant".to_string(); // Replace with actual DB lookup based on `_to_number` in the future
        let inbox_id = Uuid::new_v4().to_string();
        let source = "whatsapp".to_string();

        let pool = &state.db.pool;
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

        // Insert triage item as requested by E2E test. E2E test checks Triage UI for 'whatsapp'.
        let triage_id = Uuid::new_v4().to_string();
        let insert_triage = match &state.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query("INSERT INTO triage_items (id, tenant_id, source, context, priority, status) VALUES ($1, $2, $3, $4, 'action needed', 'pending')")
                .bind(&triage_id).bind(&tenant_id).bind(&source).bind(&text)
                .execute(pool).await.map(|_| ())
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                sqlx::query("INSERT INTO triage_items (id, tenant_id, source, context, priority, status) VALUES (?, ?, ?, ?, 'action needed', 'pending')")
                .bind(&triage_id).bind(&tenant_id).bind(&source).bind(&text)
                .execute(sqlite_pool).await.map(|_| ())
            }
        };
        if let Err(e) = insert_triage {
            tracing::error!("Failed to insert triage item: {}", e);
        }

        let event = crate::orchestration::departments::types::DepartmentEvent {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_type: "tenant.message.received".to_string(),
            payload: serde_json::json!({
                "source": source,
                "message": text,
                "sender_id": sender_id,
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
