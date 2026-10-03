use crate::db::DB;
use crate::domain::repository::omnichannel_repo::{
    Conversation, Message, MessageCursor, MessagePage, OmniChannelRepo, WidgetChatError,
};
use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

pub struct WidgetChatState {
    pub repo: OmniChannelRepo,
}
impl WidgetChatState {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            repo: OmniChannelRepo::new(db),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateConversationRequest {
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
}

pub async fn create_conversation(
    State(state): State<Arc<WidgetChatState>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(req): Json<CreateConversationRequest>,
) -> Result<Json<Conversation>, StatusCode> {
    let tenant = verified_tenant(&claims, req.tenant_id)?;
    state
        .repo
        .create_conversation(tenant, req.inbox_id, req.contact_id)
        .await
        .map(Json)
        .map_err(status)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateMessageRequest {
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
}

pub async fn create_message(
    State(state): State<Arc<WidgetChatState>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(req): Json<CreateMessageRequest>,
) -> Result<Json<Message>, StatusCode> {
    let tenant = verified_tenant(&claims, req.tenant_id)?;
    // Strict middleware rechecks this actor's current stored membership. An
    // account ID is opaque text, never a request-provided or invented UUID.
    state
        .repo
        .create_message(tenant, req.conversation_id, &claims.sub, req.content)
        .await
        .map(Json)
        .map_err(status)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryQuery {
    limit: Option<usize>,
    cursor: Option<String>,
}

pub async fn get_messages(
    State(state): State<Arc<WidgetChatState>>,
    Extension(claims): Extension<::server_common::Claims>,
    Path((tenant_id, conversation_id)): Path<(Uuid, Uuid)>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<MessagePage>, StatusCode> {
    let tenant = verified_tenant(&claims, tenant_id)?;
    let cursor = query
        .cursor
        .as_deref()
        .map(MessageCursor::decode)
        .transpose()
        .map_err(status)?;
    state
        .repo
        .get_messages_by_conversation_id(tenant, conversation_id, query.limit.unwrap_or(50), cursor)
        .await
        .map(Json)
        .map_err(status)
}

fn verified_tenant(claims: &::server_common::Claims, target: Uuid) -> Result<Uuid, StatusCode> {
    let raw = claims
        .organization_id
        .as_deref()
        .ok_or(StatusCode::FORBIDDEN)?;
    let tenant = Uuid::parse_str(raw).map_err(|_| StatusCode::FORBIDDEN)?;
    // Never collapse distinct raw account IDs by trimming or case normalization.
    if raw != tenant.to_string() || tenant != target {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(tenant)
}
fn status(error: WidgetChatError) -> StatusCode {
    match error {
        WidgetChatError::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
        WidgetChatError::NotFound => StatusCode::NOT_FOUND,
        WidgetChatError::InvalidRequest => StatusCode::BAD_REQUEST,
        WidgetChatError::Corrupt => StatusCode::INTERNAL_SERVER_ERROR,
        WidgetChatError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
