use tonic::{Request, Status};
use crate::pricing::rate_limit::RedisRateLimiter;
use redis::Client;
use std::sync::Arc;
use crate::auth::orchestration::AuthInfo;
use std::sync::OnceLock;

pub static GLOBAL_RATE_LIMITER: OnceLock<Option<Arc<RedisRateLimiter>>> = OnceLock::new();

pub fn get_global_rate_limiter() -> &'static Option<Arc<RedisRateLimiter>> {
    GLOBAL_RATE_LIMITER.get_or_init(|| {
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    if let Ok(client) = Client::open(redis_url) {
        Some(Arc::new(RedisRateLimiter::new(client)))
    } else {
        None
    }
})
}

pub async fn check_grpc_tier_limits(req: &Request<()>) -> Result<(), Status> {
    if let Some(auth_info) = req.extensions().get::<AuthInfo>() {
        let tenant_id = &auth_info.org_id;
        let agent_id = &auth_info.agent_id;

        if let Some(limiter) = get_global_rate_limiter() {
            if !tenant_id.is_empty() {
                let status = limiter.record_action(tenant_id, agent_id).await.map_err(|e| Status::internal(e))?;
                if status.soft_limit_reached {
                    let msg = status.user_message.unwrap_or_else(|| "Tier limit reached. Please upgrade.".to_string());
                    // Return RESOURCE_EXHAUSTED so the frontend knows it's a soft limit to trigger upgrade prompt
                    return Err(Status::resource_exhausted(msg));
                }
            }
        }
    }
    Ok(())
}
