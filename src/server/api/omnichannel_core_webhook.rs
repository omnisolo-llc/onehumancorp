use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::DB;

#[derive(Debug, Deserialize)]
pub struct WebhookPayload {
    pub tenant_id: String,
    pub provider_type: String, // e.g., "whatsapp", "web_widget"
    pub sender_id: String,     // e.g., phone number or unique visitor id
    pub message_content: String,
}

#[derive(Debug, Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub message_id: Option<String>,
}

pub async fn handle_webhook(
    State(db): State<Arc<DB>>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    let tenant_id = payload.tenant_id.clone();

    let inbox_id = Uuid::new_v4().to_string();
    let _channel_id = Uuid::new_v4().to_string();

    let inbox_res = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO inboxes (id, tenant_id, name) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
                .bind(&inbox_id)
                .bind(&tenant_id)
                .bind(format!("{} Inbox", payload.provider_type))
                .execute(&db.pool)
                .await
                .map(|_| ())
        },
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("INSERT INTO inboxes (id, tenant_id, name) VALUES (?, ?, ?) ON CONFLICT DO NOTHING")
                .bind(&inbox_id)
                .bind(&tenant_id)
                .bind(format!("{} Inbox", payload.provider_type))
                .execute(pool)
                .await
                .map(|_| ())
        }
    };

    if let Err(e) = inbox_res {
        tracing::error!("Failed to upsert inbox: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
    }

    let contact_id = Uuid::new_v4().to_string();
    let contact_res = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO contacts (id, tenant_id, phone_number) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
                .bind(&contact_id)
                .bind(&tenant_id)
                .bind(&payload.sender_id)
                .execute(&db.pool)
                .await
                .map(|_| ())
        },
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("INSERT INTO contacts (id, tenant_id, phone_number) VALUES (?, ?, ?) ON CONFLICT DO NOTHING")
                .bind(&contact_id)
                .bind(&tenant_id)
                .bind(&payload.sender_id)
                .execute(pool)
                .await
                .map(|_| ())
        }
    };

    if let Err(e) = contact_res {
        tracing::error!("Failed to upsert contact: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
    }

    let conversation_id = Uuid::new_v4().to_string();
    let conv_res = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, 'open') ON CONFLICT DO NOTHING")
                .bind(&conversation_id)
                .bind(&tenant_id)
                .bind(&inbox_id)
                .bind(&contact_id)
                .execute(&db.pool)
                .await
                .map(|_| ())
        },
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status) VALUES (?, ?, ?, ?, 'open') ON CONFLICT DO NOTHING")
                .bind(&conversation_id)
                .bind(&tenant_id)
                .bind(&inbox_id)
                .bind(&contact_id)
                .execute(pool)
                .await
                .map(|_| ())
        }
    };

    if let Err(e) = conv_res {
        tracing::error!("Failed to upsert conversation: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
    }

    let message_id = Uuid::new_v4().to_string();
    let msg_res = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO messages (id, tenant_id, conversation_id, content, sender_type) VALUES ($1, $2, $3, $4, 'contact')")
                .bind(&message_id)
                .bind(&tenant_id)
                .bind(&conversation_id)
                .bind(&payload.message_content)
                .execute(&db.pool)
                .await
                .map(|_| ())
        },
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("INSERT INTO messages (id, tenant_id, conversation_id, content, sender_type) VALUES (?, ?, ?, ?, 'contact')")
                .bind(&message_id)
                .bind(&tenant_id)
                .bind(&conversation_id)
                .bind(&payload.message_content)
                .execute(pool)
                .await
                .map(|_| ())
        }
    };

    if let Err(e) = msg_res {
        tracing::error!("Failed to insert message: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
    }

    let payload_json = serde_json::json!({
        "message_id": message_id,
        "conversation_id": conversation_id,
        "content": payload.message_content,
        "sender_id": payload.sender_id
    });

    let job_id = Uuid::new_v4().to_string();
    let _ = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'omnichannel_message_received', $3, 'PENDING')")
                .bind(&job_id)
                .bind(&tenant_id)
                .bind(payload_json.to_string())
                .execute(&db.pool)
                .await
                .map(|_| ())
        },
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'omnichannel_message_received', ?, 'PENDING')")
                .bind(&job_id)
                .bind(&tenant_id)
                .bind(payload_json.to_string())
                .execute(pool)
                .await
                .map(|_| ())
        }
    };

    (StatusCode::OK, Json(WebhookResponse { success: true, message_id: Some(message_id) })).into_response()
}
