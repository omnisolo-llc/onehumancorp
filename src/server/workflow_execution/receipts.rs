//! Durable receipt contract for admitted text-only work on the configured ORM.
//! This module is not mounted while its storage contract is being implemented.
use super::{AdmittedAnalysis, Authority};
use crate::persistence::AppDatabase;
use sea_orm::{
    ConnectionTrait, DatabaseTransaction, QueryResult, Statement, TransactionTrait, Value,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub(crate) struct RequestMetadata {
    pub request_id: Uuid,
    pub name: String,
    pub workflow: String,
    pub requested_model: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum StoredPhase {
    Queued,
    Dispatching,
    Completed,
    Cancelled,
    OutcomeUnknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct Receipt {
    pub id: String,
    pub tenant_id: String,
    pub actor_id: String,
    pub request_id: String,
    pub name: String,
    pub workflow: String,
    pub task: String,
    pub provider: String,
    pub model: String,
    pub max_output_tokens: i32,
    pub phase: StoredPhase,
    pub created_at: i64,
    pub updated_at: i64,
    pub output: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug)]
pub(crate) enum Error {
    Invalid,
    Forbidden,
    Conflict,
    NotFound,
    Unavailable,
    Corrupt,
    Database(sea_orm::DbErr),
}
impl From<sea_orm::DbErr> for Error {
    fn from(value: sea_orm::DbErr) -> Self {
        Self::Database(value)
    }
}

#[derive(Clone)]
pub(crate) struct ReceiptStore {
    database: AppDatabase,
}

pub(crate) struct Reservation {
    receipt: Receipt,
    admitted: Option<AdmittedAnalysis>,
    authority: Authority,
}
impl Reservation {
    pub(crate) fn receipt(&self) -> &Receipt {
        &self.receipt
    }
    pub(crate) fn replayed(&self) -> bool {
        self.admitted.is_none()
    }
}

pub(crate) struct DispatchLease {
    receipt: Receipt,
    admitted: AdmittedAnalysis,
    nonce: String,
    fingerprint: String,
}

/// Internal storage-finalization capability, never an HTTP input or DTO.
/// It cannot repeat provider execution; it only acknowledges this exact claim.
#[derive(Clone)]
pub(crate) struct CompletionProof {
    pub(super) receipt: Receipt,
    pub(super) authority: Authority,
    pub(super) nonce: String,
    pub(super) generation: i64,
    pub(super) fingerprint: String,
}
impl DispatchLease {
    pub(crate) fn completion_proof(&self) -> CompletionProof {
        CompletionProof {
            receipt: self.receipt.clone(),
            authority: self.admitted.authority.clone(),
            nonce: self.nonce.clone(),
            generation: 1,
            fingerprint: self.fingerprint.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Payload {
    version: u8,
    name: String,
    workflow: String,
    requested_model: String,
    task: String,
    provider: String,
    model: String,
    max_output_tokens: i32,
}
impl Payload {
    fn validate(&self) -> Result<(), Error> {
        if self.version != 1
            || self.name.trim().is_empty()
            || self.name.chars().count() > 200
            || self.name.contains('\0')
            || self.task.trim().is_empty()
            || self.task.chars().count() > super::MAX_TASK_CHARACTERS
            || !matches!(self.workflow.as_str(), "" | "expert_task" | "analysis")
            || (!self.requested_model.trim().is_empty()
                && self.requested_model != "Auto"
                && self.requested_model != self.model)
            || super::AnalysisPolicy::new(
                self.provider.clone(),
                self.model.clone(),
                self.max_output_tokens,
            )
            .is_err()
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
    fn matches_request(&self, input: &AdmittedAnalysis, request: &RequestMetadata) -> bool {
        self.name == request.name
            && self.workflow == request.workflow
            && self.requested_model == request.requested_model
            && self.task == input.task
    }
}

struct Stored {
    receipt: Receipt,
    payload: Payload,
    fingerprint: String,
}
fn statement(tx: &DatabaseTransaction, sql: &str, values: Vec<Value>) -> Statement {
    Statement::from_sql_and_values(tx.get_database_backend(), sql, values)
}
async fn execute(tx: &DatabaseTransaction, sql: &str, values: Vec<Value>) -> Result<u64, Error> {
    Ok(tx
        .execute(statement(tx, sql, values))
        .await?
        .rows_affected())
}
async fn row(
    tx: &DatabaseTransaction,
    sql: &str,
    values: Vec<Value>,
) -> Result<Option<QueryResult>, Error> {
    Ok(tx.query_one(statement(tx, sql, values)).await?)
}
fn fingerprint(
    tenant: &str,
    actor: &str,
    request: &str,
    token: &str,
    expiry: i64,
    session: Option<&str>,
    payload: &Payload,
) -> Result<String, Error> {
    let bytes = serde_json::to_vec(&(
        "ohc-tenant-text-admission-v1",
        tenant,
        actor,
        request,
        token,
        expiry,
        session,
        payload,
    ))
    .map_err(|_| Error::Corrupt)?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}
fn field<T: sea_orm::TryGetable>(row: &QueryResult, name: &str) -> Result<T, Error> {
    row.try_get("", name).map_err(|_| Error::Corrupt)
}
fn decode(row: QueryResult) -> Result<Stored, Error> {
    let payload: Payload =
        serde_json::from_str(&field::<String>(&row, "payload")?).map_err(|_| Error::Corrupt)?;
    payload.validate().map_err(|_| Error::Corrupt)?;
    let id = field::<String>(&row, "id")?;
    let request_id = field::<String>(&row, "request_id")?;
    if Uuid::parse_str(&id).is_err() || Uuid::parse_str(&request_id).is_err() {
        return Err(Error::Corrupt);
    }
    let tenant_id = field::<String>(&row, "tenant_id")?;
    let actor_id = field::<String>(&row, "actor_id")?;
    let token_id = field::<String>(&row, "token_id")?;
    let token_expires_at = field::<i64>(&row, "token_expires_at")?;
    let session_id = field::<Option<String>>(&row, "session_id")?;
    let saved = field::<String>(&row, "fingerprint")?;
    if fingerprint(
        &tenant_id,
        &actor_id,
        &request_id,
        &token_id,
        token_expires_at,
        session_id.as_deref(),
        &payload,
    )? != saved
    {
        return Err(Error::Corrupt);
    }
    let phase = match field::<String>(&row, "phase")?.as_str() {
        "queued" => StoredPhase::Queued,
        "dispatching" => StoredPhase::Dispatching,
        "completed" => StoredPhase::Completed,
        "cancelled" => StoredPhase::Cancelled,
        "outcome_unknown" => StoredPhase::OutcomeUnknown,
        _ => return Err(Error::Corrupt),
    };
    let output = field::<Option<String>>(&row, "output")?;
    if output
        .as_ref()
        .is_some_and(|value| value.trim().is_empty() || value.len() > super::MAX_OUTPUT_BYTES)
    {
        return Err(Error::Corrupt);
    }
    let receipt = Receipt {
        id,
        tenant_id,
        actor_id,
        request_id,
        name: payload.name.clone(),
        workflow: payload.workflow.clone(),
        task: payload.task.clone(),
        provider: payload.provider.clone(),
        model: payload.model.clone(),
        max_output_tokens: payload.max_output_tokens,
        phase,
        created_at: field(&row, "created_at")?,
        updated_at: field(&row, "updated_at")?,
        output,
        error: field(&row, "error")?,
    };
    Ok(Stored {
        receipt,
        payload,
        fingerprint: saved,
    })
}

impl ReceiptStore {
    pub(crate) fn new(database: AppDatabase) -> Self {
        Self { database }
    }

    async fn begin_transaction(
        &self,
        authority: &Authority,
        write: bool,
    ) -> Result<(DatabaseTransaction, i64), Error> {
        // PostgreSQL admission remains unavailable until its forced-RLS migration
        // and restricted-role contract are verified. No backend fallback exists.
        if self.database.connection().get_database_backend() != sea_orm::DatabaseBackend::Sqlite {
            return Err(Error::Unavailable);
        }
        if authority.tenant_id.trim().is_empty()
            || authority.tenant_id.trim() != authority.tenant_id
            || authority.tenant_id.eq_ignore_ascii_case("system")
            || authority.tenant_id.len() > 512
            || authority.actor_id.trim().is_empty()
            || authority.actor_id.len() > 512
            || authority.token_id.trim().is_empty()
            || authority.token_id.len() > 512
            || authority
                .session_id
                .as_ref()
                .is_some_and(|s| s.trim().is_empty() || s.len() > 512)
        {
            return Err(Error::Forbidden);
        }
        let tx = self.database.connection().begin().await?;
        if write {
            // Obtain SQLite write intent before the first read snapshot. No row
            // is changed and no row-level trigger is invoked by this statement.
            execute(
                &tx,
                "UPDATE tenant_workflow_receipts SET id=id WHERE 0",
                vec![],
            )
            .await?;
        }
        let enabled = row(&tx, "PRAGMA foreign_keys", vec![])
            .await?
            .ok_or(Error::Unavailable)?;
        if enabled
            .try_get_by_index::<i64>(0)
            .map_err(|_| Error::Unavailable)?
            != 1
        {
            return Err(Error::Unavailable);
        }
        let clock = row(
            &tx,
            "SELECT CAST(strftime('%s','now') AS INTEGER) AS now",
            vec![],
        )
        .await?
        .ok_or(Error::Unavailable)?;
        let now: i64 = field(&clock, "now")?;
        Ok((tx, now))
    }

    async fn require_authority(
        tx: &DatabaseTransaction,
        authority: &Authority,
        now: i64,
    ) -> Result<(), Error> {
        if authority.expires_at <= now {
            return Err(Error::Forbidden);
        }
        let identity = row(
            tx,
            "SELECT active FROM users WHERE id=$1 AND tenant_id=$2",
            vec![(&authority.actor_id).into(), (&authority.tenant_id).into()],
        )
        .await?
        .ok_or(Error::Forbidden)?;
        if !field::<bool>(&identity, "active")? {
            return Err(Error::Forbidden);
        }
        let roles = tx
            .query_all(statement(
                tx,
                "SELECT role_name FROM identity_user_roles WHERE user_id=$1 AND tenant_id=$2",
                vec![(&authority.actor_id).into(), (&authority.tenant_id).into()],
            ))
            .await?;
        let roles: Vec<String> = roles
            .iter()
            .map(|r| field(r, "role_name"))
            .collect::<Result<_, _>>()?;
        if !roles
            .iter()
            .any(|r| r.eq_ignore_ascii_case("owner") || r.eq_ignore_ascii_case("admin"))
        {
            return Err(Error::Forbidden);
        }
        if row(
            tx,
            "SELECT jti FROM auth_revoked_tokens WHERE jti=$1 AND tenant_id=$2",
            vec![(&authority.token_id).into(), (&authority.tenant_id).into()],
        )
        .await?
        .is_some()
        {
            return Err(Error::Forbidden);
        }
        Ok(())
    }

    async fn transaction(
        &self,
        authority: &Authority,
        write: bool,
    ) -> Result<(DatabaseTransaction, i64), Error> {
        let (tx, now) = self.begin_transaction(authority, write).await?;
        Self::require_authority(&tx, authority, now).await?;
        Ok((tx, now))
    }

    pub(crate) async fn reserve(
        &self,
        input: AdmittedAnalysis,
        request: RequestMetadata,
    ) -> Result<Reservation, Error> {
        if request.request_id.is_nil() {
            return Err(Error::Invalid);
        }
        let payload = Payload {
            version: 1,
            name: request.name.clone(),
            workflow: request.workflow.clone(),
            requested_model: request.requested_model.clone(),
            task: input.task.clone(),
            provider: input.policy.provider.clone(),
            model: input.policy.model.clone(),
            max_output_tokens: input.policy.max_output_tokens,
        };
        payload.validate()?;
        let authority = input.authority.clone();
        let (tx, now) = self.transaction(&authority, true).await?;
        let request_id = request.request_id.to_string();
        if let Some(prior) = row(&tx, "SELECT * FROM tenant_workflow_receipts WHERE tenant_id=$1 AND actor_id=$2 AND request_id=$3", vec![(&authority.tenant_id).into(), (&authority.actor_id).into(), (&request_id).into()]).await? {
            let prior = decode(prior)?;
            if !prior.payload.matches_request(&input, &request) { return Err(Error::Conflict); }
            tx.commit().await?;
            return Ok(Reservation { receipt: prior.receipt, admitted: None, authority });
        }
        let id = Uuid::new_v4().to_string();
        let hash = fingerprint(
            &authority.tenant_id,
            &authority.actor_id,
            &request_id,
            &authority.token_id,
            authority.expires_at,
            authority.session_id.as_deref(),
            &payload,
        )?;
        execute(&tx, "INSERT INTO tenant_workflow_receipts(id,tenant_id,actor_id,request_id,fingerprint,payload,token_id,token_expires_at,session_id,phase,generation,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,'queued',0,$10,$10)", vec![(&id).into(), (&authority.tenant_id).into(), (&authority.actor_id).into(), request_id.into(), hash.into(), serde_json::to_string(&payload).map_err(|_| Error::Invalid)?.into(), (&authority.token_id).into(), authority.expires_at.into(), authority.session_id.clone().into(), now.into()]).await?;
        let actual = decode(row(&tx, "SELECT * FROM tenant_workflow_receipts WHERE id=$1 AND tenant_id=$2 AND actor_id=$3", vec![id.into(), (&authority.tenant_id).into(), (&authority.actor_id).into()]).await?.ok_or(Error::Corrupt)?)?;
        tx.commit().await?;
        Ok(Reservation {
            receipt: actual.receipt,
            admitted: Some(input),
            authority,
        })
    }

    /// Persist an acknowledged storage outcome for the exact admitted claim.
    /// The caller must obtain it from actual execution; this method performs no
    /// provider work and cannot make an unknown provider attempt safe to retry.
    pub(crate) async fn finish(
        &self,
        proof: &CompletionProof,
        outcome: &super::AnalysisOutcome,
    ) -> Result<Receipt, Error> {
        let (requested_phase, requested_output, requested_error) = match outcome {
            super::AnalysisOutcome::Completed(output) => {
                if output.trim().is_empty() || output.len() > super::MAX_OUTPUT_BYTES {
                    return Err(Error::Invalid);
                }
                (StoredPhase::Completed, Some(output.clone()), None)
            }
            super::AnalysisOutcome::Cancelled => (StoredPhase::Cancelled, None, Some("cancelled")),
            super::AnalysisOutcome::OutcomeUnknown => (
                StoredPhase::OutcomeUnknown,
                None,
                Some("execution_uncertain"),
            ),
        };
        if proof.generation != 1
            || proof.receipt.tenant_id != proof.authority.tenant_id
            || proof.receipt.actor_id != proof.authority.actor_id
        {
            return Err(Error::Conflict);
        }
        let (tx, now) = self.begin_transaction(&proof.authority, true).await?;
        let saved = row(
            &tx,
            "SELECT * FROM tenant_workflow_receipts WHERE id=$1 AND tenant_id=$2 AND actor_id=$3",
            vec![
                (&proof.receipt.id).into(),
                (&proof.authority.tenant_id).into(),
                (&proof.authority.actor_id).into(),
            ],
        )
        .await?
        .ok_or(Error::NotFound)?;
        let generation: i64 = field(&saved, "generation")?;
        let nonce: Option<String> = field(&saved, "lease")?;
        let expires: Option<i64> = field(&saved, "lease_expires_at")?;
        if nonce.as_deref() != Some(proof.nonce.as_str())
            || field::<String>(&saved, "token_id")? != proof.authority.token_id
            || field::<i64>(&saved, "token_expires_at")? != proof.authority.expires_at
            || field::<Option<String>>(&saved, "session_id")? != proof.authority.session_id
        {
            return Err(Error::Conflict);
        }
        let saved = decode(saved)?;
        let mut expected = proof.receipt.clone();
        expected.phase = saved.receipt.phase;
        expected.updated_at = saved.receipt.updated_at;
        expected.output.clone_from(&saved.receipt.output);
        expected.error.clone_from(&saved.receipt.error);
        if expected != saved.receipt || proof.fingerprint != saved.fingerprint {
            return Err(Error::Conflict);
        }
        let authority = Self::require_authority(&tx, &proof.authority, now).await;
        if matches!(
            saved.receipt.phase,
            StoredPhase::Completed | StoredPhase::Cancelled | StoredPhase::OutcomeUnknown
        ) {
            authority?;
            if generation != 2
                || saved.receipt.phase != requested_phase
                || saved.receipt.output != requested_output
            {
                return Err(Error::Conflict);
            }
            // A terminal replay is read-only: don't reopen or re-run a trigger.
            tx.commit().await?;
            return Ok(saved.receipt);
        }
        if saved.receipt.phase != StoredPhase::Dispatching || generation != proof.generation {
            return Err(Error::Conflict);
        }
        let authority_lost = match authority {
            Ok(()) => false,
            Err(Error::Forbidden) => true,
            Err(error) => return Err(error),
        };
        let (phase, output, error) = if authority_lost {
            (StoredPhase::OutcomeUnknown, None, Some("authority_lost"))
        } else if expires.ok_or(Error::Corrupt)? <= now {
            (StoredPhase::OutcomeUnknown, None, Some("lease_expired"))
        } else {
            (requested_phase, requested_output, requested_error)
        };
        let phase = match phase {
            StoredPhase::Completed => "completed",
            StoredPhase::Cancelled => "cancelled",
            StoredPhase::OutcomeUnknown => "outcome_unknown",
            _ => return Err(Error::Invalid),
        };
        let changed = execute(&tx, "UPDATE tenant_workflow_receipts SET phase=$1,generation=generation+1,updated_at=$2,output=$3,error=$4 WHERE id=$5 AND tenant_id=$6 AND actor_id=$7 AND phase='dispatching' AND generation=$8 AND lease=$9 AND fingerprint=$10", vec![phase.into(), now.into(), output.into(), error.into(), (&proof.receipt.id).into(), (&proof.authority.tenant_id).into(), (&proof.authority.actor_id).into(), proof.generation.into(), (&proof.nonce).into(), (&proof.fingerprint).into()]).await?;
        if changed != 1 {
            return Err(Error::Conflict);
        }
        let actual = decode(row(&tx, "SELECT * FROM tenant_workflow_receipts WHERE id=$1 AND tenant_id=$2 AND actor_id=$3", vec![(&proof.receipt.id).into(), (&proof.authority.tenant_id).into(), (&proof.authority.actor_id).into()]).await?.ok_or(Error::Corrupt)?)?;
        tx.commit().await?;
        // Recording that a claimed effect is uncertain isn't authority to
        // disclose private task data after the actor's access was revoked.
        if authority_lost {
            return Err(Error::Forbidden);
        }
        Ok(actual.receipt)
    }

    pub(crate) async fn get(&self, authority: &Authority, id: &str) -> Result<Receipt, Error> {
        if Uuid::parse_str(id).is_err() {
            return Err(Error::Invalid);
        }
        let (tx, now) = self.transaction(authority, true).await?;
        let saved = row(
            &tx,
            "SELECT * FROM tenant_workflow_receipts WHERE id=$1 AND tenant_id=$2",
            vec![id.into(), (&authority.tenant_id).into()],
        )
        .await?
        .ok_or(Error::NotFound)?;
        let generation: i64 = field(&saved, "generation")?;
        let nonce: Option<String> = field(&saved, "lease")?;
        let expires: Option<i64> = field(&saved, "lease_expires_at")?;
        let mut actual = decode(saved)?;
        if actual.receipt.phase == StoredPhase::Dispatching && expires.ok_or(Error::Corrupt)? <= now
        {
            if generation != 1 || nonce.is_none() {
                return Err(Error::Corrupt);
            }
            let changed = execute(&tx, "UPDATE tenant_workflow_receipts SET phase='outcome_unknown',generation=generation+1,updated_at=$1,output=NULL,error='lease_expired' WHERE id=$2 AND tenant_id=$3 AND phase='dispatching' AND generation=1 AND lease=$4 AND lease_expires_at<=$1", vec![now.into(), id.into(), (&authority.tenant_id).into(), nonce.into()]).await?;
            if changed != 1 {
                return Err(Error::Conflict);
            }
            actual = decode(
                row(
                    &tx,
                    "SELECT * FROM tenant_workflow_receipts WHERE id=$1 AND tenant_id=$2",
                    vec![id.into(), (&authority.tenant_id).into()],
                )
                .await?
                .ok_or(Error::Corrupt)?,
            )?;
        }
        tx.commit().await?;
        Ok(actual.receipt)
    }

    pub(crate) async fn claim(
        &self,
        reservation: Reservation,
    ) -> Result<Option<DispatchLease>, Error> {
        let (tx, now) = self.transaction(&reservation.authority, true).await?;
        let Some(admitted) = reservation.admitted else {
            tx.commit().await?;
            return Ok(None);
        };
        let prior = decode(row(&tx, "SELECT * FROM tenant_workflow_receipts WHERE id=$1 AND tenant_id=$2 AND actor_id=$3", vec![(&reservation.receipt.id).into(), (&admitted.authority.tenant_id).into(), (&admitted.authority.actor_id).into()]).await?.ok_or(Error::NotFound)?)?;
        if prior.receipt != reservation.receipt {
            return Err(Error::Conflict);
        }
        let nonce = Uuid::new_v4().to_string();
        let changed = execute(&tx, "UPDATE tenant_workflow_receipts SET phase='dispatching',generation=generation+1,lease=$1,lease_expires_at=$2,updated_at=$3 WHERE id=$4 AND tenant_id=$5 AND actor_id=$6 AND fingerprint=$7 AND phase='queued' AND generation=0", vec![(&nonce).into(), (now+120).into(), now.into(), (&prior.receipt.id).into(), (&admitted.authority.tenant_id).into(), (&admitted.authority.actor_id).into(), prior.fingerprint.into()]).await?;
        if changed != 1 {
            tx.commit().await?;
            return Ok(None);
        }
        let actual = decode(row(&tx, "SELECT * FROM tenant_workflow_receipts WHERE id=$1 AND tenant_id=$2 AND actor_id=$3", vec![prior.receipt.id.into(), (&admitted.authority.tenant_id).into(), (&admitted.authority.actor_id).into()]).await?.ok_or(Error::Corrupt)?)?;
        tx.commit().await?;
        Ok(Some(DispatchLease {
            receipt: actual.receipt,
            admitted,
            nonce,
            fingerprint: actual.fingerprint,
        }))
    }
}
