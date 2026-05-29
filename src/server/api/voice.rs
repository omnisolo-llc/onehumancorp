use axum::{
    Json, Router,
    extract::{Form, State},
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct VoiceState {
    pub db: crate::db::DB,
}

#[derive(Deserialize)]
pub struct TwilioWebhookPayload {
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "To")]
    pub to: String,
    #[serde(rename = "CallSid")]
    pub call_sid: String,
}

pub fn router<S: Clone + Send + Sync + 'static>(db: crate::db::DB) -> Router<S> {
    let state = VoiceState { db };
    Router::new()
        .route("/webhook", post(twilio_webhook))
        .route("/settings", get(get_settings).post(update_settings))
        .with_state(state)
}

pub async fn twilio_webhook(
    State(state): State<VoiceState>,
    Form(payload): Form<TwilioWebhookPayload>,
) -> impl IntoResponse {
    let msg_id = uuid::Uuid::new_v4().to_string();
    let tenant_id = "default_tenant";
    let content = format!(
        "Voice Call received from {} to {}. CallSid: {}",
        payload.from, payload.to, payload.call_sid
    );

    match &state.db.store {
        crate::db::DbStore::Sqlite(pool) => {
            let _ = sqlx::query("INSERT INTO inbox_messages (id, tenant_id, source, content, status) VALUES (?, ?, ?, ?, ?)")
                .bind(&msg_id)
                .bind(tenant_id)
                .bind("voice")
                .bind(&content)
                .bind("unread")
                .execute(pool)
                .await;
        }
        crate::db::DbStore::Postgres => {
            let _ = sqlx::query("INSERT INTO inbox_messages (id, tenant_id, source, content, status) VALUES ($1, $2, $3, $4, $5)")
                .bind(&msg_id)
                .bind(tenant_id)
                .bind("voice")
                .bind(&content)
                .bind("unread")
                .execute(&state.db.pool)
                .await;
        }
    }

    let twiml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Response>
    <Say>Hello, thanks for calling. Please leave a message.</Say>
    <Record/>
</Response>"#;

    (
        [(axum::http::header::CONTENT_TYPE, "application/xml")],
        twiml,
    )
        .into_response()
}

#[derive(Serialize)]
pub struct SettingsResponse {
    pub status: String,
}

pub async fn get_settings() -> Json<SettingsResponse> {
    Json(SettingsResponse {
        status: "ok".to_string(),
    })
}

pub async fn update_settings() -> Json<SettingsResponse> {
    Json(SettingsResponse {
        status: "ok".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Form;

    #[tokio::test]
    async fn test_twilio_webhook_response() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy(database_url)
            .unwrap();

        let db = crate::db::DB {
            pool,
            store: crate::db::DbStore::Postgres,
        };

        let state = VoiceState { db };

        let payload = TwilioWebhookPayload {
            from: "+1234567890".to_string(),
            to: "+0987654321".to_string(),
            call_sid: "CA1234567890".to_string(),
        };

        let response = twilio_webhook(State(state), Form(payload))
            .await
            .into_response();

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get(axum::http::header::CONTENT_TYPE)
                .unwrap(),
            "application/xml"
        );
    }
}
