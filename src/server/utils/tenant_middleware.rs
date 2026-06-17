use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use serde_json::json;

pub async fn tenant_middleware(req: Request, next: Next) -> Response {
    // Unauthenticated/Whitelisted paths could be ignored here, but typically
    // auth middleware runs first. This middleware runs AFTER auth middleware.
    // Let's assume auth middleware puts Claims in extensions.

    // Some routes are explicitly public, we can whitelist them or just rely on Claims presence
    let path = req.uri().path();
    if path.starts_with("/api/public") || path.starts_with("/api/webhook") {
        return next.run(req).await;
    }

    let is_auth_bypass = path.starts_with("/api/v1/auth") || path.starts_with("/api/v1/webhook") || path.starts_with("/health") || path.starts_with("/metrics");
    if is_auth_bypass {
         return next.run(req).await;
    }

    let tenant_id_opt = req.extensions().get::<::server_common::Claims>()
        .and_then(|c| c.organization_id.clone());

    if let Some(tenant_id) = tenant_id_opt {
        if tenant_id.is_empty() || tenant_id == "system" {
            // For now, allow "system" or empty for backwards compatibility in standalone mode
            // or if it's explicitly needed, but the design doc says reject.
            if ::server_config::get().multitenant {
                 return (
                    StatusCode::FORBIDDEN,
                    axum::Json(json!({
                        "error": "FORBIDDEN",
                        "message": "Invalid or system tenant context."
                    }))
                ).into_response();
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
