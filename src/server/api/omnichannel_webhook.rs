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
    if claims.organization_id.as_deref() != Some(payload.tenant_id.as_str()) {
        return (
            StatusCode::FORBIDDEN,
            Json(WebhookResponse { success: false, message_id: None }),
        )
            .into_response();
    }
    let tenant_id = &payload.tenant_id;
    let channel = &payload.channel;
    let sender_id = &payload.sender_id;
    let message = &payload.message;

    // 1. Resolve Identity
    let resolver = crate::orchestration::identity_resolution::IdentityResolver::new(state.db.clone());
    let customer_id = resolver.resolve_or_create_customer(tenant_id, sender_id, channel).await.unwrap_or_else(|_| uuid::Uuid::new_v4().to_string());

    // Create ServiceLead if applicable
    let service_lead_id = uuid::Uuid::new_v4().to_string();
    if channel == "intake_form" || channel == "email_inquiry" || channel == "work_intake" || channel == "instagram_dm" || channel == "sms" || channel == "booking_form" {
        let _ = match &state.db.store {
            crate::db::DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO service_leads (id, tenant_id, customer_id, description, source, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'new', NOW(), NOW())")
                    .bind(&service_lead_id)
                    .bind(tenant_id)
                    .bind(uuid::Uuid::parse_str(&customer_id).ok())
                    .bind(message)
                    .bind(channel)
                    .execute(&state.db.pool)
                    .await;
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                let _ = sqlx::query("INSERT INTO service_leads (id, tenant_id, customer_id, description, source, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'new', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                    .bind(&service_lead_id)
                    .bind(tenant_id)
                    .bind(uuid::Uuid::parse_str(&customer_id).ok().map(|u| u.to_string()))
                    .bind(message)
                    .bind(channel)
                    .execute(sqlite_pool)
                    .await;
            }
        };
    }

    // 2. Persist Message into inbox_messages
    let inbox_id = Uuid::new_v4().to_string();
    let intent_id = Uuid::new_v4().to_string();
    let _ = match &state.db.store {
        crate::db::DbStore::Postgres => sqlx::query("INSERT INTO work_intents (id, tenant_id, source, intent_type, payload, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())")
            .bind(&intent_id)
            .bind(tenant_id)
            .bind(channel)
            .bind("customer_inquiry")
            .bind(serde_json::json!({"message": message, "sender_id": sender_id, "customer_id": customer_id}))
            .bind("PENDING")
            .execute(&state.db.pool).await.map(|_| ()).map_err(|e| e),
        crate::db::DbStore::Sqlite(sqlite_pool) => sqlx::query("INSERT INTO work_intents (id, tenant_id, source, intent_type, payload, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
            .bind(&intent_id)
            .bind(tenant_id)
            .bind(channel)
            .bind("customer_inquiry")
            .bind(serde_json::json!({"message": message, "sender_id": sender_id, "customer_id": customer_id}).to_string())
            .bind("PENDING")
            .execute(sqlite_pool).await.map(|_| ()).map_err(|e| e),
    };

    let insert_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            let res = sqlx::query(
                "INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, draft_reply, status, sender_id, created_at) VALUES ($1, $2, $3, $4, $5, '', 'unread', $6, NOW())"
            )
            .bind(&inbox_id)
            .bind(tenant_id)
            .bind(channel)
            .bind(message)
            .bind(message)
            .bind(sender_id)
            .execute(&state.db.pool)
            .await;

            if res.is_ok() {
                if let Err(e) = sqlx::query(
                    "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at) VALUES ($1, $2, $3, $4, $5, 'English', '', 'unread', $6, $7, NOW())"
                )
                .bind(&inbox_id)
                .bind(tenant_id)
                .bind(channel)
                .bind(message)
                .bind(message)
                .bind(sender_id)
                .bind(&customer_id)
                .execute(&state.db.pool)
                .await {
                    tracing::error!("Failed to insert omni_inbox_messages: {}", e);
                }
            }
            res.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let res = sqlx::query(
                "INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, draft_reply, status, sender_id, created_at) VALUES (?, ?, ?, ?, ?, '', 'unread', ?, CURRENT_TIMESTAMP)"
            )
            .bind(&inbox_id)
            .bind(tenant_id)
            .bind(channel)
            .bind(message)
            .bind(message)
            .bind(sender_id)
            .execute(sqlite_pool)
            .await;

            if res.is_ok() {
                if let Err(e) = sqlx::query(
                    "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at) VALUES (?, ?, ?, ?, ?, 'English', '', 'unread', ?, ?, CURRENT_TIMESTAMP)"
                )
                .bind(&inbox_id)
                .bind(tenant_id)
                .bind(channel)
                .bind(message)
                .bind(message)
                .bind(sender_id)
                .bind(&customer_id)
                .execute(sqlite_pool)
                .await {
                    tracing::error!("Failed to insert omni_inbox_messages (SQLite): {}", e);
                }
            }
            res.map(|_| ())
        }
    };

    if let Err(e) = insert_result {
        tracing::error!("Failed to insert omnichannel inbox message: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
    }

    // 3. Enqueue message_triage job
    let job_id = Uuid::new_v4().to_string();
    let payload_json = serde_json::json!({
        "message_id": inbox_id,
        "source": channel,
        "content": message,
        "sender_id": sender_id,
        "customer_id": customer_id,
        "service_lead_id": service_lead_id,
        "message": message
    });

    let enqueue_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'message_triage', $3, 'PENDING')")
                .bind(&job_id)
                .bind(tenant_id)
                .bind(payload_json.to_string())
                .execute(&state.db.pool)
                .await
                .map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'message_triage', ?, 'PENDING')")
                .bind(&job_id)
                .bind(tenant_id)
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

    (StatusCode::OK, Json(WebhookResponse { success: true, message_id: Some(inbox_id) })).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DB;

    use sqlx::SqlitePool;

}
