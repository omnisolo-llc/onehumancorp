use axum::{
    extract::State,
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::Row;

#[derive(Clone)]
pub struct TwilioSettingsState {
    pub db: Arc<crate::db::DB>,
}

#[derive(Deserialize)]
pub struct UpdateTwilioSettingsRequest {
    pub send_sms_reminders: bool,
    pub phone_number: Option<String>,
}

pub async fn update_twilio_settings_handler(
    State(state): State<TwilioSettingsState>,
    request: axum::extract::Request,
) -> impl IntoResponse {
    let tenant_id = match request.extensions().get::<crate::auth::AuthInfo>() {
        Some(auth) => auth.tenant_id.clone(),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let body_bytes = axum::body::to_bytes(request.into_body(), usize::MAX).await.unwrap_or_default();
    let payload: UpdateTwilioSettingsRequest = match serde_json::from_slice(&body_bytes) {
        Ok(p) => p,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid payload").into_response(),
    };

    let res = match &state.db.store {
        crate::db::DbStore::Sqlite(pool) => {
            sqlx::query("UPDATE tenants SET sms_reminders_enabled = ?, sms_phone = ? WHERE tenant_id = ?")
                .bind(payload.send_sms_reminders)
                .bind(payload.phone_number)
                .bind(&tenant_id)
                .execute(pool)
                .await
        }
        crate::db::DbStore::Postgres => {
            sqlx::query("UPDATE tenants SET sms_reminders_enabled = $1, sms_phone = $2 WHERE tenant_id = $3")
                .bind(payload.send_sms_reminders)
                .bind(payload.phone_number)
                .bind(&tenant_id)
                .execute(&state.db.pool)
                .await
        }
    };

    match res {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
