use axum::{
    extract::State,
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::db::DB;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::services::omnichannel::identity::IdentityResolver;

#[derive(Clone)]
pub struct OmnichannelWebhookState {
    pub db: Arc<DB>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

#[derive(Deserialize, Debug)]
pub struct OmnichannelWebhookPayload {
    pub tenant_id: String,
    pub channel: String, // e.g., "instagram", "whatsapp", "email"
    pub sender_id: String, // The handle or number
    pub content: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub name: Option<String>,
}

pub async fn omnichannel_webhook_post_handler(
    State(state): State<OmnichannelWebhookState>,
    Json(payload): Json<OmnichannelWebhookPayload>,
) -> impl IntoResponse {
    if payload.content.trim().is_empty() {
        return StatusCode::OK.into_response();
    }

    tracing::info!("Received Omnichannel message from {} via {}: {}", payload.sender_id, payload.channel, payload.content);

    let pool = match &state.db.store {
        crate::db::DbStore::Postgres => state.db.pool.clone(),
        crate::db::DbStore::Sqlite(_) => state.db.pool.clone(), // Fallback
    };

    let resolver = IdentityResolver::new(state.db.store.clone(), pool.clone());

    let customer_id = match resolver.resolve_identity(
        &payload.tenant_id,
        &payload.channel,
        &payload.sender_id,
        payload.email.as_deref(),
        payload.phone.as_deref(),
        payload.name.as_deref(),
    ).await {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Failed to resolve identity: {}", e);
            // Fallback to unknown
            "unknown".to_string()
        }
    };

    let inbox_id = Uuid::new_v4().to_string();

    let insert_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status) VALUES ($1, $2, $3, $4, '', 'pending')"
            )
            .bind(&inbox_id)
            .bind(&payload.tenant_id)
            .bind(&payload.channel)
            .bind(&payload.content)
            .execute(&state.db.pool)
            .await.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                "INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status) VALUES (?, ?, ?, ?, '', 'pending')"
            )
            .bind(&inbox_id)
            .bind(&payload.tenant_id)
            .bind(&payload.channel)
            .bind(&payload.content)
            .execute(sqlite_pool)
            .await.map(|_| ())
        }
    };

    if let Err(e) = insert_result {
        tracing::error!("Failed to insert inbox message: {}", e);
    }

    let event = crate::orchestration::departments::types::DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: serde_json::json!({
            "source": payload.channel,
            "message": payload.content,
            "sender_id": payload.sender_id,
            "customer_id": customer_id,
            "inbox_message_id": inbox_id,
            "original_message": payload.content,
        }),
    };

    let orchestrator_clone = state.orchestrator.clone();
    tokio::spawn(async move {
        let _ = orchestrator_clone.dispatch_event(event).await;
    });

    StatusCode::OK.into_response()
}
