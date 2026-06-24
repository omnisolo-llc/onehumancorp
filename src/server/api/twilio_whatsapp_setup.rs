use axum::{extract::State, Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TwilioWhatsAppCredentials {
    pub bot_token: String, // Maps to accountSid in UI
    pub api_token: String, // Maps to authToken in UI
    pub from_phone: String, // Maps to phoneNumber in UI
}

#[derive(Serialize)]
pub struct TwilioWhatsAppSetupResponse {
    pub success: bool,
    pub message: String,
}

pub async fn twilio_whatsapp_setup_handler(
    State(state): State<crate::api::twilio_webhook::TwilioWebhookState>,
    Json(payload): Json<TwilioWhatsAppCredentials>,
) -> impl IntoResponse {
    let tenant_id = "test_tenant"; // In a real app this comes from auth context

    let update_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "UPDATE settings SET twilio_whatsapp_account_sid = $1, twilio_whatsapp_auth_token = $2, twilio_whatsapp_phone_number = $3 WHERE tenant_id = $4"
            )
            .bind(&payload.bot_token)
            .bind(&payload.api_token)
            .bind(&payload.from_phone)
            .bind(tenant_id)
            .execute(&state.db.pool)
            .await.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                "UPDATE settings SET twilio_whatsapp_account_sid = ?, twilio_whatsapp_auth_token = ?, twilio_whatsapp_phone_number = ? WHERE tenant_id = ?"
            )
            .bind(&payload.bot_token)
            .bind(&payload.api_token)
            .bind(&payload.from_phone)
            .bind(tenant_id)
            .execute(sqlite_pool)
            .await.map(|_| ())
        }
    };

    match update_result {
        Ok(_) => {
            Json(TwilioWhatsAppSetupResponse {
                success: true,
                message: "Credentials saved".to_string(),
            }).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to save WhatsApp credentials: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
