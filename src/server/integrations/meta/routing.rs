use axum::{
    extract::{Query, State, Json},
    response::IntoResponse,
};
use serde::Deserialize;
use std::collections::HashMap;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct MetaWebhookPayload {
    pub object: String,
    pub entry: Vec<MetaEntry>,
}

#[derive(Deserialize)]
pub struct MetaEntry {
    pub id: String,
    pub messaging: Option<Vec<MetaMessaging>>,
}

#[derive(Deserialize)]
pub struct MetaMessaging {
    pub sender: MetaId,
    pub message: Option<MetaMessageBody>,
}

#[derive(Deserialize)]
pub struct MetaId {
    pub id: String,
}

#[derive(Deserialize)]
pub struct MetaMessageBody {
    pub text: String,
}

pub async fn meta_webhook_verify(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    if let Some(challenge) = params.get("hub.challenge") {
        return challenge.clone();
    }
    "Failed".to_string()
}

// Use normal query to avoid macro issues
pub async fn meta_webhook_receive(
    State(pool): State<PgPool>,
    Json(payload): Json<MetaWebhookPayload>
) -> impl IntoResponse {
    for entry in payload.entry {
        if let Some(messaging) = entry.messaging {
            for msg in messaging {
                if let Some(body) = msg.message {
                    let _ = sqlx::query(
                        "INSERT INTO messages (tenant_id, sender_id, text, channel) VALUES ($1, $2, $3, $4)"
                    )
                    .bind("default_tenant")
                    .bind(&msg.sender.id)
                    .bind(&body.text)
                    .bind("meta")
                    .execute(&pool).await;
                }
            }
        }
    }
    "EVENT_RECEIVED".to_string()
}

pub async fn meta_oauth_callback(
    State(pool): State<PgPool>,
    Query(params): Query<HashMap<String, String>>
) -> impl IntoResponse {
    if let Some(code) = params.get("code") {
        let _ = sqlx::query(
            "INSERT INTO integration_tokens (integration_id, access_token) VALUES ('meta', $1) ON CONFLICT DO UPDATE SET access_token = $1"
        )
        .bind(code)
        .execute(&pool).await;
        return "OAuth Successful".to_string();
    }
    "Failed".to_string()
}
