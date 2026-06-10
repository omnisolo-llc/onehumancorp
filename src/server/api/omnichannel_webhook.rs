use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use uuid::Uuid;
use crate::db::get_pool;

#[derive(Deserialize)]
pub struct OmnichannelWebhookPayload {
    pub tenant_id: String,
    pub source: String, // e.g. "instagram", "whatsapp", "email"
    pub sender_id: String, // phone, email, or social handle
    pub message: String,
}

#[derive(Serialize)]
pub struct OmnichannelWebhookResponse {
    pub success: bool,
    pub customer_id: Option<String>,
    pub request_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InboundMessage {
    pub source: String,
    pub sender_id: String,
    pub message: String,
    pub customer_id: Option<String>,
    pub inbox_message_id: String,
}

pub struct IdentityResolver;

impl IdentityResolver {
    pub async fn resolve_customer(tenant_id: &str, sender_id: &str, pool: &sqlx::PgPool) -> Result<Option<String>, sqlx::Error> {
        // Attempt to find a customer by matching email or phone
        let query = "SELECT id FROM customers WHERE tenant_id = $1 AND (email = $2 OR phone = $2) LIMIT 1";

        let result: Option<(String,)> = sqlx::query_as(query)
            .bind(tenant_id)
            .bind(sender_id)
            .fetch_optional(pool)
            .await?;

        Ok(result.map(|(id,)| id))
    }
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_omnichannel_webhook))
        .with_state(orchestrator)
}

async fn handle_omnichannel_webhook(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Json(payload): Json<OmnichannelWebhookPayload>,
) -> impl IntoResponse {
    let pool = get_pool();

    // 1. Identity Resolution
    let customer_id = match IdentityResolver::resolve_customer(&payload.tenant_id, &payload.sender_id, &pool).await {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Error resolving customer identity: {}", e);
            None
        }
    };

    // 2. Insert into inbox_messages
    let id = Uuid::new_v4().to_string();

    let _ = sqlx::query(
        r#"
        INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, translated_from_language, draft_reply, status, customer_id, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, 'unread', $8, NOW())
        "#
    )
    .bind(&id)
    .bind(&payload.tenant_id)
    .bind(&payload.source)
    .bind(&payload.message)
    .bind(&payload.message)
    .bind(None::<String>)
    .bind(None::<String>)
    .bind(&customer_id)
    .execute(&pool)
    .await;

    let target_language = "English".to_string();

    let _inbound_message = InboundMessage {
        source: payload.source.clone(),
        sender_id: payload.sender_id.clone(),
        message: payload.message.clone(),
        customer_id: customer_id.clone(),
        inbox_message_id: id.clone(),
    };

    let event = crate::orchestration::departments::types::DepartmentEvent {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: serde_json::json!({
            "source": payload.source,
            "original_message": payload.message,
            "message": payload.message,
            "sender_id": payload.sender_id,
            "customer_id": customer_id,
            "target_language": target_language,
            "inbox_message_id": id,
        }),
    };

    match orchestrator.dispatch_event(event).await {
        Ok(_) => (StatusCode::OK, Json(OmnichannelWebhookResponse { success: true, customer_id, request_id: Some(id) })).into_response(),
        Err(e) => {
            if e.contains("AI Budget exhausted") {
                (StatusCode::TOO_MANY_REQUESTS, Json(OmnichannelWebhookResponse { success: false, customer_id: None, request_id: None })).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(OmnichannelWebhookResponse { success: false, customer_id: None, request_id: None })).into_response()
            }
        }
    }
}
