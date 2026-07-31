use axum::{
    extract::{Extension, Json, State},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;

#[derive(Deserialize, Debug, Clone)]
pub struct OmnichannelPayload {
    pub tenant_id: String,
    #[serde(alias = "source")]
    pub channel: String,
    pub sender_id: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub message_id: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub orchestrator: Arc<DepartmentOrchestrator>,
    pub db: Arc<crate::db::DB>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", post(handle_omnichannel_webhook))
        .with_state(state)
}

pub async fn handle_omnichannel_webhook(
    State(state): State<AppState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<OmnichannelPayload>,
) -> impl IntoResponse {
    let tenant_id_str = match claims.organization_id.as_deref() {
        Some(org_id) => {
            if org_id != payload.tenant_id.as_str() {
                return (StatusCode::UNAUTHORIZED, Json(WebhookResponse { success: false, message_id: None })).into_response();
            }
            org_id.to_string()
        },
        None => payload.tenant_id.clone(),
    };

    let tenant_id = match Uuid::parse_str(&tenant_id_str) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(WebhookResponse { success: false, message_id: None })).into_response(),
    };

    let channel_type = payload.channel.clone();
    let sender_id = payload.sender_id.clone();
    let message = payload.message.clone();

    // Setup ChatService
    let chat_service = crate::services::chat::service::ChatService::new(state.db.clone());

    // 1. Resolve Contact
    let contact = match chat_service.get_contact_by_identifier(tenant_id, &sender_id).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            let email = if sender_id.contains('@') { Some(sender_id.clone()) } else { None };
            let phone = if !sender_id.contains('@') && sender_id.chars().any(|c| c.is_digit(10)) { Some(sender_id.clone()) } else { None };
            // If they are not returning, they are created
            match chat_service.create_contact(tenant_id, Some("Unknown Contact".to_string()), email, phone).await {
                Ok(c) => c,
                Err(e) => {
                    tracing::error!("Failed to create contact: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
                }
            }
        },
        Err(e) => {
            tracing::error!("Failed to get contact: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
        }
    };

    // 2. Resolve Inbox and Channel
    let default_inbox_name = "Default Omnichannel Inbox";
    let inbox = match chat_service.get_inbox_by_name(tenant_id, default_inbox_name).await {
        Ok(Some(i)) => i,
        Ok(None) => match chat_service.create_inbox(tenant_id, default_inbox_name.to_string()).await {
            Ok(i) => i,
            Err(e) => {
                tracing::error!("Failed to create inbox: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
            }
        },
        Err(e) => {
            tracing::error!("Failed to get inbox: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
        }
    };

    let channel = match chat_service.get_channel_by_type(tenant_id, inbox.id, &channel_type).await {
        Ok(Some(c)) => c,
        Ok(None) => match chat_service.create_channel(tenant_id, inbox.id, channel_type.clone(), serde_json::json!({})).await {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Failed to create channel: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
            }
        },
        Err(e) => {
            tracing::error!("Failed to get channel: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
        }
    };

    // 3. Resolve Conversation
    let conversation = match chat_service.get_conversation(tenant_id, inbox.id, contact.id).await {
        Ok(Some(c)) => c,
        Ok(None) => match chat_service.start_conversation(tenant_id, inbox.id, contact.id, None).await {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("Failed to start conversation: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
            }
        },
        Err(e) => {
            tracing::error!("Failed to get conversation: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
        }
    };

    // 4. Create Message
    let chat_message = match chat_service.send_message(tenant_id, conversation.id, "contact".to_string(), Some(contact.id), message.clone()).await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("Failed to create message: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
        }
    };

    // We add back the logic to queue AI workflow
    let job_id = Uuid::new_v4().to_string();
    let payload_json_ai = serde_json::json!({
        "message_id": chat_message.id.to_string(),
        "source": channel_type,
        "content": message,
        "sender_id": sender_id,
        "customer_id": contact.id.to_string(),
        "message": message
    });

    let enqueue_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'message_triage', $3, 'PENDING')")
                .bind(&job_id)
                .bind(tenant_id_str.clone())
                .bind(payload_json_ai.to_string())
                .execute(&state.db.pool)
                .await
                .map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'message_triage', ?, 'PENDING')")
                .bind(&job_id)
                .bind(tenant_id_str.clone())
                .bind(payload_json_ai.to_string())
                .execute(sqlite_pool)
                .await
                .map(|_| ())
        }
    };

    if let Err(e) = enqueue_result {
        tracing::error!("Failed to enqueue message_triage job: {}", e);
    }

    // 5. Fire event to trigger UI push
    let payload_json = serde_json::json!({
        "message_id": chat_message.id.to_string(),
        "conversation_id": conversation.id.to_string(),
        "inbox_id": inbox.id.to_string(),
        "channel_id": channel.id.to_string(),
        "contact_id": contact.id.to_string(),
        "source": channel_type,
        "content": message,
        "sender_id": sender_id,
        "tenant_id": tenant_id_str
    });

    let event = crate::orchestration::departments::types::DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: tenant_id_str,
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: payload_json,
    };

    let orchestrator_clone = state.orchestrator.clone();
    tokio::spawn(async move {
        let _ = orchestrator_clone.dispatch_event(event).await;
    });

    (StatusCode::OK, Json(WebhookResponse { success: true, message_id: Some(chat_message.id.to_string()) })).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DB;

    use sqlx::SqlitePool;

    use axum::extract::State;
    use axum::Json;
    use axum::response::IntoResponse;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_handle_omnichannel_webhook() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        let schema = "CREATE TABLE chat_inboxes (    id TEXT PRIMARY KEY,    tenant_id TEXT NOT NULL,    name TEXT NOT NULL,    created_at TEXT DEFAULT CURRENT_TIMESTAMP,    updated_at TEXT DEFAULT CURRENT_TIMESTAMP);CREATE TABLE chat_channels (    id TEXT PRIMARY KEY,    tenant_id TEXT NOT NULL,    inbox_id TEXT NOT NULL REFERENCES chat_inboxes(id) ON DELETE CASCADE,    channel_type TEXT NOT NULL,    config TEXT DEFAULT '{}',    created_at TEXT DEFAULT CURRENT_TIMESTAMP,    updated_at TEXT DEFAULT CURRENT_TIMESTAMP);CREATE TABLE chat_contacts (    id TEXT PRIMARY KEY,    tenant_id TEXT NOT NULL,    name TEXT,    email TEXT,    phone TEXT,    created_at TEXT DEFAULT CURRENT_TIMESTAMP,    updated_at TEXT DEFAULT CURRENT_TIMESTAMP);CREATE TABLE chat_conversations (    id TEXT PRIMARY KEY,    tenant_id TEXT NOT NULL,    inbox_id TEXT NOT NULL REFERENCES chat_inboxes(id) ON DELETE CASCADE,    contact_id TEXT NOT NULL REFERENCES chat_contacts(id) ON DELETE CASCADE,    assignee_id TEXT,    status TEXT NOT NULL DEFAULT 'open',    created_at TEXT DEFAULT CURRENT_TIMESTAMP,    updated_at TEXT DEFAULT CURRENT_TIMESTAMP);CREATE TABLE chat_messages (    id TEXT PRIMARY KEY,    tenant_id TEXT NOT NULL,    conversation_id TEXT NOT NULL REFERENCES chat_conversations(id) ON DELETE CASCADE,    sender_type TEXT NOT NULL,    sender_id TEXT,    content TEXT NOT NULL,    created_at TEXT DEFAULT CURRENT_TIMESTAMP,    updated_at TEXT DEFAULT CURRENT_TIMESTAMP); CREATE TABLE ohc_job_queue (id TEXT PRIMARY KEY, tenant_id TEXT, job_type TEXT, payload TEXT, status TEXT);";
        sqlx::query(schema).execute(&pool).await.unwrap();
        let db = DB {
            pool: sqlx::PgPool::connect_lazy("postgres://dummy").unwrap(),
            store: crate::db::DbStore::Sqlite(pool.clone()),
        };

        let transport = std::sync::Arc::new(ohc_builtin_agent::mesh::transport::InProcessTransport::new());
        let mesh = std::sync::Arc::new(crate::orchestration::mesh::CentrifugeNode::new(transport));
        let orchestrator = std::sync::Arc::new(crate::orchestration::departments::DepartmentOrchestrator::new(std::sync::Arc::new(db.clone()), mesh));

        let app_state = AppState { db: std::sync::Arc::new(db), orchestrator };

        let payload = OmnichannelPayload {
            tenant_id: "00000000-0000-0000-0000-000000000000".into(),
            channel: "sms".into(),
            sender_id: "+123".into(),
            message: "Hello".into(),
        };

        let claims = ::server_common::Claims {
            sub: "test".into(),
            exp: 0,
            iat: 0,
            organization_id: Some("00000000-0000-0000-0000-000000000000".into()),
            username: "test".into(),
            email: "test@example.com".into(),
            roles: vec![],
            session_id: None,
            jti: "test".into(),
        };

        let res = handle_omnichannel_webhook(State(app_state), Extension(claims), Json(payload)).await.into_response();

        assert_eq!(res.status(), StatusCode::OK);
    }
}
