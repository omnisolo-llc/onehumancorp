//! Transactional local fulfillment of durably accepted catalog work. This is
//! not a new bearer request: natural bearer expiry does not revoke admission.
use super::agent_feed_dispatch::{Attempt, status};
use crate::domain::agent_feed_decisions::{DispatchRecord, lock_action, receipt};
use serde_json::Value;
use server_auth::commit_authority::{AuthorityError, canonical_pg_data_pool};
use sqlx::{PgConnection, PgPool, types::Json};

pub struct CanonicalCatalogDispatch {
    pool: PgPool,
}
impl CanonicalCatalogDispatch {
    pub async fn bind(store: &server_auth::Store, data: &PgPool) -> Result<Self, AuthorityError> {
        let repository = store.portable_repo().ok_or(AuthorityError::Unavailable)?;
        let pool = canonical_pg_data_pool(
            repository.connection(),
            data,
            &[
                "products",
                "agent_feed_items",
                "agent_feed_decisions",
                "ohc_job_queue",
            ],
        )
        .await?;
        Ok(Self { pool })
    }

    pub async fn execute(&self, attempt: &Attempt) -> Result<(), AuthorityError> {
        let tenant = &attempt.tenant_id;
        if tenant.trim().is_empty()
            || tenant.trim() != tenant
            || tenant.eq_ignore_ascii_case("system")
        {
            return Err(AuthorityError::Forbidden);
        }
        let mut tx = self.pool.begin().await?;
        server_common::auth_utils::set_org_context(&mut *tx, tenant).await?;
        sqlx::query("SET LOCAL lock_timeout='1000ms'")
            .execute(&mut *tx)
            .await?;
        sqlx::query("SET LOCAL statement_timeout='3000ms'")
            .execute(&mut *tx)
            .await?;
        let isolation: String = sqlx::query_scalar("SHOW transaction_isolation")
            .fetch_one(&mut *tx)
            .await?;
        if isolation != "read committed" {
            return Err(AuthorityError::Unavailable);
        }
        let initial: DispatchRecord = sqlx::query_as(
            "SELECT * FROM agent_feed_decisions WHERE tenant_id=$1 AND action_id=$2",
        )
        .bind(tenant)
        .bind(&attempt.action_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(AuthorityError::Forbidden)?;
        // Keep the request/claim lock ordering. Row locks retain the current
        // canonical role through the actual product commit.
        check_owner(&mut tx, &initial).await?;
        lock_action(&mut tx, tenant, &attempt.action_id).await?;
        let item: Option<String> = sqlx::query_scalar(
            "SELECT lifecycle_state FROM agent_feed_items WHERE tenant_id=$1 AND id=$2 FOR UPDATE",
        )
        .bind(tenant)
        .bind(&attempt.action_id)
        .fetch_optional(&mut *tx)
        .await?;
        let record = receipt(&mut tx, tenant, &attempt.action_id)
            .await?
            .ok_or(AuthorityError::Forbidden)?;
        let queued: Option<(String, String, Json<Value>)> = sqlx::query_as("SELECT status,job_type,payload FROM ohc_job_queue WHERE tenant_id=$1 AND id=$2 FOR UPDATE")
            .bind(tenant).bind(&attempt.job_id).fetch_optional(&mut *tx).await?;
        let Some((queue_state, kind, payload)) = queued else {
            return Err(AuthorityError::Forbidden);
        };
        if record.actor_id != initial.actor_id
            || record.token_id != initial.token_id
            || record.job_id.as_deref() != Some(attempt.job_id.as_str())
            || record.dispatch_payload.as_ref() != Some(&payload)
            || payload.0 != attempt.payload
            || kind != "agent_feed_action"
            || record.attempted_at.is_none()
        {
            return Err(AuthorityError::Forbidden);
        }
        if record.dispatch_status == "DISPATCH_RETURNED" {
            // A committed local outcome is immutable evidence, never a new insert.
            return Ok(());
        }
        if record.dispatch_status != "ATTEMPTING"
            || queue_state != "PROCESSING"
            || record.decision_state != "APPROVED"
            || item.as_deref() != Some("APPROVED")
            || payload.get("tenant_id").and_then(Value::as_str) != Some(tenant.as_str())
            || payload.get("action_id").and_then(Value::as_str) != Some(attempt.action_id.as_str())
            || payload.get("feature_type").and_then(Value::as_str) != Some("create_product")
            || payload.get("is_incident").and_then(Value::as_bool) != Some(false)
        {
            return Err(AuthorityError::Forbidden);
        }
        check_revocation(&mut tx, &record).await?;
        let product =
            crate::domain::catalog::create_product_on(tenant, &payload["payload"], &mut *tx)
                .await?;
        status(
            &mut tx,
            &record,
            "DISPATCH_RETURNED",
            &format!("Product {product} committed to the tenant catalog; external execution or delivery is not verified"),
        )
        .await?;
        sqlx::query("UPDATE agent_feed_decisions SET dispatch_returned_at=CURRENT_TIMESTAMP WHERE tenant_id=$1 AND action_id=$2")
            .bind(tenant).bind(&attempt.action_id).execute(&mut *tx).await?;
        // Revocation can commit while local writes are pending. Take the same
        // shared token fence as canonical owner writes only at final commit,
        // then use a fresh READ COMMITTED snapshot. Direct canonical revocation
        // writers participate through the existing database trigger.
        sqlx::query("SELECT pg_catalog.pg_advisory_xact_lock_shared(pg_catalog.hashtextextended(pg_catalog.jsonb_build_array('ohc-token-fence-v1',$1::text,$2::text)::text,0))")
            .bind(tenant).bind(&record.token_id).execute(&mut *tx).await?;
        check_owner(&mut tx, &record).await?;
        check_revocation(&mut tx, &record).await?;
        tx.commit().await?;
        Ok(())
    }
}

async fn check_owner(
    conn: &mut PgConnection,
    record: &DispatchRecord,
) -> Result<(), AuthorityError> {
    let owner: Option<String> = sqlx::query_scalar("SELECT u.id FROM users u JOIN identity_user_roles r ON r.user_id=u.id AND r.tenant_id=u.tenant_id WHERE u.id=$1 AND u.tenant_id=$2 AND u.active=TRUE AND pg_catalog.translate(r.role_name,'ABCDEFGHIJKLMNOPQRSTUVWXYZ','abcdefghijklmnopqrstuvwxyz') IN ('admin','owner') FOR SHARE OF u,r")
        .bind(&record.actor_id).bind(&record.tenant_id).fetch_optional(conn).await?;
    if owner.is_none() {
        return Err(AuthorityError::Forbidden);
    }
    Ok(())
}
async fn check_revocation(
    conn: &mut PgConnection,
    record: &DispatchRecord,
) -> Result<(), AuthorityError> {
    let revoked: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM auth_revoked_tokens WHERE tenant_id=$1 AND jti=$2)",
    )
    .bind(&record.tenant_id)
    .bind(&record.token_id)
    .fetch_one(conn)
    .await?;
    if revoked {
        return Err(AuthorityError::Forbidden);
    }
    Ok(())
}
