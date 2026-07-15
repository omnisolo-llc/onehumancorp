use crate::db::DB;
use crate::services::customer_memory_graph::service::{
    CustomerMemoryGraphService, CustomerProfileSummary,
};
use axum::{
    Router,
    extract::Extension,
    extract::{Json, Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct CustomerMemoryState {
    pub db: Arc<DB>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IngestEventPayload {
    pub customer_id: String,
    pub channel: String,
    pub raw_content: String,
}

#[derive(Serialize)]
pub struct IngestEventResponse {
    pub event_id: Uuid,
}

pub async fn ingest_event(
    State(state): State<CustomerMemoryState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<IngestEventPayload>,
) -> Result<Json<IngestEventResponse>, StatusCode> {
    let tenant_id = memory_tenant(&claims)?;
    if payload.customer_id.trim().is_empty()
        || payload.customer_id.chars().count() > 200
        || payload.channel.trim().is_empty()
        || payload.channel.chars().count() > 100
        || payload.raw_content.trim().is_empty()
        || payload.raw_content.chars().count() > 16_000
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    let service = CustomerMemoryGraphService::new(state.db.pool.clone());

    match service
        .ingest_interaction(
            &tenant_id,
            &payload.customer_id,
            &payload.channel,
            &payload.raw_content,
        )
        .await
    {
        Ok(event_id) => Ok(Json(IngestEventResponse { event_id })),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to ingest event: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_profile_summary(
    State(state): State<CustomerMemoryState>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(customer_id): Path<String>,
) -> Result<Json<CustomerProfileSummary>, StatusCode> {
    let tenant_id = memory_tenant(&claims)?;
    if customer_id.trim().is_empty() || customer_id.chars().count() > 200 {
        return Err(StatusCode::BAD_REQUEST);
    }
    let service = CustomerMemoryGraphService::new(state.db.pool.clone());

    match service.get_profile_summary(&tenant_id, &customer_id).await {
        Ok(summary) => Ok(Json(summary)),
        Err(e) => {
            tracing::error!("Failed to fetch profile summary: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn memory_tenant(claims: &::server_common::Claims) -> Result<String, StatusCode> {
    claims
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|tenant_id| !tenant_id.is_empty() && !tenant_id.eq_ignore_ascii_case("system"))
        .map(str::to_string)
        .ok_or(StatusCode::UNAUTHORIZED)
}

pub fn router(db: Arc<DB>, auth_store: Arc<::server_auth::Store>) -> Router {
    let state = CustomerMemoryState { db };
    Router::new()
        .route("/ingest", post(ingest_event))
        .route("/summary/{customer_id}", get(get_profile_summary))
        .layer(axum::middleware::from_fn_with_state(
            auth_store,
            ::server_auth::strict_bearer_auth_middleware,
        ))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::ServiceExt;

    #[test]
    fn ingest_contract_rejects_browser_tenant_authority() {
        assert!(
            serde_json::from_value::<IngestEventPayload>(serde_json::json!({
                "tenant_id": "attacker",
                "customer_id": "customer-a",
                "channel": "email",
                "raw_content": "Hello"
            }))
            .is_err()
        );
    }

    #[tokio::test]
    async fn memory_router_requires_a_valid_session_and_exposes_no_browser_job_runner() {
        let pool = sqlx::PgPool::connect_lazy("postgres://localhost/unused").unwrap();
        let db = Arc::new(DB {
            pool,
            store: crate::db::DbStore::Postgres,
        });
        let auth_store = Arc::new(::server_auth::Store::new());
        let now = chrono::Utc::now();
        let token = auth_store
            .issue_token(&::server_auth::User {
                id: "viewer-a".to_string(),
                username: "viewer-a".to_string(),
                email: "viewer-a@example.com".to_string(),
                password_hash: String::new(),
                roles: vec!["VIEWER".to_string()],
                active: true,
                organization_id: Some("tenant-a".to_string()),
                created_at: now,
                updated_at: now,
                oidc_subject: None,
            })
            .unwrap();
        let app = router(db, auth_store);

        let forged = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/summary/customer-a")
                    .header("x-tenant-id", "attacker")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(forged.status(), StatusCode::UNAUTHORIZED);

        let viewer = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/process")
                    .header("authorization", format!("Bearer {token}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(viewer.status(), StatusCode::NOT_FOUND);
    }
}
