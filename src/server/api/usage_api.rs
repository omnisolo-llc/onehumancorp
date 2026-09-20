//! Owner-scoped usage visibility and spending authorization. These endpoints do
//! not accept provider receipts, add payment credit or send customer charges.
use crate::hub::Hub;
use ::server_common::Claims;
use ::server_harness::middleware::usage_ledger::{LedgerError, UsageLedger};
use axum::{
    Json, Router,
    extract::{Extension, Query, State},
    http::StatusCode,
    routing::{get, put},
};
use serde::Deserialize;
use std::sync::Arc;

fn owner_tenant(claims: &Claims) -> Result<&str, StatusCode> {
    if !claims
        .roles
        .iter()
        .any(|role| role.eq_ignore_ascii_case("owner") || role.eq_ignore_ascii_case("admin"))
    {
        return Err(StatusCode::FORBIDDEN);
    }
    claims
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|tenant| !tenant.is_empty())
        .ok_or(StatusCode::UNAUTHORIZED)
}
fn status(error: LedgerError) -> StatusCode {
    match error {
        LedgerError::Invalid => StatusCode::BAD_REQUEST,
        LedgerError::Unconfigured => StatusCode::PRECONDITION_FAILED,
        LedgerError::Limit | LedgerError::Conflict | LedgerError::State => StatusCode::CONFLICT,
        LedgerError::Database => StatusCode::SERVICE_UNAVAILABLE,
    }
}
fn ledger(hub: &Hub) -> Result<UsageLedger, StatusCode> {
    hub.usage_ledger().ok_or(StatusCode::SERVICE_UNAVAILABLE)
}

pub fn router() -> Router<Arc<Hub>> {
    Router::new()
        .route("/usage", get(summary))
        .route("/usage/records", get(records))
        .route("/usage/spending-limit", put(set_limit))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LimitRequest {
    limit_micros: i64,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Page {
    #[serde(default)]
    after: String,
}

async fn set_limit(
    State(hub): State<Arc<Hub>>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<LimitRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let tenant = owner_tenant(&claims)?;
    // A user-facing ceiling, never permission to bypass platform credit policy.
    if !(0..=1_000_000_000_000).contains(&request.limit_micros) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let ledger = ledger(&hub)?;
    ledger.initialize().await.map_err(status)?;
    ledger
        .set_limit(tenant, request.limit_micros)
        .await
        .map_err(status)?;
    Ok(Json(
        serde_json::json!({"spending_authorization":ledger.summary(tenant).await.map_err(status)?,
        "currency":"USD", "unit":"micro_usd", "is_payment_credit":false}),
    ))
}
async fn summary(
    State(hub): State<Arc<Hub>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let tenant = owner_tenant(&claims)?;
    let ledger = ledger(&hub)?;
    Ok(Json(
        serde_json::json!({"account":ledger.summary(tenant).await.map_err(status)?,
        "currency":"USD", "unit":"micro_usd", "scope":"metered_api_paths_only",
        "payment_collection_enabled":false, "customer_direct_inference_is_not_rebilled":true}),
    ))
}
async fn records(
    State(hub): State<Arc<Hub>>,
    Extension(claims): Extension<Claims>,
    Query(page): Query<Page>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let tenant = owner_tenant(&claims)?;
    let entries = ledger(&hub)?
        .records(tenant, &page.after)
        .await
        .map_err(status)?;
    let next = if entries.len() == 100 {
        entries.last().map(|entry| entry.event_id.clone())
    } else {
        None
    };
    Ok(Json(
        serde_json::json!({"records":entries,"next_after":next,"snapshot":false,"limit":100}),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn limits_are_not_provider_receipts_or_cross_tenant_commands() {
        assert!(
            serde_json::from_value::<LimitRequest>(
                serde_json::json!({"limit_micros":100,"tenant_id":"other"})
            )
            .is_err()
        );
        assert!(
            serde_json::from_value::<LimitRequest>(
                serde_json::json!({"limit_micros":100,"provider_cost":-100})
            )
            .is_err()
        );
        assert!(
            serde_json::from_value::<Page>(serde_json::json!({"after":"","tenant_id":"other"}))
                .is_err()
        );
    }
}
