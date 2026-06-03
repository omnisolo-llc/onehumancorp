use axum::{Json, routing::{post, get}, Router, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookPayload {
    pub channel: String, // 'sms', 'whatsapp', 'ig', 'webchat'
    pub sender_id: String,
    pub content: String,
    pub tenant_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct UnifiedMessage {
    pub id: String,
    pub tenant_id: String,
    pub sender_id: String,
    pub channel: String,
    pub content: String,
    pub status: String,
    pub confidence_score: Option<f64>,
    pub draft_reply: Option<String>,
}

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/webhook", post(handle_webhook))
        .route("/messages", get(list_messages))
}

pub async fn handle_webhook(
    axum::extract::State(db): axum::extract::State<PgPool>,
    Json(payload): Json<WebhookPayload>,
) -> Result<Json<UnifiedMessage>, (StatusCode, String)> {
    let id = Uuid::new_v4().to_string();

    // In a real implementation we would route this to The Ambassador AI Agent to compute
    // confidence_score and draft_reply. For this exercise, we simulate the AI logic.
    let confidence_score = if payload.content.contains("vegan") {
        Some(0.85) // Medium confidence: draft reply
    } else if payload.content.contains("hours") {
        Some(0.95) // High confidence: auto reply
    } else {
        Some(0.50) // Low confidence: escalate
    };

    let draft_reply = if payload.content.contains("vegan") {
        Some(format!("Yes, we have vegan options available!"))
    } else if payload.content.contains("hours") {
        Some(format!("We are open 9am to 5pm, Monday to Friday."))
    } else {
        None
    };

    let status = if confidence_score.unwrap_or(0.0) > 0.90 {
        "auto-replied".to_string()
    } else if confidence_score.unwrap_or(0.0) > 0.70 {
        "needs-review".to_string()
    } else {
        "escalated".to_string()
    };

    let msg = UnifiedMessage {
        id: id.clone(),
        tenant_id: payload.tenant_id.clone(),
        sender_id: payload.sender_id.clone(),
        channel: payload.channel.clone(),
        content: payload.content.clone(),
        status: status.clone(),
        confidence_score,
        draft_reply: draft_reply.clone(),
    };

    let pool = db.clone();
    sqlx::query(
        r#"
        INSERT INTO unified_messages (id, tenant_id, sender_id, channel, content, status, confidence_score, draft_reply)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(&msg.id)
    .bind(&msg.tenant_id)
    .bind(&msg.sender_id)
    .bind(&msg.channel)
    .bind(&msg.content)
    .bind(&msg.status)
    .bind(msg.confidence_score)
    .bind(&msg.draft_reply)
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(msg))
}

pub async fn list_messages(
    axum::extract::State(db): axum::extract::State<PgPool>,
) -> Result<Json<Vec<UnifiedMessage>>, (StatusCode, String)> {
    let pool = db.clone();
    let records = sqlx::query_as::<_, UnifiedMessage>(
        r#"
        SELECT id, tenant_id, sender_id, channel, content, status, confidence_score, draft_reply
        FROM unified_messages
        ORDER BY timestamp DESC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let messages = records.into_iter().map(|r| UnifiedMessage {
        id: r.id,
        tenant_id: r.tenant_id,
        sender_id: r.sender_id,
        channel: r.channel,
        content: r.content,
        status: r.status,
        confidence_score: r.confidence_score,
        draft_reply: r.draft_reply,
    }).collect();

    Ok(Json(messages))
}
