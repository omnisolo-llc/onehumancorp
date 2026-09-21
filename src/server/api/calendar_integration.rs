use ::server_common::Claims;
use axum::{
    extract::{Extension, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Redirect},
    routing::{get, post},
    Router,
};
use urlencoding::encode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::DB;

pub struct CalendarIntegrationState {
    pub db: Arc<DB>,
}

#[derive(Serialize, Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: String,
    pub state: String,
}

#[derive(Serialize, Debug)]
pub struct ConnectResponse {
    pub redirect_url: String,
    pub status: String,
}

pub async fn connect_google_calendar(
    Extension(claims): Extension<Claims>,
) -> Result<Json<ConnectResponse>, (StatusCode, Json<serde_json::Value>)> {
    let client_id = match std::env::var("GOOGLE_CALENDAR_CLIENT_ID") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "status": "error",
                    "message": "Google Calendar OAuth is not configured"
                })),
            ));
        }
    };
    let redirect_uri = match std::env::var("GOOGLE_CALENDAR_REDIRECT_URI") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "status": "error",
                    "message": "Google Calendar OAuth is not configured"
                })),
            ));
        }
    };

    let tenant_id = claims.organization_id.unwrap_or_default();
    let state_uuid = Uuid::new_v4().to_string();
    let state_payload = format!("{}::{}", tenant_id, state_uuid);

    let redirect_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&scope={}&response_type=code&access_type=offline&prompt=consent&state={}",
        encode(&client_id),
        encode(&redirect_uri),
        encode("https://www.googleapis.com/auth/calendar.events"),
        encode(&state_payload),
    );

    Ok(Json(ConnectResponse {
        status: "success".to_string(),
        redirect_url,
    }))
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
}

pub async fn google_calendar_oauth_callback(
    State(state): State<Arc<CalendarIntegrationState>>,
    Query(query): Query<OAuthCallbackQuery>,
) -> impl IntoResponse {
    let client_id = match std::env::var("GOOGLE_CALENDAR_CLIENT_ID") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => return (StatusCode::INTERNAL_SERVER_ERROR, "Not configured").into_response(),
    };
    let client_secret = match std::env::var("GOOGLE_CALENDAR_CLIENT_SECRET") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => return (StatusCode::INTERNAL_SERVER_ERROR, "Not configured").into_response(),
    };
    let redirect_uri = match std::env::var("GOOGLE_CALENDAR_REDIRECT_URI") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => return (StatusCode::INTERNAL_SERVER_ERROR, "Not configured").into_response(),
    };

    let parts: Vec<&str> = query.state.split("::").collect();
    if parts.len() != 2 {
        return (StatusCode::BAD_REQUEST, "Invalid state").into_response();
    }
    let tenant_id = parts[0];

    let token_req = reqwest::Client::new()
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", query.code.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", redirect_uri.as_str()),
        ])
        .send()
        .await;

    let token_res = match token_req {
        Ok(resp) if resp.status().is_success() => resp.json::<TokenResponse>().await,
        _ => {
            return (StatusCode::BAD_REQUEST, "Failed to exchange token").into_response();
        }
    };

    let token_data = match token_res {
        Ok(data) => data,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Invalid token response").into_response(),
    };

    let integration_id = "google_calendar";
    let status = "connected";
    let integration_code = uuid::Uuid::new_v4().to_string();

    let res = sqlx::query(
        "INSERT INTO tool_integrations (id, tenant_id, name, status, integration_code)
         VALUES ($1, $2, 'Google Workspace Calendar', $3, $4)
         ON CONFLICT (id) DO UPDATE SET status = $3, integration_code = $4"
    )
    .bind(integration_id)
    .bind(tenant_id)
    .bind(status)
    .bind(&integration_code)
    .execute(&state.db.pool)
    .await;

    if let Err(e) = res {
        tracing::error!("Failed to save Google Calendar integration: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }

    let creds_id = uuid::Uuid::new_v4().to_string();
    let bot_token = token_data.refresh_token.unwrap_or_default();

    let res = sqlx::query(
        "INSERT INTO integration_credentials (id, tenant_id, integration_id, bot_token, api_token, from_phone)
         VALUES ($1, $2, $3, $4, $5, '')
         ON CONFLICT (tenant_id, integration_id) DO UPDATE SET bot_token = CASE WHEN $4 != '' THEN $4 ELSE integration_credentials.bot_token END, api_token = $5"
    )
    .bind(&creds_id)
    .bind(tenant_id)
    .bind(integration_id)
    .bind(&bot_token)
    .bind(&token_data.access_token)
    .execute(&state.db.pool)
    .await;

    if let Err(e) = res {
        tracing::error!("Failed to save Google Calendar credentials: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
    }

    Redirect::to("/integrations").into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(db: Arc<DB>) -> Router<S> {
    let state = Arc::new(CalendarIntegrationState { db });
    Router::new()
        .route("/connect", post(connect_google_calendar))
        .route("/callback", get(google_calendar_oauth_callback))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::server_common::Claims;
    use axum::Json;

    fn mock_claims() -> Claims {
        Claims {
            sub: "user123".to_string(),
            exp: 9999999999,
            iat: 1,
            organization_id: Some("tenant123".to_string()),
            username: "owner".to_string(),
            email: "owner@example.com".to_string(),
            roles: vec!["owner".to_string()],
            session_id: None,
            jti: "jti123".to_string(),
        }
    }

    #[tokio::test]
    async fn test_connect_google_calendar() {
        unsafe {
            std::env::set_var(
                "GOOGLE_CALENDAR_CLIENT_ID",
                "client-123.apps.googleusercontent.com",
            );
            std::env::set_var(
                "GOOGLE_CALENDAR_REDIRECT_URI",
                "https://cloud.omnisolo.co/oauth/google-calendar/callback",
            );
        }

        let claims = mock_claims();
        let response = connect_google_calendar(Extension(claims)).await.unwrap();
        let Json(data) = response;

        assert_eq!(data.status, "success");
        assert!(data.redirect_url.contains("client-123.apps.googleusercontent.com"));
        assert!(data.redirect_url.contains("https%3A%2F%2Fcloud.omnisolo.co%2Foauth%2Fgoogle-calendar%2Fcallback"));
        assert!(data.redirect_url.contains("tenant123"));

        unsafe {
            std::env::remove_var("GOOGLE_CALENDAR_CLIENT_ID");
            std::env::remove_var("GOOGLE_CALENDAR_REDIRECT_URI");
        }
    }

    #[tokio::test]
    async fn test_connect_google_calendar_requires_configuration() {
        unsafe {
            std::env::set_var("GOOGLE_CALENDAR_CLIENT_ID", "");
            std::env::remove_var("GOOGLE_CALENDAR_REDIRECT_URI");
        }
        let claims = mock_claims();
        let response = connect_google_calendar(Extension(claims)).await;

        assert!(response.is_err());
        let (status, Json(data)) = response.unwrap_err();
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(data["status"], "error");
        assert_eq!(data["message"], "Google Calendar OAuth is not configured");
    }
}
