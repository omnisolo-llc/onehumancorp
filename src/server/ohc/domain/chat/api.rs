use axum::{
    extract::{Path, State, Json},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use super::models::{conversation, message};
use super::ws::AppState as WsAppState;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use uuid::Uuid;

pub struct ChatApiState {
    pub db: DatabaseConnection,
    pub ws_state: Arc<WsAppState>,
}

pub fn chat_api_router(state: Arc<ChatApiState>) -> Router {
    Router::new()
        .route("/api/v1/tenants/:tenant_id/conversations", get(get_conversations))
        .route("/api/v1/tenants/:tenant_id/conversations/:conversation_id/messages", get(get_messages))
        .route("/api/v1/tenants/:tenant_id/conversations/:conversation_id/messages", post(send_message))
        .with_state(state)
}

pub async fn get_conversations(
    Path(tenant_id): Path<Uuid>,
    State(state): State<Arc<ChatApiState>>,
) -> Json<Vec<conversation::Model>> {
    let convos = conversation::Entity::find()
        .filter(conversation::Column::TenantId.eq(tenant_id))
        .all(&state.db)
        .await
        .unwrap_or_default();

    Json(convos)
}

pub async fn get_messages(
    Path((tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
    State(state): State<Arc<ChatApiState>>,
) -> Json<Vec<message::Model>> {
    // Verify conversation belongs to tenant
    let convo = conversation::Entity::find_by_id(conversation_id)
        .filter(conversation::Column::TenantId.eq(tenant_id))
        .one(&state.db)
        .await;

    if let Ok(Some(_)) = convo {
        let msgs = message::Entity::find()
            .filter(message::Column::TenantId.eq(tenant_id))
            .filter(message::Column::ConversationId.eq(conversation_id))
            .all(&state.db)
            .await
            .unwrap_or_default();

        Json(msgs)
    } else {
        Json(vec![])
    }
}

#[derive(serde::Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
}

pub async fn send_message(
    Path((tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
    State(state): State<Arc<ChatApiState>>,
    Json(req): Json<SendMessageRequest>,
) -> Json<Option<message::Model>> {
    use sea_orm::ActiveValue::Set;
    use sea_orm::ActiveModelTrait;

    // Verify conversation belongs to tenant
    let convo = conversation::Entity::find_by_id(conversation_id)
        .filter(conversation::Column::TenantId.eq(tenant_id))
        .one(&state.db)
        .await;

    match convo {
        Ok(Some(_)) => {
            let msg = message::ActiveModel {
                id: Set(Uuid::new_v4()),
                tenant_id: Set(tenant_id),
                conversation_id: Set(conversation_id),
                sender_id: Set(None),
                content: Set(req.content),
                created_at: Set(chrono::Utc::now()),
                updated_at: Set(chrono::Utc::now()),
                ..Default::default()
            };

            if let Ok(inserted) = msg.insert(&state.db).await {
                // Broadcast to WS
                let tx = state.ws_state.get_or_create_channel(tenant_id).await;
                let _ = tx.send(inserted.clone());

                Json(Some(inserted))
            } else {
                Json(None)
            }
        },
        _ => Json(None)
    }
}
