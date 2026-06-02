use axum::{
    Json,
};
use axum::http::StatusCode;
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn create_connection_token_handler(
    req: axum::extract::Request,
) -> Result<Json<Value>, (StatusCode, String)> {
    let tenant_id = match req.extensions().get::<::server_auth::orchestration::AuthInfo>() {
        Some(auth) => auth.org_id.clone(),
        None => return Err((StatusCode::UNAUTHORIZED, "Missing authentication context".to_string())),
    };

    let client = crate::integrations::stripe::terminal::StripeTerminalClient::new("mock_key".to_string());

    match client.create_connection_token(&tenant_id).await {
        Ok(token) => Ok(Json(json!({ "secret": token.secret }))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}
