//! Inbox effects require a canonical action identity supplied by the admitted worker.
use serde_json::Value;
use sqlx::PgPool;

pub async fn handle_approved_inbox_action(
    tenant_id: &str,
    action_id: &str,
    job_id: &str,
    admitted_payload: &Value,
    pool: &PgPool,
) -> Result<(), String> {
    use crate::orchestration::departments::message_delivery::{self, Store};
    message_delivery::dispatch_admitted(
        &Store::Postgres(pool.clone()),
        tenant_id,
        action_id,
        job_id,
        admitted_payload,
    )
    .await
    .map_err(|error| error.to_string())?
    .require_acceptance()
}

/// Retain the legacy interface without inventing an approval from payload fields.
/// The admitted feed worker calls handle_approved_inbox_action with its own ID.
pub async fn handle_inbox_action(
    _tenant_id: &str,
    _payload: &Value,
    _pool: &PgPool,
) -> Result<(), sqlx::Error> {
    Err(sqlx::Error::Configuration(
        "Canonical action identity and persisted review are required for inbox delivery".into(),
    ))
}
