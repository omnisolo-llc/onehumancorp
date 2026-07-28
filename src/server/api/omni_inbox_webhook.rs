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
use crate::services::chat::service::ChatService;
use std::str::FromStr;

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

    let tenant_id = payload.tenant_id;
    let source = payload.source.to_lowercase();
    let sender_id = payload.sender_id;
    let message = payload.message;

    // 1. Identity Resolution
    let resolver = IdentityResolver::new(state.db.clone());
    let customer_id_result = resolver.resolve_or_create_customer(&tenant_id, &sender_id, &source).await;

    if let Err(e) = customer_id_result {
         tracing::error!("Failed to resolve identity: {}", e);
         return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
    }

    let customer_id = customer_id_result.as_ref().ok().map(|s| s.as_str());
    let omni_inbox_id = Uuid::new_v4().to_string();
    let tenant_uuid = Uuid::from_str(&tenant_id).unwrap_or(Uuid::new_v4());

    let chat_service = ChatService::new(state.db.pool.clone(), state.hub.pubsub.clone());

    let inbox_id = match sqlx::query_as::<_, (Uuid,)>("SELECT id FROM chat_inboxes WHERE tenant_id = $1 LIMIT 1")
        .bind(tenant_uuid)
        .fetch_optional(&state.db.pool).await {
        Ok(Some((id,))) => id,
        _ => chat_service.create_inbox(tenant_uuid, "Omni Inbox".to_string()).await.map(|i| i.id).unwrap_or(Uuid::new_v4()),
    };

    let contact_id = match sqlx::query_as::<_, (Uuid,)>("SELECT id FROM chat_contacts WHERE tenant_id = $1 AND phone = $2 LIMIT 1")
        .bind(tenant_uuid)
        .bind(&sender_id)
        .fetch_optional(&state.db.pool).await {
        Ok(Some((id,))) => id,
        _ => chat_service.create_contact(tenant_uuid, None, None, Some(sender_id.clone())).await.map(|c| c.id).unwrap_or(Uuid::new_v4()),
    };

    let conversation_id = match sqlx::query_as::<_, (Uuid,)>("SELECT id FROM chat_conversations WHERE tenant_id = $1 AND contact_id = $2 LIMIT 1")
        .bind(tenant_uuid)
        .bind(contact_id)
        .fetch_optional(&state.db.pool).await {
        Ok(Some((id,))) => id,
        _ => chat_service.start_conversation(tenant_uuid, inbox_id, contact_id, None).await.map(|c| c.id).unwrap_or(Uuid::new_v4()),
    };

    let _ = chat_service.send_message(tenant_uuid, conversation_id, "contact".to_string(), Some(contact_id), message.clone()).await;

    // Keep existing omni_inbox_messages insertion for backward compatibility or triage job dependence
    let insert_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, status, sender_id, customer_id, created_at) VALUES ($1, $2, $3, $4, $5, 'English', 'unread', $6, $7, NOW())"
            )
            .bind(&omni_inbox_id)
            .bind(&tenant_id)
            .bind(&source)
            .bind(&message)
            .bind(&message)
            .bind(&sender_id)
            .bind(customer_id)
            .execute(&state.db.pool)
            .await.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, status, sender_id, customer_id, created_at) VALUES (?, ?, ?, ?, ?, 'English', 'unread', ?, ?, CURRENT_TIMESTAMP)"
            )
            .bind(&omni_inbox_id)
            .bind(&tenant_id)
            .bind(&source)
            .bind(&message)
            .bind(&message)
            .bind(&sender_id)
            .bind(customer_id)
            .execute(sqlite_pool)
            .await.map(|_| ())
        }
    };

    if let Err(e) = insert_result {
        tracing::error!("Failed to insert omni_inbox_message: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
    }
    // 3. Enqueue to ohc_job_queue
    let job_id = Uuid::new_v4().to_string();
    let mut payload_json = serde_json::json!({
        "message_id": omni_inbox_id,
        "inbox_message_id": omni_inbox_id,
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

    (StatusCode::OK, Json(WebhookResponse { success: true })).into_response()
}
