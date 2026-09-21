use axum::{
    Json, Router,
    extract::{Extension, Query},
    response::{IntoResponse, Redirect},
    routing::get,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

pub fn router() -> Router<Arc<dyn omnisolo_builtin_agent::mesh::transport::MeshTransport>> {
    Router::new()
        .route("/connect", get(connect))
        .route("/callback", get(callback))
}

#[derive(Serialize, Deserialize)]
struct ConnectResponse {
    status: String,
    redirect_url: String,
}

async fn connect(Extension(claims): Extension<::server_common::Claims>) -> Json<ConnectResponse> {
    let client_id =
        std::env::var("GOOGLE_CALENDAR_CLIENT_ID").unwrap_or_else(|_| "mock-client-id".to_string());
    let redirect_uri = std::env::var("GOOGLE_CALENDAR_REDIRECT_URI").unwrap_or_else(|_| {
        "https://cloud.omnisolo.co/api/v1/oauth/google-calendar/callback".to_string()
    });

    let state = claims.organization_id.unwrap_or_default();

    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=https://www.googleapis.com/auth/calendar&access_type=offline&prompt=consent&state={}",
        urlencoding::encode(&client_id),
        urlencoding::encode(&redirect_uri),
        urlencoding::encode(&state)
    );

    Json(ConnectResponse {
        status: "success".to_string(),
        redirect_url: auth_url,
    })
}

#[derive(Deserialize)]
struct CallbackQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
}

async fn callback(Query(query): Query<CallbackQuery>) -> impl IntoResponse {
    if query.error.is_some() {
        return Redirect::to("/integrations?error=oauth_denied");
    }
    let code = match query.code {
        Some(c) => c,
        None => return Redirect::to("/integrations?error=missing_code"),
    };
    let tenant_id = match query.state {
        Some(s) => s,
        None => return Redirect::to("/integrations?error=missing_state"),
    };

    let client_id =
        std::env::var("GOOGLE_CALENDAR_CLIENT_ID").unwrap_or_else(|_| "mock-client-id".to_string());
    let client_secret = std::env::var("GOOGLE_CALENDAR_CLIENT_SECRET")
        .unwrap_or_else(|_| "mock-client-secret".to_string());
    let redirect_uri = std::env::var("GOOGLE_CALENDAR_REDIRECT_URI").unwrap_or_else(|_| {
        "https://cloud.omnisolo.co/api/v1/oauth/google-calendar/callback".to_string()
    });

    let client = Client::new();
    let res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", code.as_str()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", redirect_uri.as_str()),
        ])
        .send()
        .await;

    let (access_token, _refresh_token) = match res {
        Ok(resp) => {
            if resp.status().is_success() {
                let json: Value = resp.json().await.unwrap_or_default();
                let access = json["access_token"].as_str().unwrap_or("").to_string();
                let refresh = json["refresh_token"].as_str().unwrap_or("").to_string();
                (access, refresh)
            } else {
                return Redirect::to("/integrations?error=oauth_failed");
            }
        }
        Err(_) => return Redirect::to("/integrations?error=network_failed"),
    };

    if !access_token.is_empty() && !tenant_id.is_empty() {
        let registry = crate::integrations::registry::IntegrationsRegistry::new();
        let _ = registry.connect(
            "google_calendar",
            "https://www.googleapis.com/calendar/v3",
            ::server_omnisolo::orchestration::ConnectIntegrationRequest {
                bot_token: "".to_string(),
                chat_id: "".to_string(),
                webhook_url: "".to_string(),
                api_token: access_token,
                from_phone: "".to_string(),
                integration_id: "google_calendar".to_string(),
                base_url: "".to_string(),
            },
        );
    }

    Redirect::to("/integrations?success=google_calendar_connected")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_claims() -> ::server_common::Claims {
        ::server_common::Claims {
            sub: "user-123".to_string(),
            organization_id: Some("tenant-123".to_string()),
            roles: vec!["owner".to_string()],
            exp: 10000000000,
            iat: 10000000000,
            jti: "".to_string(),
            email: "".to_string(),
            session_id: None,
            username: "".to_string(),
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
                "https://cloud.omnisolo.co/api/v1/oauth/google-calendar/callback",
            );
        }

        let claims = mock_claims();
        let Json(response) = connect(Extension(claims)).await;
        let redirect_url = response.redirect_url;
        assert_eq!(response.status, "success");
        assert!(redirect_url.contains("client-123.apps.googleusercontent.com"));
        assert!(redirect_url.contains(
            "https%3A%2F%2Fcloud.omnisolo.co%2Fapi%2Fv1%2Foauth%2Fgoogle-calendar%2Fcallback"
        ));
        assert!(redirect_url.contains("tenant-123"));
    }
}
