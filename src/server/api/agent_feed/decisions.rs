//! Authenticated owner decisions. No provider work happens in an HTTP request.
use crate::domain::agent_feed_decisions::{self as decisions, Error};
use axum::{
    Json, Router,
    extract::{Extension, Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, put},
};
pub use decisions::DecisionInput as UpdateStateRequest;
use server_auth::commit_authority::{
    AuthorityError, AuthorizedPgOwner, CanonicalPgAuthority, canonical_pg_data_pool,
};
use server_common::Claims;
use sqlx::PgPool;
use std::sync::Arc;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/api/v1/agent-feed/{id}", put(update_feed_item_state))
        .route("/api/v1/agent-feed/{id}/state", put(update_feed_item_state))
        .route("/api/v1/agent-feed/{id}/decision", get(read_decision))
}
async fn authorize(
    pool: &PgPool,
    store: Option<Extension<Arc<server_auth::Store>>>,
    claims: &Claims,
    headers: &HeaderMap,
) -> Result<AuthorizedPgOwner, Error> {
    let store = store
        .ok_or(Error::Authority(AuthorityError::Unavailable))?
        .0;
    let expected_user = headers
        .get_all("x-ohc-expected-user")
        .iter()
        .collect::<Vec<_>>();
    let expected_tenant = headers
        .get_all("x-ohc-expected-tenant")
        .iter()
        .collect::<Vec<_>>();
    if (!expected_user.is_empty() || !expected_tenant.is_empty())
        && (expected_user.len() != 1
            || expected_tenant.len() != 1
            || expected_user[0].to_str().ok() != Some(claims.sub.as_str())
            || expected_tenant[0].to_str().ok() != claims.organization_id.as_deref())
    {
        return Err(Error::Conflict(
            "The session no longer matches the expected owner",
        ));
    }
    let repository = store
        .portable_repo()
        .ok_or(Error::Authority(AuthorityError::Unavailable))?;
    let pool = canonical_pg_data_pool(
        repository.connection(),
        pool,
        &[
            "agent_feed_items",
            "agent_feed",
            "agent_approvals",
            "agent_action_requests",
            "omni_inbox_messages",
            "orders",
            "invoices",
            "ohc_job_queue",
            "agent_feed_decisions",
        ],
    )
    .await?;
    Ok(CanonicalPgAuthority::bind(store, &pool)?
        .authorize(claims, headers)
        .await?)
}
fn failure(error: Error) -> Response {
    let (status, message) = match error {
        Error::Authority(AuthorityError::Forbidden) => (
            StatusCode::FORBIDDEN,
            "Current canonical owner authority is required",
        ),
        Error::NotFound => (StatusCode::NOT_FOUND, "Feed item not found"),
        Error::Invalid(message) => (StatusCode::BAD_REQUEST, message),
        Error::Conflict(message) => (StatusCode::CONFLICT, message),
        error => {
            tracing::warn!(?error, "Agent-feed decision persistence unconfirmed");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Decision outcome is unconfirmed. Read the recorded decision before retrying.",
            )
        }
    };
    (
        status,
        [("cache-control", "no-store")],
        Json(serde_json::json!({"error":message,"success":false})),
    )
        .into_response()
}
async fn update_feed_item_state(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    store: Option<Extension<Arc<server_auth::Store>>>,
    claims: Option<Extension<Claims>>,
    headers: HeaderMap,
    Json(payload): Json<UpdateStateRequest>,
) -> Response {
    let Some(Extension(claims)) = claims else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let owner = match authorize(&pool, store, &claims, &headers).await {
        Ok(owner) => owner,
        Err(e) => return failure(e),
    };
    match decisions::record(owner, &claims.jti, &id, payload).await {
        Ok(response) => {
            crate::invalidate_agent_feed_caches(&response.item.tenant_id).await;
            // Notification is best effort after durable admission. It is not a
            // queue receipt or acknowledgement of external work/delivery.
            if let Some(client) = crate::get_redis_client() {
                let tenant = response.item.tenant_id.clone();
                let event=serde_json::json!({"event_type":"approval_decision","data":{"request_id":id,"status":response.item.lifecycle_state,"department":response.item.event_source,"dispatch":response.dispatch}}).to_string();
                tokio::spawn(async move {
                    if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                        let _: Result<(), redis::RedisError> = redis::cmd("PUBLISH")
                            .arg(format!("agent_feed:{tenant}"))
                            .arg(event)
                            .query_async(&mut conn)
                            .await;
                    }
                });
            }
            (
                StatusCode::OK,
                [("cache-control", "no-store")],
                Json(response),
            )
                .into_response()
        }
        Err(error) => failure(error),
    }
}
async fn read_decision(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    store: Option<Extension<Arc<server_auth::Store>>>,
    claims: Option<Extension<Claims>>,
    headers: HeaderMap,
) -> Response {
    let Some(Extension(claims)) = claims else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let owner = match authorize(&pool, store, &claims, &headers).await {
        Ok(owner) => owner,
        Err(e) => return failure(e),
    };
    match decisions::read(owner, &id).await {
        Ok(response) => (
            StatusCode::OK,
            [("cache-control", "no-store")],
            Json(response),
        )
            .into_response(),
        Err(error) => failure(error),
    }
}
