use axum::{
    extract::State,
    response::IntoResponse,
    Json,
    routing::post,
    Router,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::auth::{Store, User};
use crate::oidc::{validate_oidc_token, OIDCConfig};

#[derive(Deserialize)]
pub struct HandshakeRequest {
    pub token: String,
    pub tenant_id: String,
    pub mode: String,
}

#[derive(Serialize)]
pub struct HandshakeResponse {
    pub token: String,
    pub tenant_id: String,
    pub mode: String,
}

pub async fn handshake_handler(
    State(auth_store): State<Arc<Store>>,
    Json(payload): Json<HandshakeRequest>,
) -> Result<Json<HandshakeResponse>, (StatusCode, String)> {
    let is_development = std::env::var("OHC_AGENT_AUTH_DISABLED").unwrap_or_default() == "true";

    // Check if we are using an ephemeral dev token in dev mode
    if is_development && payload.token.starts_with("dev_") {
         return Ok(Json(HandshakeResponse {
             token: format!("dev_jwt_{}", payload.token),
             tenant_id: payload.tenant_id,
             mode: payload.mode,
         }));
    }

    if payload.token.is_empty() {
        return Err((StatusCode::UNAUTHORIZED, "Missing token".to_string()));
    }

    // Properly validate the OIDC token
    let cfg = OIDCConfig {
        issuer_url: std::env::var("VITE_OHC_AUTH_DOMAIN").unwrap_or_else(|_| "auth.onehumancorp.com".to_string()),
        client_id: "ohc-thin-client".to_string(), // In a real app this might be configurable
        enabled: true,
    };

    // If not in development mode, we validate against the external Identity Provider
    let claims = if !is_development {
        validate_oidc_token(&payload.token, &cfg).await.map_err(|e| (StatusCode::UNAUTHORIZED, e))?
    } else {
        // Mock claims for dev environments without real OIDC setup where OHC_AGENT_AUTH_DISABLED=true but the token doesn't start with dev_
         crate::auth::Claims {
             sub: "dev_user".to_string(),
             username: "dev".to_string(),
             email: "dev@local".to_string(),
             roles: vec!["operator".to_string()],
             organization_id: Some(payload.tenant_id.clone()),
             session_id: None,
             iat: 0,
             exp: 0,
             jti: "dev_jti".to_string(),
         }
    };

    // Prevent IDOR by ensuring the token's organization_id (if present) matches the requested tenant_id
    if let Some(token_org_id) = &claims.organization_id {
        if token_org_id != &payload.tenant_id {
             return Err((StatusCode::FORBIDDEN, "Tenant ID mismatch".to_string()));
        }
    }


    let user = User {
        id: claims.sub,
        username: claims.username,
        email: claims.email,
        password_hash: "".to_string(),
        roles: claims.roles,
        active: true,
        organization_id: Some(payload.tenant_id.clone()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        oidc_subject: None,
    };

    let jwt_token = auth_store.issue_token(&user).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(HandshakeResponse {
        token: jwt_token,
        tenant_id: payload.tenant_id,
        mode: payload.mode,
    }))
}

pub fn router<S: Clone + Send + Sync + 'static>(auth_store: Arc<Store>) -> Router<S> {
    Router::new()
        .route("/handshake", post(handshake_handler))
        .with_state(auth_store)
}
