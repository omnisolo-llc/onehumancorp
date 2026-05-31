use axum::{
    extract::Extension,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use crate::common::Claims;
use std::sync::RwLock;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref VOICE_SETTINGS: RwLock<HashMap<String, serde_json::Value>> = RwLock::new(HashMap::new());
}

pub async fn get_voice_settings_handler(Extension(user): Extension<Claims>) -> axum::response::Response {
    let org_id = user.organization_id.unwrap_or_default();
    let settings = {
        let store = VOICE_SETTINGS.read().unwrap();
        store.get(&org_id).cloned().unwrap_or_else(|| json!({
            "enabled": false,
            "greeting": "",
            "forward": ""
        }))
    };
    (axum::http::StatusCode::OK, Json(settings)).into_response()
}

pub async fn save_voice_settings_handler(Extension(user): Extension<Claims>, Json(payload): Json<serde_json::Value>) -> axum::response::Response {
    let org_id = user.organization_id.unwrap_or_default();
    {
        let mut store = VOICE_SETTINGS.write().unwrap();
        store.insert(org_id, payload);
    }
    (axum::http::StatusCode::OK, Json(json!({"success": true}))).into_response()
}

pub async fn incoming_voice_webhook_handler() -> axum::response::Response {
    // This is a mock webhook that simulates receiving an inbound call and injecting it into the DB.
    // Since we are mocking the Twilio side entirely, we will just insert a record into inbox_messages directly.
    let pool = crate::db::get_pool();
    let id = format!("voice_{}", uuid::Uuid::new_v4());

    // For demonstration, we'll try to find a real tenant if possible, or fallback to a dummy.
    let tenant_id = "demo_tenant"; // the e2e test uses a dynamic login, but mock webhooks can just inject to the default one for test purposes

    let res = sqlx::query(
        "INSERT INTO inbox_messages (id, tenant_id, source, content, draft_reply, status)
         VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING"
    )
    .bind(id)
    .bind(tenant_id)
    .bind("Voice Call")
    .bind("Caller wants a plumbing quote. Audio: [Voice Recording]")
    .bind("")
    .bind("UNREAD")
    .execute(&pool)
    .await;

    match res {
        Ok(_) => (axum::http::StatusCode::OK, Json(json!({"success": true}))).into_response(),
        Err(e) => {
            tracing::error!("Failed to insert mock voice message: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))).into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_voice_settings_empty() {
        let claims = Claims {
            sub: "test_user".to_string(),
            organization_id: Some("org_123".to_string()),
            exp: 9999999999,
            role: "admin".to_string(),
        };
        let res = get_voice_settings_handler(Extension(claims)).await;
        let (parts, _body) = res.into_parts();
        assert_eq!(parts.status, axum::http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_save_voice_settings() {
        let claims = Claims {
            sub: "test_user".to_string(),
            organization_id: Some("org_123".to_string()),
            exp: 9999999999,
            role: "admin".to_string(),
        };
        let payload = json!({
            "enabled": true,
            "greeting": "Hello",
            "forward": "123"
        });
        let res = save_voice_settings_handler(Extension(claims), Json(payload)).await;
        assert_eq!(res.status(), axum::http::StatusCode::OK);
    }
}
