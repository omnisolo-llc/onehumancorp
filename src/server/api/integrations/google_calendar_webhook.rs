use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WebhookQuery {
    pub tenant_id: String,
}

pub async fn google_calendar_webhook_handler(
    headers: HeaderMap,
    Query(query): Query<WebhookQuery>,
) -> impl IntoResponse {
    let tenant_id = query.tenant_id;
    let channel_id = headers
        .get("x-goog-channel-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let resource_id = headers
        .get("x-goog-resource-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let resource_state = headers
        .get("x-goog-resource-state")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    tracing::info!(
        "Received Google Calendar webhook for tenant {}: channel={}, resource={}, state={}",
        tenant_id,
        channel_id,
        resource_id,
        resource_state
    );

    if resource_state == "sync" {
        return (StatusCode::OK, "Sync acknowledged").into_response();
    }

    // Validate token/channel
    // We assume some validation logic for the channel_id
    if channel_id.is_empty() {
        return (StatusCode::BAD_REQUEST, "Missing channel ID").into_response();
    }

    // Google Calendar notifications are hints.
    // Enqueue a reconciliation job to fetch the changes using `events.list` and `syncToken`.
    tracing::info!(
        "Enqueueing reconciliation job for tenant {} and channel {}",
        tenant_id,
        channel_id
    );
    let pool = crate::db::get_pool();

    let enqueue_res = sqlx::query(
        "INSERT INTO agent_jobs (tenant_id, job_type, payload, status)
         VALUES ($1, 'google_calendar_reconciliation', $2, 'pending')",
    )
    .bind(&tenant_id)
    .bind(serde_json::json!({
        "channel_id": channel_id,
        "resource_id": resource_id,
        "resource_state": resource_state
    }))
    .execute(&pool)
    .await;

    if let Err(e) = enqueue_res {
        tracing::error!("Failed to enqueue reconciliation job: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to process webhook",
        )
            .into_response();
    }

    (StatusCode::OK, "Webhook received").into_response()
}
