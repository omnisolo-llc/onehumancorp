use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::identity_resolution::IdentityResolver;
use crate::Hub;

#[derive(Clone)]
pub struct OmniInboxWebhookState {
    pub hub: Arc<Hub>,
    pub db: Arc<crate::db::DB>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

#[derive(Deserialize)]
pub struct OmniInboxPayload {
    pub tenant_id: String,
    pub source: String,
    pub sender_id: String,
    pub message: String,
    pub message_id: Option<String>,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub success: bool,
}

pub async fn omni_inbox_post_handler(
    State(state): State<OmniInboxWebhookState>,
    Json(payload): Json<OmniInboxPayload>,
) -> impl IntoResponse {
    if payload.message.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(WebhookResponse { success: false })).into_response();
    }

    let tenant_id_str = payload.tenant_id.clone();
    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(tid) => tid,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(WebhookResponse { success: false })).into_response(),
    };

    let source = payload.source.to_lowercase();
    let sender_id = payload.sender_id;
    let message = payload.message;
    let provider_message_id = payload.message_id.clone().unwrap_or_default();

    // 1. Identity Resolution
    let resolver = IdentityResolver::new(state.db.clone());
    let customer_id_result = resolver.resolve_or_create_customer(&tenant_id_str, &sender_id, &source).await;

    if let Err(e) = customer_id_result {
         tracing::error!("Failed to resolve identity: {}", e);
         return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
    }
    let customer_id = customer_id_result.as_ref().ok().map(|s| s.as_str());




 Chat

    let mut stable_msg_id = String::new();

    // We cannot use sqlx::query! because we need dynamic matching for SQLite/Postgres. We must use sqlx::query
    match &state.db.store {
        crate::db::DbStore::Postgres => {
            let mut tx_res = state.db.pool.begin().await;
            if let Ok(mut tx) = tx_res {
                let inbox_res = sqlx::query("SELECT id FROM chat_inboxes WHERE tenant_id = $1 LIMIT 1").bind(&tenant_id).fetch_optional(&mut *tx).await;
                if let Ok(Some(inbox_row)) = inbox_res {
                    use sqlx::Row;
                    let inbox_id: Uuid = inbox_row.get("id");

                    let new_contact_id = Uuid::new_v4();
                    let _ = sqlx::query("INSERT INTO chat_contacts (id, tenant_id, phone) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
                        .bind(&new_contact_id).bind(&tenant_id).bind(&sender_id).execute(&mut *tx).await;

                    let contact_res = sqlx::query("SELECT id FROM chat_contacts WHERE tenant_id = $1 AND phone = $2 LIMIT 1")
                        .bind(&tenant_id).bind(&sender_id).fetch_optional(&mut *tx).await;

                    if let Ok(Some(contact_row)) = contact_res {
                        let contact_id: Uuid = contact_row.get("id");

                        let new_conv_id = Uuid::new_v4();
                        let _ = sqlx::query("INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING")
                            .bind(&new_conv_id).bind(&tenant_id).bind(&inbox_id).bind(&contact_id).execute(&mut *tx).await;

                        let conv_res = sqlx::query("SELECT id FROM chat_conversations WHERE tenant_id = $1 AND inbox_id = $2 AND contact_id = $3 LIMIT 1")
                            .bind(&tenant_id).bind(&inbox_id).bind(&contact_id).fetch_optional(&mut *tx).await;

                        if let Ok(Some(conv_row)) = conv_res {
                            let conversation_id: Uuid = conv_row.get("id");

                            let msg_id = if provider_message_id.is_empty() {
                                let hash_input = format!("{}:{}:{}", tenant_id, conversation_id, message);
                                Uuid::new_v5(&Uuid::NAMESPACE_OID, hash_input.as_bytes())
                            } else {
                                Uuid::new_v5(&Uuid::NAMESPACE_OID, provider_message_id.as_bytes())
                            };
                            stable_msg_id = msg_id.to_string();

                            let msg_insert_res = sqlx::query("INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content) VALUES ($1, $2, $3, 'contact', $4, $5) ON CONFLICT (id) DO NOTHING")
                                .bind(&msg_id).bind(&tenant_id).bind(&conversation_id).bind(&contact_id).bind(&message).execute(&mut *tx).await;

                            if let Ok(res) = msg_insert_res {
                                if res.rows_affected() > 0 {
                                    let topic = format!("unified:chat:{}", tenant_id);
                                    let ws_payload = serde_json::json!({
                                        "action": "new_message",
                                        "message_id": msg_id.to_string(),
                                        "content": message
                                    });

                                    let mut published = false;
                                    if let Some(client) = crate::redis_pool::get_redis_client() {
                                        if let Ok(mut rconn) = client.get_connection() {
                                            let publish_res: Result<(), redis::RedisError> = redis::cmd("PUBLISH")
                                                .arg(&topic)
                                                .arg(ws_payload.to_string())
                                                .query(&mut rconn);
                                            if publish_res.is_ok() {
                                                published = true;
                                            }
                                        }
                                    }

                                    if !published {
                                        let outbox_job_id = Uuid::new_v4().to_string();
                                        let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'publish_chat_event', $3, 'PENDING')")
                                            .bind(&outbox_job_id).bind(&tenant_id_str).bind(ws_payload.to_string()).execute(&mut *tx).await;
                                    }
                                }

                                // Explicitly commit the transaction since we've inserted everything atomically
                                let _ = tx.commit().await;
                            } else {
                                let _ = tx.rollback().await;
                                return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
                            }
                        } else {
                            let _ = tx.rollback().await;
                        }
                    } else {
                        let _ = tx.rollback().await;
                    }
                } else {
                    let _ = tx.rollback().await;
                }
            } else {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
            }
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            // Simplified implementation for SQLite
            let mut tx_res = sqlite_pool.begin().await;
            if let Ok(mut tx) = tx_res {
                let inbox_res = sqlx::query("SELECT id FROM chat_inboxes WHERE tenant_id = ? LIMIT 1").bind(&tenant_id).fetch_optional(&mut *tx).await;
                if let Ok(Some(inbox_row)) = inbox_res {
                    use sqlx::Row;
                    let inbox_id: Uuid = inbox_row.get("id");

                    let new_contact_id = Uuid::new_v4();
                    let _ = sqlx::query("INSERT INTO chat_contacts (id, tenant_id, phone) VALUES (?, ?, ?) ON CONFLICT DO NOTHING")
                        .bind(&new_contact_id).bind(&tenant_id).bind(&sender_id).execute(&mut *tx).await;

                    let contact_res = sqlx::query("SELECT id FROM chat_contacts WHERE tenant_id = ? AND phone = ? LIMIT 1")
                        .bind(&tenant_id).bind(&sender_id).fetch_optional(&mut *tx).await;

                    if let Ok(Some(contact_row)) = contact_res {
                        let contact_id: Uuid = contact_row.get("id");

                        let new_conv_id = Uuid::new_v4();
                        let _ = sqlx::query("INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id) VALUES (?, ?, ?, ?) ON CONFLICT DO NOTHING")
                            .bind(&new_conv_id).bind(&tenant_id).bind(&inbox_id).bind(&contact_id).execute(&mut *tx).await;

                        let conv_res = sqlx::query("SELECT id FROM chat_conversations WHERE tenant_id = ? AND inbox_id = ? AND contact_id = ? LIMIT 1")
                            .bind(&tenant_id).bind(&inbox_id).bind(&contact_id).fetch_optional(&mut *tx).await;

                        if let Ok(Some(conv_row)) = conv_res {
                            let conversation_id: Uuid = conv_row.get("id");

                            let msg_id = if provider_message_id.is_empty() {
                                let hash_input = format!("{}:{}:{}", tenant_id, conversation_id, message);
                                Uuid::new_v5(&Uuid::NAMESPACE_OID, hash_input.as_bytes())
                            } else {
                                Uuid::new_v5(&Uuid::NAMESPACE_OID, provider_message_id.as_bytes())
                            };
                            stable_msg_id = msg_id.to_string();

                            let msg_insert_res = sqlx::query("INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content) VALUES (?, ?, ?, 'contact', ?, ?) ON CONFLICT (id) DO NOTHING")
                                .bind(&msg_id).bind(&tenant_id).bind(&conversation_id).bind(&contact_id).bind(&message).execute(&mut *tx).await;

                            if let Ok(res) = msg_insert_res {
                                if res.rows_affected() > 0 {
                                    let topic = format!("unified:chat:{}", tenant_id);
                                    let ws_payload = serde_json::json!({
                                        "action": "new_message",
                                        "message_id": msg_id.to_string(),
                                        "content": message
                                    });

                                    let mut published = false;
                                    if let Some(client) = crate::redis_pool::get_redis_client() {
                                        if let Ok(mut rconn) = client.get_connection() {
                                            let publish_res: Result<(), redis::RedisError> = redis::cmd("PUBLISH")
                                                .arg(&topic)
                                                .arg(ws_payload.to_string())
                                                .query(&mut rconn);
                                            if publish_res.is_ok() {
                                                published = true;
                                            }
                                        }
                                    }

                                    if !published {
                                        let outbox_job_id = Uuid::new_v4().to_string();
                                        let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'publish_chat_event', ?, 'PENDING')")
                                            .bind(&outbox_job_id).bind(&tenant_id_str).bind(ws_payload.to_string()).execute(&mut *tx).await;
                                    }
                                }

                                let _ = tx.commit().await;
                            } else {
                                let _ = tx.rollback().await;
                                return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
                            }
                        } else {
                            let _ = tx.rollback().await;
                        }
                    } else {
                        let _ = tx.rollback().await;
                    }
                } else {
                    let _ = tx.rollback().await;
                }
            } else {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
            }
        }
    };
    if stable_msg_id.is_empty() {
        stable_msg_id = Uuid::new_v4().to_string();
    }

    // 3. Enqueue to ohc_job_queue
    let job_id = Uuid::new_v4().to_string();
    let mut payload_json = serde_json::json!({
        "message_id": stable_msg_id,
        "inbox_message_id": stable_msg_id,
        "source": source,
        "content": message,
        "sender_id": sender_id
    });

    if let Ok(c_id) = &customer_id_result {
        payload_json["customer_id"] = serde_json::json!(c_id);
    }

    let enqueue_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'message_triage', $3, 'PENDING')")
                .bind(&job_id)
                .bind(&tenant_id_str)
                .bind(payload_json.to_string())
                .execute(&state.db.pool)
                .await
                .map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'message_triage', ?, 'PENDING')")
                .bind(&job_id)
                .bind(&tenant_id_str)
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
        tenant_id: tenant_id_str.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: payload_json,
    };

    let orchestrator_clone = state.orchestrator.clone();
    tokio::spawn(async move {
        let _ = orchestrator_clone.dispatch_event(event).await;
    });

    (StatusCode::OK, Json(WebhookResponse { success: true })).into_response()
}
