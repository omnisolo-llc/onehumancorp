use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use crate::integrations::registry::IntegrationsRegistry;
use reqwest::Client;

#[derive(Clone)]
pub struct MetaOAuthState {
    pub registry: Arc<IntegrationsRegistry>,
    pub http_client: Client,
}

#[derive(Deserialize)]
pub struct MetaLoginQuery {
    pub tenant_id: String,
}

pub async fn meta_oauth_login_handler(
    Query(query): Query<MetaLoginQuery>,
) -> impl IntoResponse {
    let client_id = std::env::var("META_CLIENT_ID").unwrap_or_else(|_| "MOCK_META_CLIENT_ID".to_string());
    let redirect_uri = std::env::var("META_REDIRECT_URI").unwrap_or_else(|_| "http://localhost:3000/api/v1/oauth/meta/callback".to_string());

    // Pass tenant_id as state so we can recover it in the callback
    let state = query.tenant_id;
    let scopes = "whatsapp_business_messaging,whatsapp_business_management";

    let redirect_url = format!(
        "https://www.facebook.com/v19.0/dialog/oauth?client_id={}&redirect_uri={}&state={}&scope={}&response_type=code",
        client_id, redirect_uri, state, scopes
    );

    Redirect::temporary(&redirect_url).into_response()
}

#[derive(Deserialize)]
pub struct MetaCallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

#[derive(serde::Deserialize)]
struct TokenResponse {
    access_token: String,
}

pub async fn meta_oauth_callback_handler(
    State(state): State<MetaOAuthState>,
    Query(query): Query<MetaCallbackQuery>,
) -> impl IntoResponse {
    if let Some(err) = query.error {
        tracing::error!("Meta OAuth error: {} - {:?}", err, query.error_description);
        return Redirect::temporary("/integrations?error=oauth_failed").into_response();
    }

    let code = match query.code {
        Some(c) => c,
        None => return Redirect::temporary("/integrations?error=missing_code").into_response(),
    };

    let tenant_id = match query.state {
        Some(t) => t,
        None => return Redirect::temporary("/integrations?error=missing_state").into_response(),
    };

    let client_id = std::env::var("META_CLIENT_ID").unwrap_or_else(|_| "MOCK_META_CLIENT_ID".to_string());
    let client_secret = std::env::var("META_CLIENT_SECRET").unwrap_or_else(|_| "MOCK_META_CLIENT_SECRET".to_string());
    let redirect_uri = std::env::var("META_REDIRECT_URI").unwrap_or_else(|_| "http://localhost:3000/api/v1/oauth/meta/callback".to_string());

    // In tests, we might mock this or skip. Let's do a basic check
    if client_id == "MOCK_META_CLIENT_ID" {
         // Fake success for local dev without real credentials
         let req = ::server_ohc::orchestration::ConnectIntegrationRequest {
             integration_id: "meta".to_string(),
             base_url: "https://graph.facebook.com/v19.0".to_string(),
             bot_token: "".to_string(),
             chat_id: "".to_string(),
             webhook_url: "".to_string(),
             api_token: "MOCK_ACCESS_TOKEN".to_string(),
             from_phone: "MOCK_PHONE_ID".to_string(),
         };
         let _ = state.registry.connect("meta", "https://graph.facebook.com/v19.0", req);
         return Redirect::temporary("/integrations?success=meta_connected").into_response();
    }

    let token_url = format!(
        "https://graph.facebook.com/v19.0/oauth/access_token?client_id={}&redirect_uri={}&client_secret={}&code={}",
        client_id, redirect_uri, client_secret, code
    );

    match state.http_client.get(&token_url).send().await {
        Ok(res) => {
            if let Ok(token_data) = res.json::<TokenResponse>().await {
                // In a real flow, we'd also fetch the WhatsApp Business Account ID and Phone Number ID here
                // For simplicity, we just store the access_token.
                let req = ::server_ohc::orchestration::ConnectIntegrationRequest {
                    integration_id: "meta".to_string(),
                    base_url: "https://graph.facebook.com/v19.0".to_string(),
                    bot_token: "".to_string(),
                    chat_id: "".to_string(),
                    webhook_url: "".to_string(),
                    api_token: token_data.access_token,
                    from_phone: "".to_string(),
                };
                let _ = state.registry.connect("meta", "https://graph.facebook.com/v19.0", req);
                Redirect::temporary("/integrations?success=meta_connected").into_response()
            } else {
                Redirect::temporary("/integrations?error=token_exchange_failed").into_response()
            }
        }
        Err(_) => Redirect::temporary("/integrations?error=network_error").into_response(),
    }
}

pub fn router(registry: Arc<IntegrationsRegistry>) -> Router<std::sync::Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    let state = MetaOAuthState {
        registry,
        http_client: Client::new(),
    };

    Router::new()
        .route("/login", get(meta_oauth_login_handler))
        .route("/callback", get(meta_oauth_callback_handler))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Query;

    #[tokio::test]
    async fn test_meta_login_handler() {
        let query = MetaLoginQuery {
            tenant_id: "tenant123".to_string(),
        };

        let response = meta_oauth_login_handler(Query(query)).await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::TEMPORARY_REDIRECT);

        let headers = response.headers();
        let location = headers.get("location").unwrap().to_str().unwrap();
        assert!(location.contains("facebook.com"));
        assert!(location.contains("state=tenant123"));
        assert!(location.contains("response_type=code"));
    }
}
