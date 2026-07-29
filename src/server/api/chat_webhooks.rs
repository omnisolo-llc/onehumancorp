use axum::{
    extract::{Json, State},
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::DB;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::services::chat::service::ChatService;
use crate::orchestration::departments::types::DepartmentEvent;
use crate::hub::Hub;

#[derive(Clone)]
pub struct ChatWebhookState {
    pub hub: Arc<Hub>,
    pub db: Arc<DB>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
    pub chat_service: ChatService,
}

#[derive(Deserialize, Debug)]
pub struct TwilioSmsPayload {
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "To")]
    pub to: String,
    #[serde(rename = "Body")]
    pub body: String,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub success: bool,
}

pub async fn twilio_chat_webhook_handler(
    State(state): State<ChatWebhookState>,
    axum::extract::Form(payload): axum::extract::Form<TwilioSmsPayload>,
) -> impl IntoResponse {
    let clean_to = payload.to.replace("whatsapp:", "");
    let clean_from = payload.from.replace("whatsapp:", "");

    let tenant_id_opt = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query_scalar::<_, String>(
                "SELECT tenant_id FROM integration_credentials WHERE (from_phone = $1 OR from_phone = $2) AND integration_id IN ('twilio', 'whatsapp', 'whatsapp_cloud_api') LIMIT 1"
            )
            .bind(&payload.to)
            .bind(&clean_to)
            .fetch_optional(&state.db.pool)
            .await.unwrap_or(None)
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query_scalar::<_, String>(
                "SELECT tenant_id FROM integration_credentials WHERE (from_phone = ? OR from_phone = ?) AND integration_id IN ('twilio', 'whatsapp', 'whatsapp_cloud_api') LIMIT 1"
            )
            .bind(&payload.to)
            .bind(&clean_to)
            .fetch_optional(sqlite_pool)
            .await.unwrap_or(None)
        }
    };

    let tenant_id_str = match tenant_id_opt {
        Some(tid) => tid,
        None => return (StatusCode::NOT_FOUND, Json(WebhookResponse { success: false })).into_response(),
    };

    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(WebhookResponse { success: false })).into_response(),
    };

    let contact = match state.chat_service.create_contact(tenant_id, None, None, Some(clean_from.clone())).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to create contact: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
        }
    };

    let inbox = match state.chat_service.create_inbox(tenant_id, "Twilio SMS".to_string()).await {
         Ok(i) => i,
         Err(e) => {
             tracing::error!("Failed to create inbox: {}", e);
             return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
         }
    };

    let conversation = match state.chat_service.start_conversation(tenant_id, inbox.id, contact.id, None).await {
         Ok(c) => c,
         Err(e) => {
             tracing::error!("Failed to create conversation: {}", e);
             return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
         }
    };

    let message = match state.chat_service.send_message(tenant_id, conversation.id, "contact".to_string(), Some(contact.id), payload.body.clone()).await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
        }
    };

    let job_id = Uuid::new_v4().to_string();
    let payload_json = serde_json::json!({
        "message_id": message.id.to_string(),
        "inbox_message_id": message.id.to_string(),
        "source": "Twilio SMS",
        "content": payload.body,
        "sender_id": clean_from,
        "customer_id": contact.id.to_string(),
    });

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

    let event = DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: tenant_id_str,
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: payload_json,
    };

    let orchestrator_clone = state.orchestrator.clone();
    tokio::spawn(async move {
        let _ = orchestrator_clone.dispatch_event(event).await;
    });

    (StatusCode::OK, Json(WebhookResponse { success: true })).into_response()
}

#[derive(Deserialize, Debug)]
pub struct WebWidgetPayload {
    pub tenant_id: String,
    pub customer_email: Option<String>,
    pub customer_name: Option<String>,
    pub message: String,
}

pub async fn webwidget_chat_webhook_handler(
    State(state): State<ChatWebhookState>,
    Json(payload): Json<WebWidgetPayload>,
) -> impl IntoResponse {
    let tenant_id_str = payload.tenant_id.clone();
    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(WebhookResponse { success: false })).into_response(),
    };

    let contact = match state.chat_service.create_contact(tenant_id, payload.customer_name, payload.customer_email.clone(), None).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to create contact: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
        }
    };

    let inbox = match state.chat_service.create_inbox(tenant_id, "Web Widget".to_string()).await {
         Ok(i) => i,
         Err(e) => {
             tracing::error!("Failed to create inbox: {}", e);
             return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
         }
    };

    let conversation = match state.chat_service.start_conversation(tenant_id, inbox.id, contact.id, None).await {
         Ok(c) => c,
         Err(e) => {
             tracing::error!("Failed to create conversation: {}", e);
             return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
         }
    };

    let message = match state.chat_service.send_message(tenant_id, conversation.id, "contact".to_string(), Some(contact.id), payload.message.clone()).await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false })).into_response();
        }
    };

    let job_id = Uuid::new_v4().to_string();
    let sender_id = payload.customer_email.unwrap_or_else(|| "anonymous_web".to_string());

    let payload_json = serde_json::json!({
        "message_id": message.id.to_string(),
        "inbox_message_id": message.id.to_string(),
        "source": "Web Widget",
        "content": payload.message,
        "sender_id": sender_id,
        "customer_id": contact.id.to_string(),
    });

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

    let event = DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: tenant_id_str,
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: payload_json,
    };

    let orchestrator_clone = state.orchestrator.clone();
    tokio::spawn(async move {
        let _ = orchestrator_clone.dispatch_event(event).await;
    });

    (StatusCode::OK, Json(WebhookResponse { success: true })).into_response()
}

use crate::common::Claims;
use crate::common::auth_utils::strict_ui_claim_tenant;
use axum::extract::Extension;
use axum::extract::Path;

#[derive(Serialize)]
pub struct ConversationListResponse {
    pub conversations: Vec<crate::services::chat::models::ChatConversation>,
}

#[derive(Serialize)]
pub struct MessageListResponse {
    pub messages: Vec<crate::services::chat::models::ChatMessage>,
}

pub async fn list_chat_conversations_handler(
    State(state): State<ChatWebhookState>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id_str = match strict_ui_claim_tenant(&claims) {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Json(ConversationListResponse { conversations: vec![] })).into_response(),
    };

    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(ConversationListResponse { conversations: vec![] })).into_response(),
    };

    match state.chat_service.list_conversations(tenant_id).await {
        Ok(conversations) => (StatusCode::OK, Json(ConversationListResponse { conversations })).into_response(),
        Err(e) => {
            tracing::error!("Failed to list conversations: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ConversationListResponse { conversations: vec![] })).into_response()
        }
    }
}

pub async fn list_chat_messages_handler(
    State(state): State<ChatWebhookState>,
    Extension(claims): Extension<Claims>,
    Path(conversation_id): Path<Uuid>,
) -> impl IntoResponse {
    let tenant_id_str = match strict_ui_claim_tenant(&claims) {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, Json(MessageListResponse { messages: vec![] })).into_response(),
    };

    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(u) => u,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(MessageListResponse { messages: vec![] })).into_response(),
    };

    match state.chat_service.list_messages(tenant_id, conversation_id).await {
        Ok(messages) => (StatusCode::OK, Json(MessageListResponse { messages })).into_response(),
        Err(e) => {
            tracing::error!("Failed to list messages: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(MessageListResponse { messages: vec![] })).into_response()
        }
    }
}
