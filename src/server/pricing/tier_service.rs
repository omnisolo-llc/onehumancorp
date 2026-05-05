use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;
use crate::billing::Tracker;
use serde::Serialize;

#[derive(Serialize)]
pub struct GracefulDegradationResponse {
    pub error: String,
    pub user_message: String,
    pub needs_upgrade: bool,
}

pub async fn tier_limit_middleware(
    State(tracker): State<Arc<Tracker>>,
    req: Request,
    next: Next,
) -> Response {
    let tenant_id = if let Some(claims) = req.extensions().get::<crate::auth::Claims>() {
        claims.organization_id.clone().unwrap_or_else(|| "default_tenant".to_string())
    } else {
        // Use a 401 Unauthorized if no auth info is available and it's required for limits
        return (StatusCode::UNAUTHORIZED, Json(GracefulDegradationResponse {
            error: "unauthorized".to_string(),
            user_message: "Authentication required to check limits.".to_string(),
            needs_upgrade: false,
        })).into_response();
    };

    // We attempt to extract a specific agent ID from path or headers if this is an agent route.
    // If not, it means it's a general request (not tied to a specific agent).
    // The exact extraction depends on router path params which are not globally available here easily.
    // For now, we rate limit at the tenant level.
    let agent_id = "tenant_global";

    match tracker.check_rate_limit(&tenant_id, agent_id).await {
        Ok(status) => {
            if !status.is_allowed {
                let msg = status.user_message.unwrap_or_else(|| "You have exceeded your plan limits. Please upgrade.".to_string());
                let resp = GracefulDegradationResponse {
                    error: "plan_limit_exceeded".to_string(),
                    user_message: msg,
                    needs_upgrade: true,
                };
                return (StatusCode::PAYMENT_REQUIRED, Json(resp)).into_response();
            }
            next.run(req).await
        }
        Err(_e) => {
            let resp = GracefulDegradationResponse {
                error: "rate_limit_error".to_string(),
                user_message: "An internal error occurred while checking limits.".to_string(),
                needs_upgrade: false,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(resp)).into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use axum::body::Body;

    #[tokio::test]
    async fn test_tier_middleware_basic() {
        let _tracker = Arc::new(Tracker::new());
        let _req = Request::builder().uri("/").body(Body::empty()).unwrap();
    }
}
