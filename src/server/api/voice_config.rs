use axum::{
    extract::Extension,
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use ::server_common::Claims;
use crate::db::get_pool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoiceAgentConfigPayload {
    pub is_enabled: bool,
    pub allow_booking: bool,
    pub allow_sms_links: bool,
    pub primary_language: String,
    pub custom_instructions: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TranscriptLog {
    pub id: String,
    pub session_id: String,
    pub role: String,
    pub text: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CallSession {
    pub session_id: String,
    pub caller_phone: String,
    pub status: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub summary: String,
}


pub fn router() -> Router {
    Router::new()
        .route("/", get(get_voice_config).post(update_voice_config))
        .route("/logs", get(get_voice_logs))
}

async fn get_voice_config(
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let pool = get_pool();
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to start transaction").into_response(),
    };

    if let Err(_) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to set context").into_response();
    }

    let record = sqlx::query!(
        "SELECT is_enabled, allow_booking, allow_sms_links, primary_language, custom_instructions FROM voice_agent_configs WHERE tenant_id = $1",
        tenant_id
    )
    .fetch_optional(&mut *tx)
    .await;

    let config = match record {
        Ok(Some(row)) => {
            VoiceAgentConfigPayload {
                is_enabled: row.is_enabled.unwrap_or(false),
                allow_booking: row.allow_booking.unwrap_or(false),
                allow_sms_links: row.allow_sms_links.unwrap_or(false),
                primary_language: row.primary_language.unwrap_or_else(|| "English".to_string()),
                custom_instructions: row.custom_instructions.unwrap_or_default(),
            }
        },
        _ => {
            // Default response
            VoiceAgentConfigPayload {
                is_enabled: false,
                allow_booking: false,
                allow_sms_links: false,
                primary_language: "English".to_string(),
                custom_instructions: "".to_string(),
            }
        }
    };

    let _ = tx.commit().await;
    (StatusCode::OK, Json(config)).into_response()
}

async fn update_voice_config(
    Extension(claims): Extension<Claims>,
    Json(payload): Json<VoiceAgentConfigPayload>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    let pool = get_pool();
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to start transaction").into_response(),
    };

    if let Err(_) = crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to set context").into_response();
    }

    let existing = sqlx::query!(
        "SELECT id FROM voice_agent_configs WHERE tenant_id = $1",
        tenant_id
    )
    .fetch_optional(&mut *tx)
    .await;

    match existing {
        Ok(Some(row)) => {
            let id = row.id;
            let result = sqlx::query!(
                "UPDATE voice_agent_configs SET is_enabled = $1, allow_booking = $2, allow_sms_links = $3, primary_language = $4, custom_instructions = $5 WHERE id = $6",
                payload.is_enabled,
                payload.allow_booking,
                payload.allow_sms_links,
                payload.primary_language,
                payload.custom_instructions,
                id
            )
            .execute(&mut *tx)
            .await;

            if result.is_err() {
                 return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update config").into_response();
            }
        }
        _ => {
             let id = Uuid::new_v4().to_string();
             let result = sqlx::query!(
                "INSERT INTO voice_agent_configs (id, tenant_id, is_enabled, allow_booking, allow_sms_links, primary_language, custom_instructions) VALUES ($1, $2, $3, $4, $5, $6, $7)",
                id,
                tenant_id,
                payload.is_enabled,
                payload.allow_booking,
                payload.allow_sms_links,
                payload.primary_language,
                payload.custom_instructions
            )
            .execute(&mut *tx)
            .await;

            if result.is_err() {
                 return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert config").into_response();
            }
        }
    }

    let _ = tx.commit().await;
    (StatusCode::OK, Json(payload)).into_response()
}

async fn get_voice_logs(
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let _tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    // Return mock data for UI rendering per instructions
    let logs = vec![
        CallSession {
            session_id: "mock_session_1".to_string(),
            caller_phone: "+15551234567".to_string(),
            status: "Completed".to_string(),
            start_time: Utc::now() - chrono::Duration::hours(2),
            end_time: Some(Utc::now() - chrono::Duration::hours(2) + chrono::Duration::minutes(5)),
            summary: "Booked plumbing estimate for Tuesday".to_string(),
        },
        CallSession {
            session_id: "mock_session_2".to_string(),
            caller_phone: "+15559876543".to_string(),
            status: "Completed".to_string(),
            start_time: Utc::now() - chrono::Duration::days(1),
            end_time: Some(Utc::now() - chrono::Duration::days(1) + chrono::Duration::minutes(3)),
            summary: "Asked for business hours".to_string(),
        },
    ];

    (StatusCode::OK, Json(logs)).into_response()
}