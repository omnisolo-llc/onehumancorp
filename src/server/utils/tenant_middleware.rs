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

