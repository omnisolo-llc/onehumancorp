use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentEvent;
use crate::db::DB;
use super::identity::resolve_identity;

#[derive(Clone)]
pub struct OmnichannelWebhookState {
    pub db: Arc<DB>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

#[derive(Deserialize)]
pub struct OmnichannelPayload {
    pub tenant_id: String,
    pub source: String,
    pub sender_id: String,
    pub message: String,
    #[serde(default)]
    pub target_language: Option<String>,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub message_id: Option<String>,
}

pub fn router<S>(state: OmnichannelWebhookState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/webhook", post(handle_omnichannel_webhook))
        .with_state(state)
}

pub async fn handle_omnichannel_webhook(
    State(state): State<OmnichannelWebhookState>,
    Json(payload): Json<OmnichannelPayload>,
) -> impl IntoResponse {
    let customer_id = resolve_identity(&state.db, &payload.tenant_id, &payload.source, &payload.sender_id).await;

    let id = Uuid::new_v4().to_string();
    let target_language = payload.target_language.unwrap_or_else(|| "English".to_string());

    let pool = &state.db.pool;

    let insert_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                r#"
                INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, status, sender_id, customer_id, created_at)
                VALUES ($1, $2, $3, $4, $5, 'unread', $6, $7, NOW())
                "#
            )
            .bind(&id)
            .bind(&payload.tenant_id)
            .bind(&payload.source)
            .bind(&payload.message)
            .bind(&payload.message)
            .bind(&payload.sender_id)
            .bind(&customer_id)
            .execute(pool)
            .await.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                r#"
                INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, status, sender_id, customer_id, created_at)
                VALUES (?, ?, ?, ?, ?, 'unread', ?, ?, CURRENT_TIMESTAMP)
                "#
            )
            .bind(&id)
            .bind(&payload.tenant_id)
            .bind(&payload.source)
            .bind(&payload.message)
            .bind(&payload.message)
            .bind(&payload.sender_id)
            .bind(&customer_id)
            .execute(sqlite_pool)
            .await.map(|_| ())
        }
    };

    if let Err(e) = insert_result {
        tracing::error!("Failed to insert into inbox_messages: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
    }

    let event = DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: serde_json::json!({
            "source": payload.source,
            "original_message": payload.message,
            "target_language": target_language,
            "inbox_message_id": id,
            "sender_id": payload.sender_id,
            "customer_id": customer_id,
        }),
    };

    match state.orchestrator.dispatch_event(event).await {
        Ok(_) => (StatusCode::OK, Json(WebhookResponse { success: true, message_id: Some(id) })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response()
    }
}
