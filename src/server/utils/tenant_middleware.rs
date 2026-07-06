use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use serde_json::json;

fn is_multitenant_mode() -> bool {
    #[cfg(test)]
    {
        if let Ok(val) = std::env::var("OHC_MULTITENANT") {
            return val == "true";
        }
    }
    ::server_config::get().multitenant
}

pub async fn tenant_middleware(req: Request, next: Next) -> Response {
    // Unauthenticated/Whitelisted paths could be ignored here, but typically
    // auth middleware runs first. This middleware runs AFTER auth middleware.
    // Let's assume auth middleware puts Claims in extensions.

    // Some routes are explicitly public, we can whitelist them or just rely on Claims presence
    let path = req.uri().path();
    if path.starts_with("/api/public") || path.starts_with("/api/webhook") {
        return next.run(req).await;
    }

    let is_auth_bypass = path.starts_with("/api/v1/auth") || path.starts_with("/api/onboarding") || path.starts_with("/api/agents/webhook") || path.starts_with("/api/v1/webhook") || path.starts_with("/health") || path.starts_with("/metrics") || path.starts_with("/api/v1/growth/embed") || path.starts_with("/api/dev/");
    if is_auth_bypass {
         return next.run(req).await;
    }

    let tenant_id_opt = req.extensions().get::<::server_common::Claims>()
        .and_then(|c| c.organization_id.clone());

    if let Some(tenant_id) = tenant_id_opt {
        if tenant_id.is_empty() || tenant_id == "system" {
            // For now, allow "system" or empty for backwards compatibility in standalone mode
            // or if it's explicitly needed, but the design doc says reject.
            if is_multitenant_mode() {
                 return (
                    StatusCode::FORBIDDEN,
                    axum::Json(json!({
                        "error": "FORBIDDEN",
                        "message": "Invalid or system tenant context."
                    }))
                ).into_response();
            }
        }

        // Validate query parameters to prevent Tenant Leakage (IDOR)
        if is_multitenant_mode() {
            if let Some(query_str) = req.uri().query() {
                for part in query_str.split('&') {
                    let mut kv = part.splitn(2, '=');
                    if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                        let decoded_k = ::urlencoding::decode(k).unwrap_or(std::borrow::Cow::Borrowed(k));
                        let decoded_v = ::urlencoding::decode(v).unwrap_or(std::borrow::Cow::Borrowed(v));
                        if decoded_k == "tenant_id" || decoded_k == "tenant" {
                            if !decoded_v.trim().is_empty() && decoded_v.trim() != tenant_id {
                                return (
                                    StatusCode::FORBIDDEN,
                                    axum::Json(json!({
                                        "error": "FORBIDDEN",
                                        "message": "Tenant mismatch."
                                    }))
                                ).into_response();
                            }
                        }
                    }
                }
            }
        }

        // Valid context, inject into request if needed, but it's already in Claims.
        // Also ensure immutable context (already done via Claims being immutable).
        return next.run(req).await;
    } else {
        // No claims means no tenant context
        // If it's a route that requires it, fail closed.
        return (
            StatusCode::UNAUTHORIZED,
            axum::Json(json!({
                "error": "UNAUTHORIZED",
                "message": "Missing tenant context."
            }))
        ).into_response();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use tower::ServiceExt; // for `oneshot`
    use serde_json::Value;

    async fn dummy_handler() -> &'static str {
        "ok"
    }

    fn setup_router(_multitenant: bool) -> Router {
        Router::new()
            .route("/api/public/test", get(dummy_handler))
            .route("/api/v1/auth/test", get(dummy_handler))
            .route("/health", get(dummy_handler))
            .route("/api/protected", get(dummy_handler))
            .route("/api/protected_with_query", get(dummy_handler))
            .layer(axum::middleware::from_fn(tenant_middleware))
    }

    #[tokio::test]
    async fn test_public_routes_bypass() {
        let app = setup_router(true);

        let req = Request::builder()
            .uri("/api/public/test")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_auth_bypass() {
        let app = setup_router(true);

        let req = Request::builder()
            .uri("/api/v1/auth/test")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_health_bypass() {
        let app = setup_router(true);

        let req = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_missing_claims_rejected() {
        let app = setup_router(true);

        let req = Request::builder()
            .uri("/api/protected")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(json["error"], "UNAUTHORIZED");
    }

    #[tokio::test]
    async fn test_valid_claims_accepted() {
        let app = setup_router(true);

        let mut req = Request::builder()
            .uri("/api/protected")
            .body(Body::empty())
            .unwrap();

        req.extensions_mut().insert(::server_common::Claims {
            sub: "user_1".to_string(),
            organization_id: Some("tenant_1".to_string()),
            exp: 10000000000,
            iat: 0,
            session_id: Some("1".to_string()),
            roles: vec![],
            username: "test@example.com".to_string(),
            jti: "a".to_string(),
            email: "test@example.com".to_string(),
        });

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_url_encoded_tenant_id_spoofing() {
        temp_env::async_with_vars([("OHC_MULTITENANT", Some("true"))], async {
            let app = setup_router(true);

            // Attempting to spoof `tenant_1` with URL encoding
            let mut req = Request::builder()
                .uri("/api/protected_with_query?%74enant_id=tenant_2")
                .body(Body::empty())
                .unwrap();

            req.extensions_mut().insert(::server_common::Claims {
                sub: "user_1".to_string(),
                organization_id: Some("tenant_1".to_string()),
                exp: 10000000000,
                iat: 0,
                session_id: Some("1".to_string()),
                roles: vec![],
                username: "test@example.com".to_string(),
                jti: "a".to_string(),
                email: "test@example.com".to_string(),
            });

            let response = app.clone().oneshot(req).await.unwrap();
            assert_eq!(response.status(), StatusCode::FORBIDDEN);

            // Another variant
            let mut req2 = Request::builder()
                .uri("/api/protected_with_query?tenant_id=tenant_2%20")
                .body(Body::empty())
                .unwrap();

            req2.extensions_mut().insert(::server_common::Claims {
                sub: "user_1".to_string(),
                organization_id: Some("tenant_1".to_string()),
                exp: 10000000000,
                iat: 0,
                session_id: Some("1".to_string()),
                roles: vec![],
                username: "test@example.com".to_string(),
                jti: "a".to_string(),
                email: "test@example.com".to_string(),
            });

            let response2 = app.clone().oneshot(req2).await.unwrap();
            assert_eq!(response2.status(), StatusCode::FORBIDDEN);
        }).await;
    }
}
