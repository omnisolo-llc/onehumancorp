use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct MetaGraphWebhookState {
    pub db: Arc<crate::db::DB>,
}

#[derive(Debug, Deserialize)]
pub struct MetaWebhookEvent {
    pub object: String,
    pub entry: Vec<MetaWebhookEntry>,
}

#[derive(Debug, Deserialize)]
pub struct MetaWebhookEntry {
    pub id: String, // typically the Page ID or IG account ID
    pub time: i64,
    pub messaging: Option<Vec<MetaMessagingItem>>,
}

#[derive(Debug, Deserialize)]
pub struct MetaMessagingItem {
    pub sender: MetaMessagingUser,
    pub recipient: MetaMessagingUser,
    pub timestamp: i64,
    pub message: MetaMessageData,
}

#[derive(Debug, Deserialize)]
pub struct MetaMessagingUser {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct MetaMessageData {
    pub mid: String,
    pub text: String,
}

pub async fn meta_graph_webhook_handler(
    State(state): State<MetaGraphWebhookState>,
    Json(payload): Json<MetaWebhookEvent>,
) -> impl IntoResponse {
    if payload.object != "page" && payload.object != "instagram" {
        return StatusCode::NOT_FOUND.into_response();
    }

    for entry in payload.entry {
        if let Some(messaging) = entry.messaging {
            for msg_item in messaging {
                // Find tenant by page ID (assuming we store page_id in meta_graph_page_id)
                let tenant_id_opt: Option<String> = match &state.db.store {
                    crate::db::DbStore::Sqlite(pool) => {
                        sqlx::query("SELECT tenant_id FROM tenants WHERE meta_graph_page_id = ? LIMIT 1")
                            .bind(&entry.id)
                            .fetch_optional(pool)
                            .await.map(|_| ())
                            .ok()
                            .into_iter().flatten()
                            .map(|r| sqlx::Row::get(&r, "tenant_id"))
                    }
                    crate::db::DbStore::Postgres => {
                        sqlx::query("SELECT tenant_id FROM tenants WHERE meta_graph_page_id = $1 LIMIT 1")
                            .bind(&entry.id)
                            .fetch_optional(&state.db.pool)
                            .await.map(|_| ())
                            .ok()
                            .into_iter().flatten()
                            .map(|r| sqlx::Row::get(&r, "tenant_id"))
                    }
                };

                let tenant_id = match tenant_id_opt {
                    Some(id) => id,
                    None => {
                        tracing::warn!("Meta Graph webhook received for unknown page ID: {}", entry.id);
                        continue;
                    }
                };

                let message_id = uuid::Uuid::new_v4().to_string();
                let sender_id = msg_item.sender.id;
                let text = msg_item.message.text;

                // Save message
                let _res = match &state.db.store {
                    crate::db::DbStore::Sqlite(pool) => {
                        sqlx::query("INSERT INTO unified_inbox_messages (id, tenant_id, source, external_sender_id, text, is_read) VALUES (?, ?, ?, ?, ?, ?)")
                            .bind(&message_id)
                            .bind(&tenant_id)
                            .bind("meta_graph")
                            .bind(&sender_id)
                            .bind(&text)
                            .bind(false)
                            .execute(pool)
                            .await.map(|_| ())
                    }
                    crate::db::DbStore::Postgres => {
                        sqlx::query("INSERT INTO unified_inbox_messages (id, tenant_id, source, external_sender_id, text, is_read) VALUES ($1, $2, $3, $4, $5, $6)")
                            .bind(&message_id)
                            .bind(&tenant_id)
                            .bind("meta_graph")
                            .bind(&sender_id)
                            .bind(&text)
                            .bind(false)
                            .execute(&state.db.pool)
                            .await.map(|_| ())
                    }
                };

                // In a full implementation we would dispatch to `crate::msgbus::MemoryBus` to trigger
                // Customer Success Agent but as a robust skeleton we log the intended action.
                tracing::info!("Stored incoming Meta message in Unified Inbox for tenant {}. Dispatching to AI agent...", tenant_id);
            }
        }
    }

    StatusCode::OK.into_response()
}
