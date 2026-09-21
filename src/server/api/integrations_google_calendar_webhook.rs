use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde_json::Value;

#[derive(Clone)]
pub struct GoogleCalendarWebhookState {
    pub db: crate::db::DB,
    pub redis_client: redis::Client,
}

pub async fn google_calendar_webhook_handler(
    State(state): State<GoogleCalendarWebhookState>,
    headers: HeaderMap,
    Json(_payload): Json<Value>,
) -> impl IntoResponse {
    let channel_id = headers
        .get("x-goog-channel-id")
        .and_then(|h| h.to_str().ok());
    let resource_id = headers
        .get("x-goog-resource-id")
        .and_then(|h| h.to_str().ok());
    let resource_state = headers
        .get("x-goog-resource-state")
        .and_then(|h| h.to_str().ok());
    let message_number = headers
        .get("x-goog-message-number")
        .and_then(|h| h.to_str().ok());

    if channel_id.is_none() || resource_id.is_none() {
        return StatusCode::FORBIDDEN;
    }

    let channel_id = channel_id.unwrap();
    let message_number = message_number.unwrap_or("unknown");

    // Look up the channel ID in the calendar_integrations table
    let row = sqlx::query(
        "SELECT id, tenant_id FROM calendar_integrations WHERE provider = 'google_calendar' AND sync_metadata->>'webhook_channel_id' = $1"
    )
    .bind(channel_id)
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    if row.is_none() {
        return StatusCode::NOT_FOUND;
    }

    // Deduplicate using message number
    let redis_key = format!(
        "webhook:google_calendar:message_number:{}:{}",
        channel_id, message_number
    );
    let mut conn = match state.redis_client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let acquired: bool = redis::cmd("SET")
        .arg(&redis_key)
        .arg("1")
        .arg("EX")
        .arg(86400) // 24 hours TTL
        .arg("NX")
        .query_async(&mut conn)
        .await
        .unwrap_or(false);

    if !acquired {
        // Already processed this message
        return StatusCode::NO_CONTENT;
    }

    if let Some(res_state) = resource_state
        && (res_state == "sync" || res_state == "exists")
    {
        // Acknowledge the sync/exists notification
    }

    // Enqueue a background synchronization job
    let _ = redis::cmd("LPUSH")
        .arg("calendar_sync_jobs")
        .arg(channel_id)
        .query_async::<()>(&mut conn)
        .await;

    StatusCode::NO_CONTENT
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;

    #[tokio::test]
    async fn test_google_calendar_webhook_missing_headers() {
        let db = crate::db::DB::new().await.unwrap();
        let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let state = GoogleCalendarWebhookState { db, redis_client };
        let headers = HeaderMap::new();

        let response =
            google_calendar_webhook_handler(State(state), headers, Json(serde_json::json!({})))
                .await
                .into_response();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}
