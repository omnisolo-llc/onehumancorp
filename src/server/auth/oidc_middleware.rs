use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use axum::http::StatusCode;
use crate::oidc::{validate_oidc_token, OIDCConfig};
use crate::orchestration::AuthInfo;

pub async fn oidc_auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 1. Extract token
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| {
            if s.starts_with("Bearer ") {
                Some(s[7..].to_string())
            } else {
                None
            }
        });

    let token = match auth_header {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // 2. Load config
    // In a real app we'd load this from state or env properly,
    // for this middleware we construct it or pull it from global config.
    let oidc_config = OIDCConfig {
        issuer_url: std::env::var("OIDC_ISSUER_URL").unwrap_or_default(),
        client_id: std::env::var("OIDC_CLIENT_ID").unwrap_or_default(),
        enabled: std::env::var("OIDC_ENABLED").map(|v| v == "true").unwrap_or(true),
    };

    if !oidc_config.enabled {
        // Fall back to guest_auth_middleware logic or just proceed if not strict
        return Ok(next.run(req).await);
    }

    // 3. Validate Token
    let claims = match validate_oidc_token(&token, &oidc_config).await {
        Ok(claims) => claims,
        Err(err) => {
            tracing::warn!("OIDC validation failed: {}", err);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let tenant_id = claims.organization_id.clone().unwrap_or_else(|| "default-tenant".to_string());
    let user_id = claims.sub.clone();

    // 4. Inject into extensions
    req.extensions_mut().insert(claims.clone());

    // Inject AuthInfo which contains the spiffe_id minting
    req.extensions_mut().insert(AuthInfo {
        org_id: tenant_id.clone(),
        agent_id: user_id.clone(),
        spiffe_id: format!("spiffe://onehumancorp.io/tenant/{}/user/{}", tenant_id, user_id),
    });

    Ok(next.run(req).await)
}
