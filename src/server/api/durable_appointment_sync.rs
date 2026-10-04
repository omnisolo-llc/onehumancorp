//! Appointment receipts and effects share a current canonical owner transaction.
use super::super::super::sync_transaction::commit_owner;
use super::{Claim, EVENTS_ROUTE, Outcome, SyncError, block, claim, finish, task, valid_id};
use crate::api::field_ops::records::{FieldAccess, check_transition};
use chrono::{DateTime, Utc};
use serde_json::{Value, json};
use server_auth::commit_authority::AuthorityError;

fn timestamp(value: &Value) -> Option<DateTime<Utc>> {
    value
        .as_str()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|t| t.with_timezone(&Utc))
}
fn schedule(payload: &Value, field: &str) -> Result<Option<DateTime<Utc>>, ()> {
    match payload.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => timestamp(value).map(Some).ok_or(()),
    }
}
pub(super) async fn apply(
    access: &FieldAccess,
    claims: &server_common::Claims,
    headers: &axum::http::HeaderMap,
    e: &super::SyncEvent,
) -> Result<Outcome, SyncError> {
    let blocked = |reason| block(&e.id, EVENTS_ROUTE, reason);
    let owner = match access.authorize(claims, headers).await {
        Ok(owner) => owner,
        Err((status, _)) => {
            return Ok(blocked(if status == axum::http::StatusCode::FORBIDDEN {
                "current_owner_authority_required"
            } else {
                "canonical_authority_unavailable"
            }));
        }
    };
    let tenant = owner.tenant_id().to_owned();
    let actor = owner.actor_id().to_owned();
    if !valid_id(&e.id) || !valid_id(&e.entity_id) || e.base_version < 0 {
        return Ok(blocked("invalid_event_identity"));
    }
    if e.action_type != "UpdateStatus" {
        return Ok(blocked("unsupported_operation"));
    }
    if !e.payload.is_object() {
        return Ok(blocked("invalid_payload"));
    }
    let Some(status) = e.payload.get("status").and_then(Value::as_str).filter(|s| {
        [
            "Requested",
            "Pending",
            "Scheduled",
            "Confirmed",
            "En-Route",
            "In-Progress",
            "Completed",
            "Cancelled",
        ]
        .contains(s)
    }) else {
        return Ok(blocked("invalid_appointment_status"));
    };
    let Some(expected_status) = e.payload.get("expected_status").and_then(Value::as_str) else {
        return Ok(blocked("expected_status_required"));
    };
    let Some(observed) = e.payload.get("expected_updated_at").and_then(timestamp) else {
        return Ok(Outcome::new(
            &e.id,
            EVENTS_ROUTE,
            "reconciliation",
            Some("entity_version_required"),
        ));
    };
    if let Some(notes) = e.payload.get("notes") {
        if !matches!(notes, Value::Null | Value::String(_))
            || !matches!(
                e.payload.get("expected_notes"),
                Some(Value::Null) | Some(Value::String(_))
            )
        {
            return Ok(blocked("expected_notes_required"));
        }
        if notes
            .as_str()
            .is_some_and(|n| n.len() > 16_384 || n.contains('\0'))
        {
            return Ok(blocked("invalid_appointment_notes"));
        }
    }
    let (Ok(requested_start), Ok(requested_end)) = (
        schedule(&e.payload, "scheduled_start_time"),
        schedule(&e.payload, "scheduled_end_time"),
    ) else {
        return Ok(blocked("invalid_appointment_schedule"));
    };
    let mut tx = match owner.begin().await {
        Ok(tx) => tx,
        Err(AuthorityError::Forbidden) => return Ok(blocked("current_owner_authority_required")),
        Err(_) => return Ok(blocked("canonical_authority_unavailable")),
    };
    let identity = serde_json::to_value(e).expect("SyncEvent is JSON serializable");
    let key = match claim(
        tx.connection(),
        &tenant,
        EVENTS_ROUTE,
        &e.id,
        &e.action_type,
        &identity,
    )
    .await?
    {
        Claim::New(key) => key,
        Claim::Replay(outcome) => {
            commit_owner(tx).await?;
            return Ok(outcome);
        }
    };
    let current: Option<Value> = sqlx::query_scalar("SELECT jsonb_build_object('status',status,'notes',notes,'updated_at',updated_at,'scheduled_start_time',scheduled_start_time,'scheduled_end_time',scheduled_end_time) FROM appointments WHERE id=$1 AND tenant_id=$2 FOR UPDATE")
        .bind(&e.entity_id).bind(&tenant).fetch_optional(tx.connection()).await?;
    let Some(current) = current else {
        return Ok(blocked("entity_not_found_in_tenant"));
    };
    let version: i64 = sqlx::query_scalar("SELECT COALESCE(MAX(result_version),1) FROM sync_events WHERE tenant_id=$1 AND entity_type='appointment' AND entity_id=$2 AND receipt_status='acknowledged'")
        .bind(&tenant).bind(&e.entity_id).fetch_one(tx.connection()).await?;
    let old_status = current.get("status").and_then(Value::as_str).unwrap_or("");
    if current.get("updated_at").and_then(timestamp) != Some(observed)
        || !old_status.eq_ignore_ascii_case(expected_status)
        || (e.payload.get("notes").is_some()
            && current.get("notes") != e.payload.get("expected_notes"))
    {
        sqlx::query("INSERT INTO sync_conflict_queue (id,tenant_id,event_id,entity_id,entity_type,base_version,current_version,payload) VALUES ($1,$2,$3,$4,'appointment',$5,$6,$7)")
            .bind(uuid::Uuid::new_v4().to_string()).bind(&tenant).bind(&e.id).bind(&e.entity_id).bind(e.base_version).bind(version).bind(&e.payload).execute(tx.connection()).await?;
        task(tx.connection(), &tenant, "operations", "sync_conflict_alert", json!({"event_id":e.id,"entity_id":e.entity_id,"entity_type":"appointment","current_state":current})).await?;
        finish(tx.connection(), &tenant, &key, "reconciliation").await?;
        commit_owner(tx).await?;
        return Ok(Outcome::new(
            &e.id,
            EVENTS_ROUTE,
            "reconciliation",
            Some("expected_state_changed"),
        ));
    }
    if check_transition(old_status, status).is_err() {
        return Ok(blocked("terminal_appointment_cannot_reopen"));
    }
    // Null/omitted schedule fields preserve the stored counterpart, matching
    // the online field route. Explicit null notes continue to clear notes.
    let start = requested_start.or_else(|| current.get("scheduled_start_time").and_then(timestamp));
    let end = requested_end.or_else(|| current.get("scheduled_end_time").and_then(timestamp));
    if start.zip(end).is_some_and(|(start, end)| end < start) {
        return Ok(blocked("invalid_appointment_schedule"));
    }
    let changed = sqlx::query("UPDATE appointments SET status=$1,notes=CASE WHEN $2 THEN $3 ELSE notes END,scheduled_start_time=$4,scheduled_end_time=$5,updated_at=GREATEST(clock_timestamp(),updated_at+INTERVAL '1 microsecond') WHERE id=$6 AND tenant_id=$7 AND updated_at=$8")
        .bind(status).bind(e.payload.get("notes").is_some()).bind(e.payload.get("notes").and_then(Value::as_str)).bind(start).bind(end).bind(&e.entity_id).bind(&tenant).bind(observed).execute(tx.connection()).await?;
    if changed.rows_affected() != 1 {
        return Err(SyncError::Rejected("expected_state_changed"));
    }
    if status != "Completed" || !old_status.eq_ignore_ascii_case("Completed") {
        task(
            tx.connection(),
            &tenant,
            "operations",
            if status == "Completed" {
                "job.completed"
            } else {
                "appointment.status.updated"
            },
            json!({"sync_event_id":e.id,"entity_id":e.entity_id,"status":status,"actor_id":actor}),
        )
        .await?;
    }
    sqlx::query("UPDATE sync_events SET entity_type='appointment',entity_id=$1,base_version=$2,result_version=$3 WHERE id=$4 AND tenant_id=$5")
        .bind(&e.entity_id).bind(e.base_version).bind(version+1).bind(&key).bind(&tenant).execute(tx.connection()).await?;
    finish(tx.connection(), &tenant, &key, "acknowledged").await?;
    commit_owner(tx).await?;
    let mut outcome = Outcome::new(&e.id, EVENTS_ROUTE, "acknowledged", None);
    outcome.result_version = Some(version + 1);
    Ok(outcome)
}
