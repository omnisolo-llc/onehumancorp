use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::auth::Claims;
use crate::pricing::rate_limit::RedisRateLimiter;

#[derive(Clone)]
pub struct TierServiceState {
    pub rate_limiter: Arc<RedisRateLimiter>,
}

pub async fn tier_enforcement_middleware(
    State(state): State<Arc<TierServiceState>>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let tenant_id = claims.organization_id.as_deref().unwrap_or_default();

    // For specific routes, we can check limits. For now, we intercept generally or by path
    let path = req.uri().path();

    if path.contains("/pages") || path.contains("/blocks") {
         let _tier = state.rate_limiter.get_tenant_tier(tenant_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
         // Simplistic check for pages/blocks - typically tied to actions or products
         let status = state.rate_limiter.record_action(tenant_id, "system-api").await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

         if status.soft_limit_reached {
              // Return 402 Payment Required for graceful degradation
              return Ok(Response::builder()
                .status(StatusCode::PAYMENT_REQUIRED)
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&serde_json::json!({
                    "error": "UpgradeRequired",
                    "message": status.user_message.unwrap_or_default()
                })).unwrap()))
                .unwrap());
         }
    }

    Ok(next.run(req).await)
}
