//! Durable receipt contract for admitted text-only work on the configured ORM.
//! Claims are committed before dispatch; expired or cancelled effects never retry.
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
    pub agent_role: Option<String>,
}

/// Additive list contract: full records, at most 20 plus one lookahead row.
/// Legacy identity query parameters are ignored, never used as authority.
#[derive(Default, Deserialize)]
pub(crate) struct ReceiptQuery {
    pub limit: Option<u32>,
    pub before: Option<String>,
}
impl ReceiptQuery {
    fn bounds(&self) -> Result<(i64, String, i64), Error> {
        let limit = self.limit.unwrap_or(20);
        if !(1..=20).contains(&limit) {
            return Err(Error::Invalid);
        }
        let (timestamp, id) = match &self.before {
            None => (i64::MAX, String::new()),
            Some(cursor) => {
                let (timestamp, id) = cursor.split_once(':').ok_or(Error::Invalid)?;
                let parsed = timestamp.parse::<i64>().map_err(|_| Error::Invalid)?;
                if !(0..=253_402_300_799).contains(&parsed) || parsed.to_string() != timestamp {
                    return Err(Error::Invalid);
                }
                canonical_request_uuid(id)?;
                (parsed, id.to_owned())
            }
        };
        Ok((timestamp, id, i64::from(limit) + 1))
    }
}
fn canonical_request_uuid(value: &str) -> Result<Uuid, Error> {
    let id = Uuid::parse_str(value).map_err(|_| Error::Invalid)?;
    if id.is_nil() || id.to_string() != value {
        return Err(Error::Invalid);
    }
    Ok(id)
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
    pub agent_role: Option<String>,
    pub funding: Option<super::funding::FundingPolicy>,
    pub input_token_bound: Option<i64>,
    pub prepared_request_digest: Option<String>,
    pub phase: StoredPhase,
    pub created_at: i64,
    pub updated_at: i64,
    pub output: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug)]
pub(crate) enum Error {
    Invalid,
    RequestIdentity,
    Budget,
    Forbidden,
    Conflict,
    NotFound,
    Unavailable,
    Corrupt,
    Database(sea_orm::DbErr),
}
impl Error {
    pub(crate) fn status(&self) -> axum::http::StatusCode {
        use axum::http::StatusCode;
        match self {
            Self::Invalid | Self::RequestIdentity => StatusCode::BAD_REQUEST,
            Self::Budget => StatusCode::CONFLICT,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Conflict => StatusCode::CONFLICT,
            Self::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
    pub(crate) fn message(&self) -> &'static str {
        match self {
            Self::Invalid => "Invalid workflow request identity or payload",
            Self::RequestIdentity => {
                "A single canonical non-nil Idempotency-Key UUID is required before a task can be submitted"
            }
            Self::Budget => {
                "The authorized usage budget cannot cover this request; no provider call was sent"
            }
            Self::Forbidden => "Current owner or administrator authority is required",
            Self::Conflict => {
                "The request conflicts with an existing workflow; no new work was started"
            }
            Self::NotFound => "Workflow not found",
            _ => "Durable workflow storage is unavailable; acceptance is unconfirmed",
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message())
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Database(cause) => Some(cause),
            _ => None,
        }
    }
}
impl From<super::AdmissionError> for Error {
    fn from(error: super::AdmissionError) -> Self {
        match error {
            super::AdmissionError::Invalid => Self::Invalid,
            super::AdmissionError::Forbidden => Self::Forbidden,
            super::AdmissionError::Unavailable => Self::Unavailable,
        }
    }
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
    pub(crate) fn bind_usage(&mut self, usage: super::funding::UsageTicket) {
        if let Some(input) = &mut self.admitted {
            input.execution_id = self.receipt.id.clone();
            input.usage = Some(usage);
        }
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
    pub(crate) fn into_admitted(self) -> AdmittedAnalysis {
        let mut admitted = self.admitted;
        admitted.execution_id = self.receipt.id;
        admitted
    }
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    agent_role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    funding: Option<super::funding::FundingPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    input_token_bound: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    prepared_request_digest: Option<String>,
}
impl Payload {
    fn validate(&self) -> Result<(), Error> {
        if self.version != 1
            || self.name.trim().is_empty()
            || self.name.chars().count() > 200
            || self.name.contains('\0')
            || self.agent_role.as_ref().is_some_and(|role| {
                role.trim().is_empty() || role.chars().count() > 120 || role.contains('\0')
            })
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
        if self.funding.is_some() != self.input_token_bound.is_some()
            || self.funding.is_some() != self.prepared_request_digest.is_some()
            || self.prepared_request_digest.as_ref().is_some_and(|digest| {
                digest.len() != 64
                    || !digest
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            })
            || self
                .input_token_bound
                .is_some_and(|bound| !(4096..=1_000_000).contains(&bound))
        {
            return Err(Error::Invalid);
        }
        if let Some(funding) = &self.funding {
            funding.validate(
                &super::AnalysisPolicy::new(
                    self.provider.clone(),
                    self.model.clone(),
                    self.max_output_tokens,
                )
                .map_err(|_| Error::Invalid)?,
            )?;
        }
        Ok(())
    }
    fn matches_request(&self, input: &AdmittedAnalysis, request: &RequestMetadata) -> bool {
        self.name == request.name
            && self.workflow == request.workflow
            && self.requested_model == request.requested_model
            && self.agent_role == request.agent_role
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
    let sql = if tx.get_database_backend() == sea_orm::DatabaseBackend::Postgres
        && sql.starts_with("SELECT * FROM tenant_workflow_receipts ")
    {
        format!("{sql} FOR UPDATE")
    } else {
        sql.to_owned()
    };
    Ok(tx.query_one(statement(tx, &sql, values)).await?)
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
    if payload
        .funding
        .as_ref()
        .is_some_and(|funding| funding.operator_tenant != tenant_id)
    {
        return Err(Error::Corrupt);
    }
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
    if chrono::DateTime::from_timestamp(field::<i64>(&row, "created_at")?, 0).is_none() {
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
        agent_role: payload.agent_role.clone(),
        funding: payload.funding.clone(),
        input_token_bound: payload.input_token_bound,
        prepared_request_digest: payload.prepared_request_digest.clone(),
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
    pub(crate) fn database(&self) -> &AppDatabase {
        &self.database
    }
    pub(crate) fn new(database: AppDatabase) -> Self {
        Self { database }
    }

    async fn begin_transaction(
        &self,
        authority: &Authority,
        write: bool,
    ) -> Result<(DatabaseTransaction, i64), Error> {
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
        let tx = match self.database.connection().get_database_backend() {
            sea_orm::DatabaseBackend::Sqlite => {
                let tx = self.database.connection().begin().await?;
                if write {
                    // Acquire write intent before the first SQLite read snapshot.
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
                tx
            }
            sea_orm::DatabaseBackend::Postgres => {
                // Reuse the canonical tenant context, including removal of any
                // assumed system role. A signed tenant never grants bypass RLS.
                let tx = server_auth::seaorm_store::begin_tenant_transaction(
                    self.database.connection(),
                    &authority.tenant_id,
                )
                .await
                .map_err(|_| Error::Unavailable)?;
                execute(&tx, "SET LOCAL statement_timeout = '3000ms'", vec![]).await?;
                execute(&tx, "SET LOCAL lock_timeout = '1000ms'", vec![]).await?;
                let isolation = row(&tx, "SHOW transaction_isolation", vec![])
                    .await?
                    .ok_or(Error::Unavailable)?;
                if field::<String>(&isolation, "transaction_isolation")? != "read committed" {
                    // Stale repeatable-read snapshots cannot attest to current
                    // authority after waiting. Do not silently change pool policy.
                    return Err(Error::Unavailable);
                }
                // Canonical authority precedes receipt locks. The actor lock
                // also serializes duplicate admission for the same actor.
                row(
                    &tx,
                    "SELECT id FROM users WHERE id=$1 AND tenant_id=$2 FOR UPDATE",
                    vec![(&authority.actor_id).into(), (&authority.tenant_id).into()],
                )
                .await?;
                tx.query_all(statement(&tx, "SELECT role_name FROM identity_user_roles WHERE user_id=$1 AND tenant_id=$2 ORDER BY role_name FOR SHARE", vec![(&authority.actor_id).into(), (&authority.tenant_id).into()])).await?;
                tx
            }
            sea_orm::DatabaseBackend::MySql => return Err(Error::Unavailable),
        };
        let now = Self::clock(&tx).await?;
        Ok((tx, now))
    }

    async fn clock(tx: &DatabaseTransaction) -> Result<i64, Error> {
        let sql = match tx.get_database_backend() {
            sea_orm::DatabaseBackend::Sqlite => {
                "SELECT CAST(strftime('%s','now') AS INTEGER) AS now"
            }
            sea_orm::DatabaseBackend::Postgres => {
                "SELECT floor(extract(epoch FROM clock_timestamp()))::bigint AS now"
            }
            _ => return Err(Error::Unavailable),
        };
        let value = row(tx, sql, vec![]).await?.ok_or(Error::Unavailable)?;
        field(&value, "now")
    }

    pub(super) async fn recheck_before_commit(
        tx: &DatabaseTransaction,
        authority: &Authority,
    ) -> Result<(), Error> {
        if tx.get_database_backend() == sea_orm::DatabaseBackend::Postgres {
            // Acquire only after blocked receipt writes. Canonical revocation
            // writers take the exclusive counterpart in the database trigger.
            // The fresh authority snapshot and COMMIT now share one fence.
            execute(tx, "SELECT pg_advisory_xact_lock_shared(hashtextextended(jsonb_build_array('ohc-token-fence-v1',$1::text,$2::text)::text,0))", vec![(&authority.tenant_id).into(), (&authority.token_id).into()]).await?;
            Self::require_authority(tx, authority, Self::clock(tx).await?).await?;
        }
        Ok(())
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

    pub(super) async fn transaction(
        &self,
        authority: &Authority,
        write: bool,
    ) -> Result<(DatabaseTransaction, i64), Error> {
        let (tx, now) = self.begin_transaction(authority, write).await?;
        Self::require_authority(&tx, authority, now).await?;
        Ok((tx, now))
    }

    pub(crate) async fn find_request(
        &self,
        authority: &Authority,
        task: &str,
        request: &RequestMetadata,
    ) -> Result<Option<Reservation>, Error> {
        tokio::time::timeout(std::time::Duration::from_secs(5), async {
            let (tx, _) = self.transaction(authority, true).await?;
            let prior = row(&tx, "SELECT * FROM tenant_workflow_receipts WHERE tenant_id=$1 AND actor_id=$2 AND request_id=$3", vec![(&authority.tenant_id).into(), (&authority.actor_id).into(), request.request_id.to_string().into()]).await?;
            let prior = prior.map(decode).transpose()?;
            if let Some(prior) = &prior
                && (prior.payload.task != task || prior.payload.name != request.name || prior.payload.workflow != request.workflow || prior.payload.requested_model != request.requested_model || prior.payload.agent_role != request.agent_role) { return Err(Error::Conflict); }
            Self::recheck_before_commit(&tx, authority).await?;
            tx.commit().await?;
            match prior {
                Some(prior)=>Ok(Some(Reservation {receipt:self.get(authority,&prior.receipt.id).await?,admitted:None,authority:authority.clone()})),
                None=>Ok(None),
            }
        }).await.map_err(|_| Error::Unavailable)?
    }

    pub(crate) async fn by_request_id(
        &self,
        authority: &Authority,
        request_id: &str,
    ) -> Result<Receipt, Error> {
        canonical_request_uuid(request_id)?;
        tokio::time::timeout(std::time::Duration::from_secs(5), async {
            let (tx,_) = self.transaction(authority,true).await?;
            let saved=row(&tx,"SELECT id FROM tenant_workflow_receipts WHERE tenant_id=$1 AND actor_id=$2 AND request_id=$3",vec![(&authority.tenant_id).into(),(&authority.actor_id).into(),request_id.into()]).await?.ok_or(Error::NotFound)?;
            let id:String=field(&saved,"id")?;
            Self::recheck_before_commit(&tx,authority).await?;
            tx.commit().await?;
            self.get(authority,&id).await
        }).await.map_err(|_|Error::Unavailable)?
    }

    pub(crate) async fn list(
        &self,
        authority: &Authority,
        query: &ReceiptQuery,
    ) -> Result<Vec<Receipt>, Error> {
        let (before, id, limit) = query.bounds()?;
        tokio::time::timeout(std::time::Duration::from_secs(5), async {
            let (tx, now) = self.transaction(authority, true).await?;
            // Queued work has no effect. A process lost before claim cannot
            // resume the old capability. Claimed work remains uncertain.
            execute(&tx, "UPDATE tenant_workflow_receipts SET phase='cancelled',generation=1,updated_at=$1,error='cancelled' WHERE tenant_id=$2 AND phase='queued' AND created_at<=$3", vec![now.into(), (&authority.tenant_id).into(), (now-120).into()]).await?;
            execute(&tx, "UPDATE tenant_workflow_receipts SET phase='outcome_unknown',generation=2,updated_at=$1,error='lease_expired' WHERE tenant_id=$2 AND phase='dispatching' AND lease_expires_at<=$1", vec![now.into(), (&authority.tenant_id).into()]).await?;
            let rows = tx.query_all(statement(&tx,"SELECT * FROM tenant_workflow_receipts WHERE tenant_id=$1 AND (created_at<$2 OR (created_at=$2 AND id<$3)) ORDER BY created_at DESC,id DESC LIMIT $4",vec![(&authority.tenant_id).into(),before.into(),id.into(),limit.into()])).await?;
            let result=rows.into_iter().map(|r| decode(r).map(|r|r.receipt)).collect::<Result<Vec<_>,_>>()?;
            Self::recheck_before_commit(&tx,authority).await?;
            tx.commit().await?;
            Ok(result)
        }).await.map_err(|_|Error::Unavailable)?
    }

    pub(crate) async fn cancel(&self, authority: &Authority, id: &str) -> Result<Receipt, Error> {
        if Uuid::parse_str(id).is_err() {
            return Err(Error::Invalid);
        }
        tokio::time::timeout(std::time::Duration::from_secs(5), async {
            let (tx, now)=self.transaction(authority,true).await?;
            let saved=decode(row(&tx,"SELECT * FROM tenant_workflow_receipts WHERE id=$1 AND tenant_id=$2",vec![id.into(),(&authority.tenant_id).into()]).await?.ok_or(Error::NotFound)?)?;
            match saved.receipt.phase {
                StoredPhase::Queued=>{ execute(&tx,"UPDATE tenant_workflow_receipts SET phase='cancelled',generation=1,updated_at=$1,error='cancelled' WHERE id=$2 AND tenant_id=$3 AND phase='queued'",vec![now.into(),id.into(),(&authority.tenant_id).into()]).await?; }
                StoredPhase::Dispatching=>{ execute(&tx,"UPDATE tenant_workflow_receipts SET phase='outcome_unknown',generation=2,updated_at=$1,error='execution_uncertain' WHERE id=$2 AND tenant_id=$3 AND phase='dispatching'",vec![now.into(),id.into(),(&authority.tenant_id).into()]).await?; }
                _=>{}
            }
            let saved=decode(row(&tx,"SELECT * FROM tenant_workflow_receipts WHERE id=$1 AND tenant_id=$2",vec![id.into(),(&authority.tenant_id).into()]).await?.ok_or(Error::NotFound)?)?;
            Self::recheck_before_commit(&tx,authority).await?;
            tx.commit().await?;
            Ok(saved.receipt)
        }).await.map_err(|_|Error::Unavailable)?
    }

    pub(crate) async fn reserve(
        &self,
        input: AdmittedAnalysis,
        request: RequestMetadata,
    ) -> Result<Reservation, Error> {
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            self.reserve_inner(input, request),
        )
        .await
        .map_err(|_| Error::Unavailable)?
    }

    async fn reserve_inner(
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
            agent_role: request.agent_role.clone(),
            funding: input.funding.clone(),
            input_token_bound: input.input_token_bound,
            prepared_request_digest: input
                .prepared
                .as_ref()
                .map(|request| request.digest().to_owned()),
        };
        payload.validate()?;
        let authority = input.authority.clone();
        let (tx, now) = self.transaction(&authority, true).await?;
        let request_id = request.request_id.to_string();
        if let Some(prior) = row(&tx, "SELECT * FROM tenant_workflow_receipts WHERE tenant_id=$1 AND actor_id=$2 AND request_id=$3", vec![(&authority.tenant_id).into(), (&authority.actor_id).into(), (&request_id).into()]).await? {
            let prior = decode(prior)?;
            if !prior.payload.matches_request(&input, &request) { return Err(Error::Conflict); }
            Self::recheck_before_commit(&tx, &authority).await?;
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
        Self::recheck_before_commit(&tx, &authority).await?;
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
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            self.finish_inner(proof, outcome),
        )
        .await
        .map_err(|_| Error::Unavailable)?
    }

    async fn finish_inner(
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
            super::AnalysisOutcome::Cancelled | super::AnalysisOutcome::BudgetUnavailable => {
                (StoredPhase::Cancelled, None, Some("cancelled"))
            }
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
        let now = if tx.get_database_backend() == sea_orm::DatabaseBackend::Postgres {
            Self::clock(&tx).await?
        } else {
            now
        };
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
            Self::recheck_before_commit(&tx, &proof.authority).await?;
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
        if !authority_lost {
            Self::recheck_before_commit(&tx, &proof.authority).await?;
            if tx.get_database_backend() == sea_orm::DatabaseBackend::Postgres
                && actual.receipt.phase == StoredPhase::Completed
                && Self::clock(&tx).await? >= expires.ok_or(Error::Corrupt)?
            {
                return Err(Error::Unavailable);
            }
        }
        tx.commit().await?;
        // Recording that a claimed effect is uncertain isn't authority to
        // disclose private task data after the actor's access was revoked.
        if authority_lost {
            return Err(Error::Forbidden);
        }
        Ok(actual.receipt)
    }

    pub(crate) async fn get(&self, authority: &Authority, id: &str) -> Result<Receipt, Error> {
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            self.get_inner(authority, id),
        )
        .await
        .map_err(|_| Error::Unavailable)?
    }

    async fn get_inner(&self, authority: &Authority, id: &str) -> Result<Receipt, Error> {
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
        let now = if tx.get_database_backend() == sea_orm::DatabaseBackend::Postgres {
            Self::clock(&tx).await?
        } else {
            now
        };
        let generation: i64 = field(&saved, "generation")?;
        let nonce: Option<String> = field(&saved, "lease")?;
        let expires: Option<i64> = field(&saved, "lease_expires_at")?;
        let mut actual = decode(saved)?;
        Self::require_authority(&tx, authority, now).await?;
        if actual.receipt.phase == StoredPhase::Queued && actual.receipt.created_at <= now - 120 {
            execute(&tx,"UPDATE tenant_workflow_receipts SET phase='cancelled',generation=1,updated_at=$1,error='cancelled' WHERE id=$2 AND tenant_id=$3 AND phase='queued'",vec![now.into(),id.into(),(&authority.tenant_id).into()]).await?;
            actual = decode(
                row(
                    &tx,
                    "SELECT * FROM tenant_workflow_receipts WHERE id=$1 AND tenant_id=$2",
                    vec![id.into(), (&authority.tenant_id).into()],
                )
                .await?
                .ok_or(Error::Corrupt)?,
            )?;
        } else if actual.receipt.phase == StoredPhase::Dispatching
            && expires.ok_or(Error::Corrupt)? <= now
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
        Self::recheck_before_commit(&tx, authority).await?;
        tx.commit().await?;
        Ok(actual.receipt)
    }

    pub(crate) async fn claim(
        &self,
        reservation: Reservation,
    ) -> Result<Option<DispatchLease>, Error> {
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            self.claim_inner(reservation),
        )
        .await
        .map_err(|_| Error::Unavailable)?
    }

    async fn claim_inner(&self, reservation: Reservation) -> Result<Option<DispatchLease>, Error> {
        let (tx, now) = self.transaction(&reservation.authority, true).await?;
        let Some(admitted) = reservation.admitted else {
            tx.commit().await?;
            return Ok(None);
        };
        let prior = decode(row(&tx, "SELECT * FROM tenant_workflow_receipts WHERE id=$1 AND tenant_id=$2 AND actor_id=$3", vec![(&reservation.receipt.id).into(), (&admitted.authority.tenant_id).into(), (&admitted.authority.actor_id).into()]).await?.ok_or(Error::NotFound)?)?;
        if prior.receipt != reservation.receipt {
            return Err(Error::Conflict);
        }
        let now = if tx.get_database_backend() == sea_orm::DatabaseBackend::Postgres {
            let now = Self::clock(&tx).await?;
            Self::require_authority(&tx, &admitted.authority, now).await?;
            now
        } else {
            now
        };
        let nonce = Uuid::new_v4().to_string();
        let changed = execute(&tx, "UPDATE tenant_workflow_receipts SET phase='dispatching',generation=generation+1,lease=$1,lease_expires_at=$2,updated_at=$3 WHERE id=$4 AND tenant_id=$5 AND actor_id=$6 AND fingerprint=$7 AND phase='queued' AND generation=0", vec![(&nonce).into(), (now+120).into(), now.into(), (&prior.receipt.id).into(), (&admitted.authority.tenant_id).into(), (&admitted.authority.actor_id).into(), prior.fingerprint.into()]).await?;
        if changed != 1 {
            tx.commit().await?;
            return Ok(None);
        }
        let actual = decode(row(&tx, "SELECT * FROM tenant_workflow_receipts WHERE id=$1 AND tenant_id=$2 AND actor_id=$3", vec![prior.receipt.id.into(), (&admitted.authority.tenant_id).into(), (&admitted.authority.actor_id).into()]).await?.ok_or(Error::Corrupt)?)?;
        Self::recheck_before_commit(&tx, &admitted.authority).await?;
        tx.commit().await?;
        Ok(Some(DispatchLease {
            receipt: actual.receipt,
            admitted,
            nonce,
            fingerprint: actual.fingerprint,
        }))
    }
}
