use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use crate::db;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use std::str::FromStr;

pub struct TenantContext {
    pub tenant_id: Uuid,
}

impl<S> FromRequestParts<S> for TenantContext
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(tenant_id) = parts.headers.get("x-tenant-id") {
            if let Ok(tid) = tenant_id.to_str() {
                if let Ok(uuid) = Uuid::from_str(tid) {
                    return Ok(TenantContext { tenant_id: uuid });
                }
            }
        }

        Err((StatusCode::UNAUTHORIZED, "Missing or invalid x-tenant-id header"))
    }
}

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/v1/chat/inboxes", post(create_inbox).get(list_inboxes))
        .route("/api/v1/chat/conversations/:id/messages", post(send_message).get(list_messages))
        .with_state(pool)
}

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
}

pub async fn create_inbox(
    State(pool): State<PgPool>,
    tenant: TenantContext,
    Json(payload): Json<CreateInboxRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match db::create_inbox(&pool, tenant.tenant_id, &payload.name).await {
        Ok(inbox) => Ok((StatusCode::CREATED, Json(inbox))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn list_inboxes(
    State(pool): State<PgPool>,
    tenant: TenantContext,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match db::get_inboxes(&pool, tenant.tenant_id).await {
        Ok(inboxes) => Ok((StatusCode::OK, Json(inboxes))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
    pub sender_type: String,
}

pub async fn send_message(
    State(pool): State<PgPool>,
    tenant: TenantContext,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match db::create_message(
        &pool,
        tenant.tenant_id,
        conversation_id,
        &payload.content,
        &payload.sender_type,
    ).await {
        Ok(message) => Ok((StatusCode::CREATED, Json(message))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn list_messages(
    State(pool): State<PgPool>,
    tenant: TenantContext,
    Path(conversation_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match db::get_messages(&pool, tenant.tenant_id, conversation_id).await {
        Ok(messages) => Ok((StatusCode::OK, Json(messages))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
