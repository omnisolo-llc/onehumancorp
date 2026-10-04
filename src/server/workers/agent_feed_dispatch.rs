//! Durable single-attempt fence for accepted feed work. Pre-attempt storage
//! failures can retry; an attempted/unknown operation never automatically does.
use crate::domain::agent_feed_decisions::{DispatchRecord, lock_action, receipt};
use crate::orchestration::queue::omnisolo_job_queue::OmniSoloJob;
use serde_json::Value;
use sqlx::{PgConnection, PgPool, types::Json};

pub struct Attempt {
    pub tenant_id: String,
    pub action_id: String,
    pub job_id: String,
    pub payload: Value,
}
pub(super) async fn status(
    conn: &mut PgConnection,
    record: &DispatchRecord,
    state: &str,
    detail: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE agent_feed_decisions SET dispatch_status=$1,detail=$2,updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$3 AND action_id=$4").bind(state).bind(detail).bind(&record.tenant_id).bind(&record.action_id).execute(&mut *conn).await?;
    sqlx::query("UPDATE ohc_job_queue SET status=$1,updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$2 AND id=$3").bind(state).bind(&record.tenant_id).bind(&record.job_id).execute(conn).await?;
    Ok(())
}
pub async fn claim(pool: &PgPool, job: &OmniSoloJob) -> Result<Option<Attempt>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    server_common::auth_utils::set_org_context(&mut *tx, &job.tenant_id).await?;
    sqlx::query("SET LOCAL lock_timeout='1000ms'")
        .execute(&mut *tx)
        .await?;
    sqlx::query("SET LOCAL statement_timeout='3000ms'")
        .execute(&mut *tx)
        .await?;
    let initial: Option<DispatchRecord> =
        sqlx::query_as("SELECT * FROM agent_feed_decisions WHERE tenant_id=$1 AND job_id=$2")
            .bind(&job.tenant_id)
            .bind(&job.id)
            .fetch_optional(&mut *tx)
            .await?;
    let Some(initial) = initial else {
        // Pre-migration or arbitrary queue entries have no bound owner decision.
        sqlx::query("UPDATE ohc_job_queue SET status='RECONCILIATION_REQUIRED',updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$1 AND id=$2 AND job_type='agent_feed_action' AND status IN ('PENDING','PROCESSING')").bind(&job.tenant_id).bind(&job.id).execute(&mut *tx).await?;
        tx.commit().await?;
        return Ok(None);
    };
    // Match owner-lock ordering with the request's canonical owner transaction.
    let owner:Option<String>=sqlx::query_scalar("SELECT u.id FROM users u JOIN identity_user_roles r ON r.user_id=u.id AND r.tenant_id=u.tenant_id WHERE u.id=$1 AND u.tenant_id=$2 AND u.active=TRUE AND pg_catalog.translate(r.role_name,'ABCDEFGHIJKLMNOPQRSTUVWXYZ','abcdefghijklmnopqrstuvwxyz') IN ('admin','owner') FOR SHARE OF u,r").bind(&initial.actor_id).bind(&job.tenant_id).fetch_optional(&mut *tx).await?;
    lock_action(&mut tx, &job.tenant_id, &initial.action_id).await?;
    let item: Option<String> = sqlx::query_scalar(
        "SELECT lifecycle_state FROM agent_feed_items WHERE tenant_id=$1 AND id=$2 FOR UPDATE",
    )
    .bind(&job.tenant_id)
    .bind(&initial.action_id)
    .fetch_optional(&mut *tx)
    .await?;
    let Some(record) = receipt(&mut tx, &job.tenant_id, &initial.action_id).await? else {
        return Err(sqlx::Error::RowNotFound);
    };
    let queued: Option<(String, String, Json<Value>)> = sqlx::query_as(
        "SELECT status,job_type,payload FROM ohc_job_queue WHERE tenant_id=$1 AND id=$2 FOR UPDATE",
    )
    .bind(&job.tenant_id)
    .bind(&job.id)
    .fetch_optional(&mut *tx)
    .await?;
    let Some((queue_state, kind, payload)) = queued else {
        return Err(sqlx::Error::RowNotFound);
    };
    if record.attempted_at.is_some()
        || matches!(
            record.dispatch_status.as_str(),
            "DISPATCH_RETURNED" | "RECONCILIATION_REQUIRED"
        )
    {
        if matches!(
            record.dispatch_status.as_str(),
            "DISPATCH_RETURNED" | "RECONCILIATION_REQUIRED"
        ) {
            sqlx::query("UPDATE ohc_job_queue SET status=$1,updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$2 AND id=$3 AND job_type='agent_feed_action' AND status IN ('PENDING','PROCESSING')")
                .bind(&record.dispatch_status).bind(&job.tenant_id).bind(&job.id).execute(&mut *tx).await?;
        }
        tx.commit().await?;
        return Ok(None);
    }
    if record.job_id.as_deref() != Some(job.id.as_str())
        || kind != "agent_feed_action"
        || record.dispatch_payload.as_ref() != Some(&payload)
    {
        status(
            &mut tx,
            &record,
            "RECONCILIATION_REQUIRED",
            "Queue admission does not match the durable approved action",
        )
        .await?;
        tx.commit().await?;
        return Ok(None);
    }
    if record.actor_id != initial.actor_id {
        return Err(sqlx::Error::Protocol(
            "Decision authority changed before dispatch claim".into(),
        ));
    }
    // An approval is durable beyond bearer expiry. Explicit token revocation
    // and the current canonical owner role still prohibit a not-yet-started job.
    sqlx::query("SELECT pg_catalog.pg_advisory_xact_lock_shared(pg_catalog.hashtextextended(pg_catalog.jsonb_build_array('ohc-token-fence-v1',$1::text,$2::text)::text,0))").bind(&job.tenant_id).bind(&record.token_id).execute(&mut *tx).await?;
    let revoked: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM auth_revoked_tokens WHERE tenant_id=$1 AND jti=$2)",
    )
    .bind(&job.tenant_id)
    .bind(&record.token_id)
    .fetch_one(&mut *tx)
    .await?;
    if owner.is_none()
        || revoked
        || record.decision_state != "APPROVED"
        || item.as_deref() != Some("APPROVED")
        || record.dispatch_status == "CANCELLED"
    {
        status(
            &mut tx,
            &record,
            "CANCELLED",
            "Approval or current owner authority was revoked before dispatch",
        )
        .await?;
        tx.commit().await?;
        return Ok(None);
    }
    if queue_state != "PROCESSING" || record.dispatch_status != "PENDING" {
        tx.commit().await?;
        return Ok(None);
    }
    let action = payload.0;
    let valid = action.get("action_id").and_then(Value::as_str) == Some(record.action_id.as_str())
        && action.get("tenant_id").and_then(Value::as_str) == Some(job.tenant_id.as_str())
        && action.get("payload").is_some_and(|v| !v.is_null())
        && (action.get("is_incident").and_then(Value::as_bool) == Some(true)
            || action.get("feature_type").and_then(Value::as_str).is_some());
    if !valid {
        status(
            &mut tx,
            &record,
            "RECONCILIATION_REQUIRED",
            "Malformed durable dispatch payload; no execution attempted",
        )
        .await?;
        tx.commit().await?;
        return Ok(None);
    }
    sqlx::query("UPDATE agent_feed_decisions SET dispatch_status='ATTEMPTING',attempted_at=CURRENT_TIMESTAMP,detail=NULL,updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$1 AND action_id=$2 AND attempted_at IS NULL").bind(&job.tenant_id).bind(&record.action_id).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(Some(Attempt {
        tenant_id: job.tenant_id.clone(),
        action_id: record.action_id,
        job_id: job.id.clone(),
        payload: action,
    }))
}
pub async fn finish(pool: &PgPool, attempt: &Attempt, returned: bool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    server_common::auth_utils::set_org_context(&mut *tx, &attempt.tenant_id).await?;
    lock_action(&mut tx, &attempt.tenant_id, &attempt.action_id).await?;
    let record = receipt(&mut tx, &attempt.tenant_id, &attempt.action_id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;
    if record.job_id.as_deref() != Some(attempt.job_id.as_str()) || record.attempted_at.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }
    if record.dispatch_status == "DISPATCH_RETURNED" {
        status(
            &mut tx,
            &record,
            "DISPATCH_RETURNED",
            record
                .detail
                .as_deref()
                .unwrap_or("Handler returned; external execution or delivery is not verified"),
        )
        .await?;
        return tx.commit().await;
    }
    let (state, detail) = if returned {
        (
            "DISPATCH_RETURNED",
            "Handler returned; external execution or delivery is not verified",
        )
    } else {
        (
            "RECONCILIATION_REQUIRED",
            "An attempted dispatch failed or timed out; reconcile before retrying",
        )
    };
    status(&mut tx, &record, state, detail).await?;
    if returned {
        sqlx::query("UPDATE agent_feed_decisions SET dispatch_returned_at=CURRENT_TIMESTAMP WHERE tenant_id=$1 AND action_id=$2").bind(&attempt.tenant_id).bind(&attempt.action_id).execute(&mut *tx).await?;
    }
    tx.commit().await
}
/// Reconcile the claim's durable outcome before choosing a safe pre-attempt
/// retry. A lost claim/finish COMMIT acknowledgement can never cause dispatch.
pub async fn defer_or_hold(pool: &PgPool, job: &OmniSoloJob) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    server_common::auth_utils::set_org_context(&mut *tx, &job.tenant_id).await?;
    let initial: Option<DispatchRecord> =
        sqlx::query_as("SELECT * FROM agent_feed_decisions WHERE tenant_id=$1 AND job_id=$2")
            .bind(&job.tenant_id)
            .bind(&job.id)
            .fetch_optional(&mut *tx)
            .await?;
    if let Some(initial) = initial {
        lock_action(&mut tx, &job.tenant_id, &initial.action_id).await?;
        let record = receipt(&mut tx, &job.tenant_id, &initial.action_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        if record.attempted_at.is_some() {
            if record.dispatch_status != "DISPATCH_RETURNED" {
                status(
                    &mut tx,
                    &record,
                    "RECONCILIATION_REQUIRED",
                    "Dispatch acknowledgement is unconfirmed; reconcile before retrying",
                )
                .await?;
            }
        } else if record.dispatch_status == "PENDING" {
            sqlx::query("UPDATE ohc_job_queue SET status='PENDING',retry_count=retry_count+1,next_retry_at=CURRENT_TIMESTAMP + INTERVAL '5 seconds',updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$1 AND id=$2 AND status='PROCESSING'").bind(&job.tenant_id).bind(&job.id).execute(&mut *tx).await?;
        }
    }
    tx.commit().await
}
/// Restart recovery changes only observation state, never submits work again.
pub async fn recover_abandoned(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    server_common::auth_utils::set_system_context(&mut *tx).await?;
    sqlx::query("UPDATE agent_feed_decisions SET dispatch_status='RECONCILIATION_REQUIRED',detail='Dispatch worker stopped without a durable return acknowledgement; reconcile before retrying',updated_at=CURRENT_TIMESTAMP WHERE dispatch_status='ATTEMPTING' AND attempted_at<CURRENT_TIMESTAMP-INTERVAL '65 seconds'").execute(&mut *tx).await?;
    sqlx::query("UPDATE agent_feed_decisions d SET dispatch_status='RECONCILIATION_REQUIRED',detail='Queue admission ended or became unavailable without a matching durable attempt; reconcile before retrying',updated_at=CURRENT_TIMESTAMP WHERE d.dispatch_status='PENDING' AND d.attempted_at IS NULL AND NOT EXISTS(SELECT 1 FROM ohc_job_queue q WHERE q.id=d.job_id AND q.tenant_id=d.tenant_id AND q.job_type='agent_feed_action' AND q.status IN ('PENDING','PROCESSING') AND q.payload IS NOT DISTINCT FROM d.dispatch_payload)").execute(&mut *tx).await?;
    sqlx::query("UPDATE ohc_job_queue q SET status='RECONCILIATION_REQUIRED',updated_at=CURRENT_TIMESTAMP FROM agent_feed_decisions d WHERE q.id=d.job_id AND q.tenant_id=d.tenant_id AND d.dispatch_status='RECONCILIATION_REQUIRED' AND q.status IN ('PROCESSING','PENDING')").execute(&mut *tx).await?;
    tx.commit().await
}
