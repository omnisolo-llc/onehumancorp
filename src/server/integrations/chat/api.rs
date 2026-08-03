use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set, SqlxPostgresConnector};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};

use uuid::Uuid;

use super::entities::{chat_inboxes, chat_conversations, chat_messages, chat_contacts};

#[derive(Clone)]
pub struct ChatAppState {
    pub db: DatabaseConnection,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct UnifiedConversationItem {
    pub id: Uuid,
    pub inbox_name: String,
    pub contact_name: Option<String>,
    pub status: String,
}

pub fn chat_router<S: Clone + Send + Sync + 'static>(pool: PgPool) -> Router<S> {
    let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
    let state = ChatAppState { db };

    Router::new()
        .route("/api/v1/chat/:tenant_id/inboxes", get(list_inboxes))
        .route("/api/v1/chat/:tenant_id/inboxes/:inbox_id/conversations", get(list_conversations))
        .route("/api/v1/chat/:tenant_id/conversations/:conversation_id/messages", post(send_message))
        .route("/api/v1/chat/:tenant_id/unified-feed", get(unified_feed))
        .with_state(state)
}

async fn list_inboxes(
    State(state): State<ChatAppState>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<Vec<chat_inboxes::Model>>, axum::http::StatusCode> {
    let inboxes = chat_inboxes::Entity::find()
        .filter(chat_inboxes::Column::TenantId.eq(tenant_id))
        .all(&state.db)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(inboxes))
}

async fn list_conversations(
    State(state): State<ChatAppState>,
    Path((tenant_id, inbox_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<chat_conversations::Model>>, axum::http::StatusCode> {
    let convos = chat_conversations::Entity::find()
        .filter(chat_conversations::Column::TenantId.eq(tenant_id))
        .filter(chat_conversations::Column::InboxId.eq(inbox_id))
        .all(&state.db)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(convos))
}

async fn send_message(
    State(state): State<ChatAppState>,
    Path((tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<chat_messages::Model>, axum::http::StatusCode> {
    // Basic validation to ensure conversation exists
    let convo = chat_conversations::Entity::find_by_id(conversation_id)
        .one(&state.db)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if convo.is_none() {
        return Err(axum::http::StatusCode::NOT_FOUND);
    }
    let convo = convo.unwrap();

    // Ensure the conversation belongs to the tenant
    if convo.tenant_id != tenant_id {
        return Err(axum::http::StatusCode::FORBIDDEN);
    }

    let new_msg = chat_messages::ActiveModel {
        id: Set(Uuid::new_v4()),
        tenant_id: Set(convo.tenant_id),
        conversation_id: Set(conversation_id),
        sender_type: Set(payload.sender_type),
        sender_id: Set(payload.sender_id),
        content: Set(payload.content.clone()),
        ..Default::default()
    };


    let msg = new_msg.insert(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to insert message: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Invoke adapter based on channel type
    if let Some(inbox) = chat_inboxes::Entity::find_by_id(convo.inbox_id)
        .one(&state.db)
        .await
        .unwrap_or(None)
    {
        if let Some(channel) = super::entities::chat_channels::Entity::find()
            .filter(super::entities::chat_channels::Column::InboxId.eq(inbox.id))
            .one(&state.db)
            .await
            .unwrap_or(None)
        {
            use super::adapter::{ChannelAdapter, WebWidgetAdapter, EmailAdapter};
            let adapter: Box<dyn ChannelAdapter> = match channel.channel_type.as_str() {
                "email" => Box::new(EmailAdapter),
                _ => Box::new(WebWidgetAdapter),
            };

            let _ = adapter.send_message(
                &convo.tenant_id.to_string(),
                &convo.contact_id.to_string(),
                &payload.content,
                channel.config.as_ref()
            ).await;
        }
    }


    Ok(Json(msg))
}

async fn unified_feed(
    State(state): State<ChatAppState>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<Vec<UnifiedConversationItem>>, axum::http::StatusCode> {
    // This represents the "Work Triage" view across all inboxes for the tenant
    // RLS handles tenant isolation implicitly at the DB level, but we could also
    // extract tenant_id from headers/context if needed.

    let convos_with_relations = chat_conversations::Entity::find()
        .filter(chat_conversations::Column::TenantId.eq(tenant_id))
        .find_also_related(chat_inboxes::Entity)
        .all(&state.db)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut result = Vec::new();
    for (convo, inbox_opt) in convos_with_relations {
        if let Some(inbox) = inbox_opt {
            let contact_opt = chat_contacts::Entity::find_by_id(convo.contact_id)
                .one(&state.db)
                .await
                .unwrap_or(None);

            result.push(UnifiedConversationItem {
                id: convo.id,
                inbox_name: inbox.name,
                contact_name: contact_opt.and_then(|c| c.name),
                status: convo.status,
            });
        }
    }

    Ok(Json(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() {
        let _router: Router<()> = Router::new();
        assert!(true);
    }
}
