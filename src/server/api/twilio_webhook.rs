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
        let conversation_id = Uuid::new_v4().to_string();
        let inbox_id = Uuid::new_v4().to_string();
        let draft_id = Uuid::new_v4().to_string();
        let source = "whatsapp".to_string();

        let pool = &state.db.pool;
        let insert_result = match &state.db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = pool.begin().await.unwrap();
                let _ = sqlx::query("INSERT INTO conversations (id, tenant_id, status) VALUES ($1, $2, 'pending')")
                    .bind(&conversation_id)
                    .bind(&tenant_id)
                    .execute(&mut *tx)
                    .await;

                let _ = sqlx::query("INSERT INTO messages (id, tenant_id, conversation_id, channel, direction, content) VALUES ($1, $2, $3, $4, 'inbound', $5)")
                    .bind(&inbox_id)
                    .bind(&tenant_id)
                    .bind(&conversation_id)
                    .bind(&source)
                    .bind(&text)
                    .execute(&mut *tx)
                    .await;

                let res = sqlx::query("INSERT INTO draft_replies (id, tenant_id, message_id, content, status) VALUES ($1, $2, $3, '', 'pending')")
                    .bind(&draft_id)
                    .bind(&tenant_id)
                    .bind(&inbox_id)
                    .execute(&mut *tx)
                    .await.map(|_| ());

                let _ = tx.commit().await;
                res
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let mut tx = sqlite_pool.begin().await.unwrap();
                let _ = sqlx::query("INSERT INTO conversations (id, tenant_id, status) VALUES (?, ?, 'pending')")
                    .bind(&conversation_id)
                    .bind(&tenant_id)
                    .execute(&mut *tx)
                    .await;

                let _ = sqlx::query("INSERT INTO messages (id, tenant_id, conversation_id, channel, direction, content) VALUES (?, ?, ?, ?, 'inbound', ?)")
                    .bind(&inbox_id)
                    .bind(&tenant_id)
                    .bind(&conversation_id)
                    .bind(&source)
                    .bind(&text)
                    .execute(&mut *tx)
                    .await;

                let res = sqlx::query("INSERT INTO draft_replies (id, tenant_id, message_id, content, status) VALUES (?, ?, ?, '', 'pending')")
                    .bind(&draft_id)
                    .bind(&tenant_id)
                    .bind(&inbox_id)
                    .execute(&mut *tx)
                    .await.map(|_| ());

                let _ = tx.commit().await;
                res
            }
        };

        if let Err(e) = insert_result {
            tracing::error!("Failed to insert conversation/message: {}", e);
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
