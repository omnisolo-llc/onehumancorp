use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::common::auth_utils::{UiTenantQuery, ui_tenant_id};

#[derive(Serialize, Deserialize)]
pub struct Proposal {
    pub id: Uuid,
    pub tenant_id: String,
    pub customer_id: Option<Uuid>,
    pub status: String,
    pub total_amount_cents: i64,
    pub required_deposit_cents: i64,
    pub checkout_url: Option<String>,
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list_proposals))
}

async fn list_proposals(
    State(pool): State<PgPool>,
    Query(query): Query<UiTenantQuery>,
) -> impl IntoResponse {
    let tenant_id = ui_tenant_id(&query);
    let res = sqlx::query_as!(
        Proposal,
        "SELECT id, tenant_id, customer_id, status, total_amount_cents, required_deposit_cents, checkout_url FROM interactive_proposals WHERE tenant_id = $1",
        tenant_id
    )
    .fetch_all(&pool)
    .await;

    match res {
        Ok(items) => (StatusCode::OK, Json(items)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list interactive proposals: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
