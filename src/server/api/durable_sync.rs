//! Per-request receipts and business effects commit together. A receipt is not
//! evidence of a payment or a completed downstream AI job.
use super::{OfflineMutation, OperationIntent, SyncEvent};
use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use sqlx::{PgConnection, PgPool, Postgres, Transaction};

#[path = "durable_appointment_sync.rs"]
mod appointments;
use crate::api::terminal_offline_authority as offline_authority;

use super::super::sync_transaction::{SyncError, commit_owner};
use crate::api::field_ops::records::FieldAccess;
use server_auth::commit_authority::{AuthorityError, OwnerPgTransaction};

pub(super) const EVENTS_ROUTE: &str = "/api/v1/sync/events";
pub(super) const OFFLINE_ROUTE: &str = "/api/v1/sync/offline";
pub(super) const INTENTS_ROUTE: &str = "/api/v1/sync/operation-intents";

#[derive(Debug, Clone, Serialize)]
pub struct Outcome {
    pub id: String,
    pub route: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_version: Option<i64>,
}
impl Outcome {
    pub(super) fn new(id: &str, route: &str, status: &str, reason: Option<&str>) -> Self {
        Self {
            id: id.into(),
            route: route.into(),
            status: status.into(),
            reason: reason.map(str::to_owned),
            result_version: None,
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct BatchResponse {
    pub success: bool,
    pub applied_count: i32,
    pub conflict_count: i32,
    pub failed_count: i32,
    pub outcomes: Vec<Outcome>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_reconciliation: Option<Vec<Value>>,
    #[serde(skip)]
    pub committed_products: Vec<String>,
}
impl BatchResponse {
    fn push(&mut self, outcome: Outcome) {
        match outcome.status.as_str() {
            "acknowledged" => self.applied_count += 1,
            "reconciliation" => self.conflict_count += 1,
            _ => self.failed_count += 1,
        }
        self.outcomes.push(outcome);
        self.success = self.failed_count == 0 && self.conflict_count == 0;
    }
}

pub(super) async fn begin(
    pool: &PgPool,
    tenant: &str,
) -> Result<Transaction<'static, Postgres>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    if tenant.trim().is_empty() || tenant.trim() != tenant || tenant.eq_ignore_ascii_case("system")
    {
        return Err(sqlx::Error::Protocol("A bounded tenant is required".into()));
    }
    ::server_common::auth_utils::set_org_context(&mut *tx, tenant).await?;
    Ok(tx)
}

fn receipt_key(tenant: &str, route: &str, id: &str) -> String {
    format!(
        "sync:v1:{:x}",
        Sha256::digest(json!([tenant, route, id]).to_string().as_bytes())
    )
}

enum Claim {
    New(String),
    Replay(Outcome),
}
async fn claim(
    tx: &mut PgConnection,
    tenant: &str,
    route: &str,
    id: &str,
    action: &str,
    identity: &Value,
) -> Result<Claim, sqlx::Error> {
    // Historical receipts did not prove a business mutation. Never upgrade one
    // to an acknowledgment or repeat an effect whose outcome is unknown.
    let legacy: bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM sync_events WHERE id=$1 AND tenant_id=$2 AND request_identity IS NULL)")
        .bind(id).bind(tenant).fetch_one(&mut *tx).await?;
    if legacy {
        return Ok(Claim::Replay(Outcome::new(
            id,
            route,
            "reconciliation",
            Some("legacy_receipt_requires_review"),
        )));
    }
    let key = receipt_key(tenant, route, id);
    let inserted: Option<String>=sqlx::query_scalar("INSERT INTO sync_events (id,tenant_id,action_type,payload,request_identity,receipt_route,client_event_id,receipt_status) VALUES ($1,$2,$3,$4,$5,$6,$7,'pending') ON CONFLICT (id) DO NOTHING RETURNING id")
        .bind(&key).bind(tenant).bind(action).bind(identity.to_string()).bind(identity).bind(route).bind(id).fetch_optional(&mut *tx).await?;
    if inserted.is_some() {
        return Ok(Claim::New(key));
    }
    let old: Option<(Value,Option<String>,Option<i64>)>=sqlx::query_as("SELECT request_identity,receipt_status,result_version FROM sync_events WHERE id=$1 AND tenant_id=$2 FOR UPDATE")
        .bind(&key).bind(tenant).fetch_optional(&mut *tx).await?;
    let outcome = match old {
        Some((ref old, ref status, version))
            if old == identity && status.as_deref() == Some("acknowledged") =>
        {
            let mut outcome = Outcome::new(id, route, "acknowledged", None);
            outcome.result_version = version;
            outcome
        }
        Some((ref old, ref status, _))
            if old == identity && status.as_deref() == Some("reconciliation") =>
        {
            Outcome::new(
                id,
                route,
                "reconciliation",
                Some("previous_conflict_requires_review"),
            )
        }
        _ => Outcome::new(
            id,
            route,
            "reconciliation",
            Some("request_identity_changed_or_unverified"),
        ),
    };
    Ok(Claim::Replay(outcome))
}

async fn finish(
    tx: &mut PgConnection,
    tenant: &str,
    key: &str,
    status: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE sync_events SET receipt_status=$1 WHERE id=$2 AND tenant_id=$3")
        .bind(status)
        .bind(key)
        .bind(tenant)
        .execute(&mut *tx)
        .await?;
    Ok(())
}

async fn task(
    tx: &mut PgConnection,
    tenant: &str,
    department: &str,
    event: &str,
    payload: Value,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO department_tasks (id,tenant_id,department,event_type,payload,status) VALUES ($1,$2,$3,$4,$5,'PENDING')")
        .bind(uuid::Uuid::new_v4().to_string()).bind(tenant).bind(department).bind(event).bind(payload).execute(&mut *tx).await?;
    Ok(())
}

fn valid_id(id: &str) -> bool {
    !id.trim().is_empty() && id.len() <= 512 && id.trim() == id
}
fn block(id: &str, route: &str, reason: &str) -> Outcome {
    Outcome::new(id, route, "blocked", Some(reason))
}

#[cfg(test)]
pub(super) async fn sync_events(
    pool: &PgPool,
    tenant: &str,
    events: &[SyncEvent],
) -> BatchResponse {
    let mut response = BatchResponse {
        success: true,
        ..Default::default()
    };
    for event in events {
        match apply_event(pool, tenant, event).await {
            Ok((outcome, product)) => {
                if let Some(p) = product {
                    response.committed_products.push(p);
                }
                response.push(outcome);
            }
            Err(error) => {
                tracing::warn!(%error,"Sync event transaction did not commit");
                response.push(Outcome::new(
                    &event.id,
                    EVENTS_ROUTE,
                    error.status(),
                    Some(error.reason()),
                ));
            }
        }
    }
    response
}

pub(super) async fn sync_authorized_events(
    state: &super::SyncEventsState,
    claims: &server_common::Claims,
    headers: &axum::http::HeaderMap,
    tenant: &str,
    events: &[SyncEvent],
) -> BatchResponse {
    let mut response = BatchResponse {
        success: true,
        ..Default::default()
    };
    for event in events {
        let applied = if event.entity_type == "appointment" {
            appointments::apply(&state.access, claims, headers, event)
                .await
                .map(|outcome| (outcome, None))
        } else {
            apply_event(&state.pool, tenant, event).await
        };
        match applied {
            Ok((outcome, product)) => {
                if let Some(product) = product {
                    response.committed_products.push(product);
                }
                response.push(outcome);
            }
            Err(error) => {
                tracing::warn!(%error,"Sync event transaction did not commit");
                response.push(Outcome::new(
                    &event.id,
                    EVENTS_ROUTE,
                    error.status(),
                    Some(error.reason()),
                ));
            }
        }
    }
    response
}

async fn apply_event(
    pool: &PgPool,
    tenant: &str,
    e: &SyncEvent,
) -> Result<(Outcome, Option<String>), SyncError> {
    let blocked = |reason| (block(&e.id, EVENTS_ROUTE, reason), None);
    if e.entity_type == "appointment" {
        return Ok(blocked("canonical_owner_authority_required"));
    }
    if !valid_id(&e.id) || !valid_id(&e.entity_id) || e.base_version < 0 {
        return Ok(blocked("invalid_event_identity"));
    }
    let supported = matches!(
        (e.entity_type.as_str(), e.action_type.as_str()),
        ("order", "UpdateStatus")
            | ("product", "ToggleSoldOut")
            | ("audio_intent", "ProcessVoiceCommand")
    );
    if !supported {
        return Ok(blocked("unsupported_operation"));
    }
    if e.entity_type != "audio_intent"
        && e.payload
            .get("expected_updated_at")
            .and_then(Value::as_str)
            .and_then(|v| chrono::DateTime::parse_from_rfc3339(v).ok())
            .is_none()
    {
        return Ok((
            Outcome::new(
                &e.id,
                EVENTS_ROUTE,
                "reconciliation",
                Some("entity_version_required"),
            ),
            None,
        ));
    }
    if !e.payload.is_object() {
        return Ok(blocked("invalid_payload"));
    }
    if e.entity_type == "product" {
        if e.payload
            .get("is_sold_out")
            .and_then(Value::as_bool)
            .is_none()
            || e.payload
                .get("expected_is_sold_out")
                .and_then(Value::as_bool)
                .is_none()
        {
            return Ok(blocked("expected_product_state_required"));
        }
    } else if matches!(e.entity_type.as_str(), "order") {
        let status = e
            .payload
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("");
        if status.trim().is_empty()
            || status.len() > 100
            || e.payload
                .get("expected_status")
                .and_then(Value::as_str)
                .is_none()
        {
            return Ok(blocked("expected_status_required"));
        }
        if e.payload.get("notes").is_some()
            && (e.payload.get("expected_notes").is_none()
                || !matches!(
                    e.payload.get("notes"),
                    Some(Value::Null) | Some(Value::String(_))
                )
                || !matches!(
                    e.payload.get("expected_notes"),
                    Some(Value::Null) | Some(Value::String(_))
                ))
        {
            return Ok(blocked("expected_notes_required"));
        }
    } else if e
        .payload
        .get("transcription")
        .and_then(Value::as_str)
        .is_none_or(|s| s.trim().is_empty())
    {
        return Ok(blocked("verified_transcription_required"));
    }
    let identity = serde_json::to_value(e).expect("SyncEvent is JSON serializable");
    let mut tx = begin(pool, tenant).await?;
    let key = match claim(
        &mut tx,
        tenant,
        EVENTS_ROUTE,
        &e.id,
        &e.action_type,
        &identity,
    )
    .await?
    {
        Claim::New(key) => key,
        Claim::Replay(outcome) => return Ok((outcome, None)),
    };
    let current=match e.entity_type.as_str() {
        "product" => sqlx::query_scalar::<_,Value>("SELECT jsonb_build_object('is_sold_out',is_sold_out,'updated_at',updated_at) FROM products WHERE id=$1 AND tenant_id=$2 FOR UPDATE").bind(&e.entity_id).bind(tenant).fetch_optional(&mut *tx).await?,
        "order" => sqlx::query_scalar::<_,Value>("SELECT jsonb_build_object('status',status,'notes',notes,'updated_at',updated_at) FROM orders WHERE id=$1 AND tenant_id=$2 FOR UPDATE").bind(&e.entity_id).bind(tenant).fetch_optional(&mut *tx).await?,
        _=>Some(json!({})),
    };
    let Some(current) = current else {
        return Ok(blocked("entity_not_found_in_tenant"));
    };
    let version:i64=sqlx::query_scalar("SELECT COALESCE(MAX(result_version),1) FROM sync_events WHERE tenant_id=$1 AND entity_type=$2 AND entity_id=$3 AND receipt_status='acknowledged'")
        .bind(tenant).bind(&e.entity_type).bind(&e.entity_id).fetch_one(&mut *tx).await?;
    let timestamp_matches = e
        .payload
        .get("expected_updated_at")
        .and_then(Value::as_str)
        .and_then(|expected| chrono::DateTime::parse_from_rfc3339(expected).ok())
        .zip(
            current
                .get("updated_at")
                .and_then(Value::as_str)
                .and_then(|actual| chrono::DateTime::parse_from_rfc3339(actual).ok()),
        )
        .is_some_and(|(expected, actual)| expected == actual);
    let version_conflict = e.entity_type != "audio_intent"
        && (if e.payload.get("expected_updated_at").is_some() {
            !timestamp_matches
        } else {
            e.base_version != version
        });
    let conflict = version_conflict
        || match e.entity_type.as_str() {
            "product" => current.get("is_sold_out") != e.payload.get("expected_is_sold_out"),
            "order" => {
                current.get("status") != e.payload.get("expected_status")
                    || (e.payload.get("notes").is_some()
                        && current.get("notes") != e.payload.get("expected_notes"))
            }
            _ => false,
        };
    if conflict {
        sqlx::query("INSERT INTO sync_conflict_queue (id,tenant_id,event_id,entity_id,entity_type,base_version,current_version,payload) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)")
            .bind(uuid::Uuid::new_v4().to_string()).bind(tenant).bind(&e.id).bind(&e.entity_id).bind(&e.entity_type).bind(e.base_version).bind(version).bind(&e.payload).execute(&mut *tx).await?;
        task(&mut tx,tenant,"operations","sync_conflict_alert",json!({"event_id":e.id,"entity_id":e.entity_id,"entity_type":e.entity_type,"current_state":current})).await?;
        finish(&mut tx, tenant, &key, "reconciliation").await?;
        tx.commit().await.map_err(SyncError::Commit)?;
        return Ok((
            Outcome::new(
                &e.id,
                EVENTS_ROUTE,
                "reconciliation",
                Some("expected_state_changed"),
            ),
            None,
        ));
    }
    match e.entity_type.as_str() {
        "product"=>{ sqlx::query("UPDATE products SET is_sold_out=$1,updated_at=clock_timestamp() WHERE id=$2 AND tenant_id=$3").bind(e.payload["is_sold_out"].as_bool().unwrap()).bind(&e.entity_id).bind(tenant).execute(&mut *tx).await?; },
        "order"=>{
            let query="UPDATE orders SET status=$1, notes=CASE WHEN $2 THEN $3 ELSE notes END, updated_at=clock_timestamp() WHERE id=$4 AND tenant_id=$5";
            sqlx::query(query).bind(e.payload["status"].as_str().unwrap()).bind(e.payload.get("notes").is_some()).bind(e.payload.get("notes").and_then(Value::as_str)).bind(&e.entity_id).bind(tenant).execute(&mut *tx).await?;
            let event="order.status.updated";
            task(&mut tx,tenant,"operations",event,json!({"sync_event_id":e.id,"entity_id":e.entity_id,"status":e.payload["status"]})).await?;
        },
        _=>task(&mut tx,tenant,"operations","voice.intent.synced",json!({"sync_event_id":e.id,"entity_id":e.entity_id,"transcription":e.payload["transcription"]})).await?,
    }
    sqlx::query("UPDATE sync_events SET entity_type=$1,entity_id=$2,base_version=$3,result_version=$4 WHERE id=$5 AND tenant_id=$6")
        .bind(&e.entity_type).bind(&e.entity_id).bind(e.base_version).bind(version+1).bind(&key).bind(tenant).execute(&mut *tx).await?;
    finish(&mut tx, tenant, &key, "acknowledged").await?;
    tx.commit().await.map_err(SyncError::Commit)?;
    let product = (e.entity_type == "product").then(|| e.entity_id.clone());
    let mut outcome = Outcome::new(&e.id, EVENTS_ROUTE, "acknowledged", None);
    outcome.result_version = Some(version + 1);
    Ok((outcome, product))
}

enum WriteTransaction {
    Owner(OwnerPgTransaction),
    #[cfg(test)]
    Legacy(Transaction<'static, Postgres>),
}
impl WriteTransaction {
    fn connection(&mut self) -> &mut PgConnection {
        match self {
            Self::Owner(tx) => tx.connection(),
            #[cfg(test)]
            Self::Legacy(tx) => tx,
        }
    }
    async fn commit(self) -> Result<(), SyncError> {
        match self {
            Self::Owner(tx) => commit_owner(tx).await,
            #[cfg(test)]
            Self::Legacy(tx) => tx.commit().await.map_err(SyncError::Commit),
        }
    }
}
async fn begin_authorized_write(
    access: &FieldAccess,
    claims: &server_common::Claims,
    headers: &axum::http::HeaderMap,
) -> Result<(WriteTransaction, String), SyncError> {
    let owner = access
        .authorize(claims, headers)
        .await
        .map_err(|(status, _)| {
            SyncError::Rejected(if status == axum::http::StatusCode::FORBIDDEN {
                "current_owner_authority_required"
            } else {
                "canonical_authority_unavailable"
            })
        })?;
    let tenant = owner.tenant_id().to_owned();
    let tx = owner.begin().await.map_err(|error| {
        SyncError::Rejected(if matches!(error, AuthorityError::Forbidden) {
            "current_owner_authority_required"
        } else {
            "canonical_authority_unavailable"
        })
    })?;
    Ok((WriteTransaction::Owner(tx), tenant))
}
pub(super) async fn sync_authorized_intents(
    access: &FieldAccess,
    claims: &server_common::Claims,
    headers: &axum::http::HeaderMap,
    intents: &[OperationIntent],
) -> BatchResponse {
    let mut response = BatchResponse {
        success: true,
        ..Default::default()
    };
    for intent in intents {
        let result = match begin_authorized_write(access, claims, headers).await {
            Ok((tx, tenant)) => apply_intent(tx, &tenant, intent).await,
            Err(error) => Err(error),
        };
        response.push(match result {
            Ok(outcome) => outcome,
            Err(error) => Outcome::new(
                &intent.id,
                INTENTS_ROUTE,
                error.status(),
                Some(error.reason()),
            ),
        });
    }
    response
}
pub(super) async fn sync_authorized_mutations(
    access: &FieldAccess,
    claims: &server_common::Claims,
    headers: &axum::http::HeaderMap,
    mutations: &[OfflineMutation],
) -> BatchResponse {
    let mut response = BatchResponse {
        success: true,
        ..Default::default()
    };
    for mutation in mutations {
        let result = match begin_authorized_write(access, claims, headers).await {
            Ok((tx, tenant)) => apply_mutation(tx, &tenant, mutation).await,
            Err(error) => Err(error),
        };
        match result {
            Ok((outcome, product, conflict)) => {
                if let Some(product) = product {
                    response.committed_products.push(product);
                }
                if let Some(conflict) = conflict {
                    response
                        .pending_reconciliation
                        .get_or_insert_default()
                        .push(conflict);
                }
                response.push(outcome);
            }
            Err(error) => response.push(Outcome::new(
                mutation
                    .client_mutation_id
                    .as_deref()
                    .unwrap_or(&mutation.transaction_id),
                OFFLINE_ROUTE,
                error.status(),
                Some(error.reason()),
            )),
        }
    }
    response
}
#[cfg(test)]
pub(super) async fn sync_intents(
    pool: &PgPool,
    tenant: &str,
    intents: &[OperationIntent],
) -> BatchResponse {
    let mut response = BatchResponse {
        success: true,
        ..Default::default()
    };
    for intent in intents {
        let result = match begin(pool, tenant).await {
            Ok(tx) => apply_intent(WriteTransaction::Legacy(tx), tenant, intent).await,
            Err(error) => Err(error.into()),
        };
        let outcome = match result {
            Ok(o) => o,
            Err(error) => {
                tracing::warn!(%error,"Operation intent did not commit");
                Outcome::new(
                    &intent.id,
                    INTENTS_ROUTE,
                    error.status(),
                    Some(error.reason()),
                )
            }
        };
        response.push(outcome);
    }
    response
}
async fn apply_intent(
    mut tx: WriteTransaction,
    tenant: &str,
    intent: &OperationIntent,
) -> Result<Outcome, SyncError> {
    if !valid_id(&intent.id) || intent.action_type.trim().is_empty() || !intent.payload.is_object()
    {
        return Ok(block(&intent.id, INTENTS_ROUTE, "invalid_intent"));
    }
    let identity =
        json!({"id":intent.id,"action_type":intent.action_type,"payload":intent.payload});
    let key = match claim(
        tx.connection(),
        tenant,
        INTENTS_ROUTE,
        &intent.id,
        &intent.action_type,
        &identity,
    )
    .await?
    {
        Claim::New(k) => k,
        Claim::Replay(o) => {
            tx.commit().await?;
            return Ok(o);
        }
    };
    let existing: Option<(Option<Value>,)> = sqlx::query_as(
        "SELECT request_identity FROM operation_intents WHERE id=$1 AND tenant_id=$2 FOR UPDATE",
    )
    .bind(&intent.id)
    .bind(tenant)
    .fetch_optional(tx.connection())
    .await?;
    if existing.is_some() {
        return Ok(Outcome::new(
            &intent.id,
            INTENTS_ROUTE,
            "reconciliation",
            Some("legacy_intent_requires_review"),
        ));
    }
    sqlx::query("INSERT INTO operation_intents (id,tenant_id,action_type,payload,status,request_identity) VALUES ($1,$2,$3,$4,'SYNCED',$5)")
        .bind(&intent.id).bind(tenant).bind(&intent.action_type).bind(&intent.payload).bind(&identity).execute(tx.connection()).await?;
    finish(tx.connection(), tenant, &key, "acknowledged").await?;
    tx.commit().await?;
    Ok(Outcome::new(
        &intent.id,
        INTENTS_ROUTE,
        "acknowledged",
        None,
    ))
}

#[cfg(test)]
pub(super) async fn sync_mutations(
    pool: &PgPool,
    tenant: &str,
    mutations: &[OfflineMutation],
) -> BatchResponse {
    let mut response = BatchResponse {
        success: true,
        ..Default::default()
    };
    for mutation in mutations {
        let id = mutation
            .client_mutation_id
            .as_deref()
            .unwrap_or(&mutation.transaction_id);
        let result = match begin(pool, tenant).await {
            Ok(tx) => apply_mutation(WriteTransaction::Legacy(tx), tenant, mutation).await,
            Err(error) => Err(error.into()),
        };
        match result {
            Ok((outcome, product, conflict)) => {
                if let Some(p) = product {
                    response.committed_products.push(p);
                }
                if let Some(c) = conflict {
                    response
                        .pending_reconciliation
                        .get_or_insert_default()
                        .push(c);
                }
                response.push(outcome);
            }
            Err(error) => {
                tracing::warn!(%error,"Offline mutation did not commit");
                response.push(Outcome::new(
                    id,
                    OFFLINE_ROUTE,
                    error.status(),
                    Some(error.reason()),
                ));
            }
        }
    }
    response
}
async fn apply_mutation(
    mut tx: WriteTransaction,
    tenant: &str,
    m: &OfflineMutation,
) -> Result<(Outcome, Option<String>, Option<Value>), SyncError> {
    let id = m.client_mutation_id.as_deref().unwrap_or(&m.transaction_id);
    let blocked = |r| (block(id, OFFLINE_ROUTE, r), None, None);
    if !valid_id(id) || !valid_id(&m.transaction_id) {
        return Ok(blocked("stable_mutation_identity_required"));
    }
    let kind = m.mutation_type.as_deref().unwrap_or("inventory_sale");
    if !matches!(
        kind,
        "draft_quote" | "agent_intent" | "inventory_sale" | "cash_sale" | "tap_to_pay"
    ) {
        return Ok(blocked("unsupported_operation"));
    }
    let inventory = matches!(kind, "inventory_sale" | "cash_sale" | "tap_to_pay");
    if inventory
        && (m.quantity_deducted <= 0 || !valid_id(&m.product_id) || m.amount.is_some_and(|a| a < 0))
    {
        return Ok(blocked("invalid_inventory_mutation"));
    }
    if !inventory && m.payload.as_deref().is_none_or(|p| p.trim().is_empty()) {
        return Ok(blocked("payload_required"));
    }
    let identity = serde_json::to_value(m).expect("OfflineMutation is JSON serializable");
    if matches!(kind, "cash_sale" | "inventory_sale")
        && offline_authority::explicit_kind(&identity) != Some(kind)
    {
        return Ok(blocked("explicit_consistent_operation_required"));
    }
    let key = match claim(tx.connection(), tenant, OFFLINE_ROUTE, id, kind, &identity).await? {
        Claim::New(k) => k,
        Claim::Replay(o) => {
            tx.commit().await?;
            return Ok((o, None, None));
        }
    };
    let legacy:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM applied_client_mutations WHERE client_mutation_id=$1 AND tenant_id=$2)").bind(id).bind(tenant).fetch_one(tx.connection()).await?;
    if legacy {
        return Ok((
            Outcome::new(
                id,
                OFFLINE_ROUTE,
                "reconciliation",
                Some("legacy_mutation_requires_review"),
            ),
            None,
            None,
        ));
    }
    // Preserve the existing cross-version idempotency marker in the same
    // transaction as the stronger identity receipt and the business effect.
    sqlx::query(
        "INSERT INTO applied_client_mutations (client_mutation_id,tenant_id) VALUES ($1,$2)",
    )
    .bind(id)
    .bind(tenant)
    .execute(tx.connection())
    .await?;
    let mut conflict = None;
    if kind == "draft_quote" {
        task(
            tx.connection(),
            tenant,
            "sales",
            "tenant.omnichannel.message.received",
            json!({"source":"offline_app","message":m.payload,"client_mutation_id":id}),
        )
        .await?;
    } else if kind == "agent_intent" {
        let payload: Value = match serde_json::from_str(m.payload.as_deref().unwrap_or("")) {
            Ok(p) => p,
            Err(_) => return Ok(blocked("invalid_intent_payload")),
        };
        sqlx::query("INSERT INTO ohc_job_queue (id,tenant_id,job_type,payload) VALUES ($1,$2,'agent_intent',$3)").bind(uuid::Uuid::new_v4().to_string()).bind(tenant).bind(payload).execute(tx.connection()).await?;
    } else {
        let stock: Option<(i32,bool)> = sqlx::query_as(
            "SELECT inventory_count, (to_jsonb(products) ? 'pn_counter_p' AND to_jsonb(products) ? 'pn_counter_n') FROM products WHERE id=$1 AND tenant_id=$2 FOR UPDATE",
        )
        .bind(&m.product_id)
        .bind(tenant)
        .fetch_optional(tx.connection())
        .await?;
        let Some((stock, has_counters)) = stock else {
            return Ok(blocked("product_not_found_in_tenant"));
        };
        let levels: Vec<i32> = sqlx::query_scalar("SELECT available_count FROM inventory_levels WHERE variant_id=$1 AND tenant_id=$2 FOR UPDATE")
            .bind(&m.product_id).bind(tenant).fetch_all(tx.connection()).await?;
        if levels.len() > 1 {
            return Ok(blocked("inventory_location_required"));
        }
        let level = levels.first().copied();
        let available = level.unwrap_or(stock);
        if available < m.quantity_deducted {
            let c = json!({"transaction_id":m.transaction_id,"product_id":m.product_id,"shortage":i64::from(m.quantity_deducted)-i64::from(available)});
            let conflict_id = format!("sync_conflict_{}_{}", m.transaction_id, m.product_id);
            let c_db = json!({
                "transaction_id": m.transaction_id,
                "product_id": m.product_id,
                "expected_stock": m.quantity_deducted,
                "actual_stock": available,
                "message": format!("Inventory Sync Conflict: {} sold out offline, causing an online shortage. Operations is resolving this.", m.product_id)
            });
            sqlx::query(
                "INSERT INTO agent_action_requests (id, tenant_id, source, agent_type, action_type, status, confidence_score, payload, created_at, updated_at)
                 VALUES ($1, $2, 'terminal', 'operations', 'inventory.sync.conflict', 'Pending', 0.99, $3::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                 ON CONFLICT DO NOTHING"
            )
            .bind(&conflict_id)
            .bind(tenant)
            .bind(&c_db)
            .execute(tx.connection())
            .await?;
            conflict = Some(c);
        }
        sqlx::query(if has_counters {
            "UPDATE products SET pn_counter_n=COALESCE(pn_counter_n,0)+$1,inventory_count=GREATEST(0,COALESCE(pn_counter_p,0)-(COALESCE(pn_counter_n,0)+$1)),available_quantity=GREATEST(0,available_quantity-$1),updated_at=clock_timestamp() WHERE id=$2 AND tenant_id=$3"
        } else {
            "UPDATE products SET inventory_count=GREATEST(0,inventory_count-$1),available_quantity=GREATEST(0,available_quantity-$1),updated_at=clock_timestamp() WHERE id=$2 AND tenant_id=$3"
        }).bind(m.quantity_deducted).bind(&m.product_id).bind(tenant).execute(tx.connection()).await?;
        if level.is_some() {
            sqlx::query("UPDATE inventory_levels SET available_count=GREATEST(0,available_count-$1) WHERE variant_id=$2 AND tenant_id=$3").bind(m.quantity_deducted).bind(&m.product_id).bind(tenant).execute(tx.connection()).await?;
        }
        sqlx::query("INSERT INTO ohc_job_queue (id,tenant_id,job_type,payload) VALUES ($1,$2,'offline_pos_sync',$3)")
            .bind(uuid::Uuid::new_v4().to_string()).bind(tenant).bind(offline_mutation_job(&key, &identity)).execute(tx.connection()).await?;
    }
    let status = if conflict.is_some() {
        "reconciliation"
    } else {
        "acknowledged"
    };
    finish(tx.connection(), tenant, &key, status).await?;
    tx.commit().await?;
    Ok((
        Outcome::new(
            id,
            OFFLINE_ROUTE,
            status,
            conflict.as_ref().map(|_| "inventory_shortage"),
        ),
        inventory.then(|| m.product_id.clone()),
        conflict,
    ))
}

#[cfg(test)]
#[path = "durable_sync_test.rs"]
mod tests;

// Keep the exact canonical receipt identity across the producer/worker boundary.
fn offline_mutation_job(receipt_id: &str, identity: &Value) -> Value {
    json!({"transaction_id":identity["transaction_id"],"receipt_id":receipt_id,
        "mutation_type":identity["mutation_type"],"mutation":identity,
        "inventory_already_deducted":true})
}
