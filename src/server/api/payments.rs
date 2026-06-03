use axum::{Json, extract::Extension};
use std::sync::Arc;
use axum::http::HeaderMap;

#[derive(serde::Deserialize, Default)]
pub struct CreateIntentRequest {
    #[serde(default)]
    pub amount_usd: f64,
}

#[derive(serde::Serialize)]
pub struct CreateIntentResponse {
    pub success: bool,
    pub intent_id: Option<String>,
    pub error: Option<String>,
}

#[derive(serde::Serialize)]
pub struct CreateTokenResponse {
    pub success: bool,
    pub token: Option<String>,
    pub error: Option<String>,
}

pub fn router<S: Clone + Send + Sync + 'static>() -> axum::Router<S> {
    axum::Router::new()
        .route("/terminal/token", axum::routing::post(terminal_token_handler))
        .route("/intent", axum::routing::post(create_intent_handler))
}

pub async fn terminal_token_handler(
    _headers: HeaderMap,
    request: axum::extract::Request,
) -> Json<CreateTokenResponse> {
    let auth_info_opt = request.extensions().get::<::server_auth::orchestration::AuthInfo>();
    let tenant_id = match auth_info_opt {
        Some(auth_info) => if auth_info.org_id.is_empty() { "default".to_string() } else { auth_info.org_id.clone() },
        None => return Json(CreateTokenResponse { success: false, token: None, error: Some("Missing authentication context".to_string()) }),
    };

    let stripe_key = match std::env::var("STRIPE_API_KEY") {
        Ok(k) => k,
        Err(_) => return Json(CreateTokenResponse { success: false, token: None, error: Some("STRIPE_API_KEY is required".to_string()) })
    };

    let terminal = crate::integrations::stripe::terminal::StripeTerminal::new(stripe_key);

    match terminal.create_connection_token(&tenant_id).await {
        Ok(token) => Json(CreateTokenResponse { success: true, token: Some(token), error: None }),
        Err(e) => Json(CreateTokenResponse { success: false, token: None, error: Some(e) }),
    }
}

pub async fn create_intent_handler(
    _headers: HeaderMap,
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
    Json(payload): Json<CreateIntentRequest>,
) -> Json<CreateIntentResponse> {
    let tenant_id = if auth_info.org_id.is_empty() {
        "default".to_string()
    } else {
        auth_info.org_id.clone()
    };

    let stripe_key = match std::env::var("STRIPE_API_KEY") {
        Ok(k) => k,
        Err(_) => return Json(CreateIntentResponse { success: false, intent_id: None, error: Some("STRIPE_API_KEY is required".to_string()) })
    };

    let terminal = crate::integrations::stripe::terminal::StripeTerminal::new(stripe_key);

    match terminal.create_payment_intent(&tenant_id, payload.amount_usd).await {
        Ok(intent_id) => Json(CreateIntentResponse { success: true, intent_id: Some(intent_id), error: None }),
        Err(e) => Json(CreateIntentResponse { success: false, intent_id: None, error: Some(e) }),
    }
}
