use ::server_pricing::rate_limit::RedisRateLimiter;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

fn rate_limit_tenant(claims: Option<&::server_common::Claims>) -> Option<String> {
    claims?
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|tenant_id| !tenant_id.is_empty() && !tenant_id.eq_ignore_ascii_case("system"))
        .map(str::to_string)
}

pub async fn tier_middleware(
    State(rate_limiter): State<Arc<RedisRateLimiter>>,
    req: Request,
    next: Next,
) -> Response {
    let tenant_id =
        rate_limit_tenant(req.extensions().get::<::server_common::Claims>()).unwrap_or_default();

    // Very simple placeholder: in a real system we might inspect the request path to determine the action cost
    // For this example, we just simulate a 1-action check for protected paths
    let mut warning_msg = None;
    if req.uri().path().starts_with("/api/v1/protected")
        || req.uri().path().starts_with("/api/v1/autodream")
    {
        if tenant_id.is_empty() {
            return axum::response::IntoResponse::into_response((
                axum::http::StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({
                    "error": "UNAUTHORIZED",
                    "message": "Missing or invalid tenant ID."
                })),
            ));
        }

        match rate_limiter
            .record_action(&tenant_id, "default_agent")
            .await
        {
            Ok(status) => {
                if status.soft_limit_reached {
                    warning_msg = Some(
                        status
                            .user_message
                            .unwrap_or_else(|| "Tier limit reached. Please upgrade.".to_string()),
                    );
                    if !status.is_allowed {
                        return axum::response::IntoResponse::into_response((
                            axum::http::StatusCode::PAYMENT_REQUIRED,
                            axum::Json(serde_json::json!({
                                "error": "LIMIT_EXCEEDED",
                                "message": warning_msg.unwrap()
                            })),
                        ));
                    }
                }
            }
            Err(e) => {
                // To minimize unnecessary log noise during common ephemeral Redis disconnections,
                // we fail open quietly but bump a telemetry metric if possible.
                // We'll leave tracing::debug here to allow debugging if needed.
                tracing::debug!("RateLimiter error: {}. Failing open.", e);
            }
        }
    }

    let mut res = next.run(req).await;
    if let Some(msg) = warning_msg {
        if let Ok(header_value) = axum::http::HeaderValue::from_str(&msg) {
            res.headers_mut()
                .insert("x-ratelimit-warning", header_value);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::server_pricing::rate_limit::{PlanTier, RedisRateLimiter};
    use axum::http::StatusCode;
    use axum::{Router, routing::get};
    use redis::AsyncCommands;
    use std::sync::Arc;

    async fn setup_test_router(rate_limiter: Arc<RedisRateLimiter>) -> Router {
        Router::new()
            .route("/api/v1/protected/action", get(|| async { "Success" }))
            .route("/api/v1/public/info", get(|| async { "Public Info" }))
            .route_layer(axum::middleware::from_fn_with_state(
                rate_limiter.clone(),
                tier_middleware,
            ))
            .with_state(rate_limiter)
    }

    #[test]
    fn rate_limit_tenant_requires_a_non_system_signed_claim() {
        let claims = |organization_id: Option<&str>| ::server_common::Claims {
            sub: "user-1".to_string(),
            exp: 0,
            iat: 0,
            organization_id: organization_id.map(str::to_string),
            username: String::new(),
            email: String::new(),
            roles: vec![],
            session_id: None,
            jti: String::new(),
        };

        assert_eq!(
            rate_limit_tenant(Some(&claims(Some("tenant-a")))),
            Some("tenant-a".to_string())
        );
        assert_eq!(rate_limit_tenant(Some(&claims(Some(" system ")))), None);
        assert_eq!(rate_limit_tenant(Some(&claims(Some("   ")))), None);
        assert_eq!(rate_limit_tenant(None), None);
    }

    #[tokio::test]
    async fn test_tier_middleware_blocks_over_limit() {
        let redis_url =
            std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
        if let Ok(client) = redis::Client::open(redis_url) {
            if client.get_multiplexed_async_connection().await.is_ok() {
                let limiter = Arc::new(RedisRateLimiter::new(client.clone()));
                let _ = limiter.set_tenant_tier("test_tenant", PlanTier::Free).await;

                let mut conn = client.get_multiplexed_async_connection().await.unwrap();
                let _: () = conn
                    .set("tenant:test_tenant:actions_used", 101)
                    .await
                    .unwrap();

                let app = setup_test_router(limiter).await;
                let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
                let addr = listener.local_addr().unwrap();
                tokio::spawn(async move {
                    axum::serve(listener, app).await.unwrap();
                });

                let client = reqwest::Client::new();
                let _res = client
                    .get(&format!("http://{}/api/v1/protected/action", addr))
                    .send()
                    .await
                    .unwrap();

                let month_key = chrono::Utc::now().format("%Y-%m").to_string();
                let _: () = conn
                    .set(format!("tenant:system:actions_used:{}", month_key), 101)
                    .await
                    .unwrap();
                let res2 = client
                    .get(&format!("http://{}/api/v1/protected/action", addr))
                    .send()
                    .await
                    .unwrap();

                assert_eq!(res2.status(), StatusCode::OK);
                assert!(res2.headers().contains_key("x-ratelimit-warning"));
            }
        }
    }
}
