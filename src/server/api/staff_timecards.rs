//! Canonical, atomic clock effects. A receipt is stored on the effect itself;
//! neither an accepted intent nor a lost COMMIT reply proves a clock was saved.
use crate::db::{DB, DbStore};
use axum::{
    Json,
    extract::{Extension, OriginalUri, Path},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use server_auth::commit_authority::{
    AuthorityError, AuthorizedPgOwner, AuthorizedSqliteOwner, CanonicalPgAuthority,
    CanonicalSqliteAuthority,
};
use std::{collections::HashSet, sync::Arc};

use super::sync_transaction::{SyncError, commit_owner, commit_sqlite_owner};

const ROUTE: &str = "/api/v1/staff/timecard";

#[derive(Clone)]
pub struct TimecardAccess {
    store: Arc<server_auth::Store>,
    postgres: Option<CanonicalPgAuthority>,
    sqlite: Option<CanonicalSqliteAuthority>,
}
impl TimecardAccess {
    /// Bind only to the canonical identity store's actual business relations.
    /// A copied tenant or equal database URL is not an authority boundary.
    pub async fn configured(db: &DB, store: Arc<server_auth::Store>) -> Self {
        let mut access = Self {
            store: store.clone(),
            postgres: None,
            sqlite: None,
        };
        match &db.store {
            DbStore::Postgres => {
                if let Some(repository) = store.portable_repo() {
                    match server_auth::commit_authority::canonical_pg_data_pool(
                        repository.connection(),
                        &db.pool,
                        &["ohc_timecard_event", "ohc_staff_member"],
                    )
                    .await
                    {
                        Ok(pool) => access.postgres = CanonicalPgAuthority::bind(store, &pool).ok(),
                        Err(error) => {
                            tracing::warn!(%error, "Canonical timecard storage unavailable")
                        }
                    }
                }
            }
            DbStore::Sqlite(pool) => {
                access.sqlite = CanonicalSqliteAuthority::bind(store, pool).ok()
            }
        }
        access
    }
    async fn authorize(
        &self,
        claims: &server_common::Claims,
        headers: &HeaderMap,
    ) -> Result<Owner, Error> {
        server_auth::commit_authority::verify_owner(&self.store, claims, headers).await?;
        if let Some(authority) = &self.postgres {
            return Ok(Owner::Postgres(authority.authorize(claims, headers).await?));
        }
        if let Some(authority) = &self.sqlite {
            return Ok(Owner::Sqlite(authority.authorize(claims, headers).await?));
        }
        Err(AuthorityError::Unavailable.into())
    }
}
enum Owner {
    Postgres(AuthorizedPgOwner),
    Sqlite(AuthorizedSqliteOwner),
}
impl Owner {
    fn actor_id(&self) -> &str {
        match self {
            Self::Postgres(o) => o.actor_id(),
            Self::Sqlite(o) => o.actor_id(),
        }
    }
}

#[derive(Deserialize)]
pub struct SyncTimecardRequest {
    pub events: Vec<TimecardEventInput>,
}
#[derive(Clone, Deserialize, Serialize)]
pub struct TimecardEventInput {
    pub id: String,
    pub staff_id: String,
    pub event_type: String,
    pub offline_timestamp: String,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct ReceiptIdentity {
    version: u8,
    actor_id: String,
    id: String,
    staff_id: String,
    event_type: String,
    offline_timestamp: String,
}
struct Event {
    identity: ReceiptIdentity,
    encoded: String,
    instant: DateTime<Utc>,
}
fn safe_event_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 128
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}
fn valid_subject(id: &str) -> bool {
    !id.is_empty() && id.len() <= 512 && id.trim() == id && !id.chars().any(char::is_control)
}
fn parse_instant(raw: &str) -> Result<DateTime<Utc>, Error> {
    if raw.len() > 64
        || raw.len() < 20
        || !raw.as_bytes()[..4].iter().all(u8::is_ascii_digit)
        || raw.as_bytes()[4] != b'-'
    {
        return Err(Error::Invalid("invalid_clock_timestamp"));
    }
    // Chrono may truncate digits beyond nanoseconds. Inspect the original
    // decimal before parsing, so e.g. .1234560001 cannot silently become .123456.
    if raw.as_bytes().get(19) == Some(&b'.') {
        let digits = raw.as_bytes()[20..]
            .iter()
            .take_while(|b| b.is_ascii_digit());
        if digits
            .enumerate()
            .any(|(index, digit)| index >= 6 && *digit != b'0')
        {
            return Err(Error::Invalid(
                "clock_timestamp_requires_microsecond_precision",
            ));
        }
    }
    let instant = DateTime::parse_from_rfc3339(raw)
        .map_err(|_| Error::Invalid("invalid_clock_timestamp"))?
        .with_timezone(&Utc);
    // Chrono represents leap seconds as a nanosecond component >= 1e9.
    // Both stores accept only instants exactly representable by TIMESTAMPTZ.
    if instant.timestamp_subsec_nanos() >= 1_000_000_000
        || !instant.timestamp_subsec_nanos().is_multiple_of(1_000)
    {
        return Err(Error::Invalid(
            "clock_timestamp_requires_microsecond_precision",
        ));
    }
    Ok(instant)
}
fn validate(events: Vec<TimecardEventInput>, actor: &str) -> Result<Vec<Event>, Error> {
    if events.is_empty() || events.len() > 100 {
        return Err(Error::Invalid("clock_batch_requires_1_to_100_events"));
    }
    let mut ids = HashSet::new();
    let mut validated = Vec::with_capacity(events.len());
    for event in events {
        if !safe_event_id(&event.id) || !ids.insert(event.id.clone()) {
            return Err(Error::Invalid("invalid_or_duplicate_clock_id"));
        }
        if !valid_subject(&event.staff_id)
            || !matches!(event.event_type.as_str(), "CLOCK_IN" | "CLOCK_OUT")
        {
            return Err(Error::Invalid("invalid_clock_subject_or_type"));
        }
        let instant = parse_instant(&event.offline_timestamp)?;
        let identity = ReceiptIdentity {
            version: 1,
            actor_id: actor.into(),
            id: event.id,
            staff_id: event.staff_id,
            event_type: event.event_type,
            offline_timestamp: event.offline_timestamp,
        };
        let encoded = serde_json::to_string(&identity)
            .map_err(|_| Error::Invalid("invalid_clock_identity"))?;
        validated.push(Event {
            identity,
            encoded,
            instant,
        });
    }
    // Stable lock order prevents differently ordered batches deadlocking.
    validated.sort_unstable_by(|a, b| a.identity.id.cmp(&b.identity.id));
    Ok(validated)
}

#[derive(sqlx::FromRow)]
struct PersistedEvent {
    staff_id: String,
    event_type: String,
    event_time: DateTime<Utc>,
    request_identity: Option<String>,
}
/// Shared verification boundary for POST replay and the receipt lookup. A
/// matching request identity alone cannot acknowledge a missing/changed effect.
fn verify_effect(row: &PersistedEvent, expected: &ReceiptIdentity) -> Result<(), Error> {
    let identity: ReceiptIdentity = row
        .request_identity
        .as_deref()
        .and_then(|raw| serde_json::from_str(raw).ok())
        .ok_or(Error::Conflict)?;
    if identity != *expected
        || identity.version != 1
        || !safe_event_id(&identity.id)
        || !valid_subject(&identity.actor_id)
        || !valid_subject(&identity.staff_id)
        || !matches!(identity.event_type.as_str(), "CLOCK_IN" | "CLOCK_OUT")
        || row.staff_id != identity.staff_id
        || row.event_type != identity.event_type
        || parse_instant(&identity.offline_timestamp).map_err(|_| Error::Conflict)?
            != row.event_time
    {
        return Err(Error::Conflict);
    }
    Ok(())
}
async fn pg_subject(
    c: &mut sqlx::PgConnection,
    tenant: &str,
    event: &ReceiptIdentity,
) -> Result<(), Error> {
    if event.staff_id == event.actor_id {
        return Ok(());
    }
    let exists: Option<String> = sqlx::query_scalar(
        "SELECT id FROM ohc_staff_member WHERE tenant_id=$1 AND id=$2 FOR SHARE",
    )
    .bind(tenant)
    .bind(&event.staff_id)
    .fetch_optional(c)
    .await?;
    if exists.is_none() {
        return Err(Error::Authority(AuthorityError::Forbidden));
    }
    Ok(())
}
async fn sqlite_subject(
    c: &mut sqlx::SqliteConnection,
    tenant: &str,
    event: &ReceiptIdentity,
) -> Result<(), Error> {
    if event.staff_id == event.actor_id {
        return Ok(());
    }
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM main.ohc_staff_member WHERE tenant_id=? AND id=?)",
    )
    .bind(tenant)
    .bind(&event.staff_id)
    .fetch_one(c)
    .await?;
    if !exists {
        return Err(Error::Authority(AuthorityError::Forbidden));
    }
    Ok(())
}
async fn pg_event(
    c: &mut sqlx::PgConnection,
    tenant: &str,
    id: &str,
) -> Result<Option<PersistedEvent>, Error> {
    Ok(sqlx::query_as("SELECT staff_id,event_type,event_time,request_identity FROM ohc_timecard_event WHERE tenant_id=$1 AND id=$2 FOR UPDATE")
        .bind(tenant).bind(id).fetch_optional(c).await?)
}
async fn sqlite_event(
    c: &mut sqlx::SqliteConnection,
    tenant: &str,
    id: &str,
) -> Result<Option<PersistedEvent>, Error> {
    Ok(sqlx::query_as("SELECT staff_id,event_type,event_time,request_identity FROM main.ohc_timecard_event WHERE tenant_id=? AND id=?")
        .bind(tenant).bind(id).fetch_optional(c).await?)
}
async fn persist(owner: Owner, events: &[Event]) -> Result<(), Error> {
    match owner {
        Owner::Postgres(owner) => {
            let tenant = owner.tenant_id().to_owned();
            let mut tx = owner.begin().await?;
            for event in events {
                sqlx::query("SELECT pg_catalog.pg_advisory_xact_lock(pg_catalog.hashtextextended(pg_catalog.jsonb_build_array('ohc-timecard-v1',$1::text,$2::text)::text,0))")
                    .bind(&tenant).bind(&event.identity.id).execute(tx.connection()).await?;
                pg_subject(tx.connection(), &tenant, &event.identity).await?;
                match pg_event(tx.connection(), &tenant, &event.identity.id).await? {
                    Some(row) => verify_effect(&row, &event.identity)?,
                    None => {
                        sqlx::query("INSERT INTO ohc_timecard_event(id,tenant_id,staff_id,event_type,event_time,request_identity) VALUES($1,$2,$3,$4,$5,$6)")
                            .bind(&event.identity.id).bind(&tenant).bind(&event.identity.staff_id).bind(&event.identity.event_type)
                            .bind(event.instant).bind(&event.encoded).execute(tx.connection()).await.map_err(insert_error)?;
                    }
                }
            }
            // Flush deferred constraints/triggers before certifying effects.
            // Only reads and the canonical authority fence follow this point.
            sqlx::query("SET CONSTRAINTS ALL IMMEDIATE")
                .execute(tx.connection())
                .await?;
            for event in events {
                pg_subject(tx.connection(), &tenant, &event.identity).await?;
                verify_effect(
                    &pg_event(tx.connection(), &tenant, &event.identity.id)
                        .await?
                        .ok_or(Error::Conflict)?,
                    &event.identity,
                )?;
            }
            commit_owner(tx).await.map_err(Error::Commit)?;
        }
        Owner::Sqlite(owner) => {
            let tenant = owner.tenant_id().to_owned();
            let mut tx = owner.begin().await?;
            for event in events {
                sqlite_subject(tx.connection(), &tenant, &event.identity).await?;
                match sqlite_event(tx.connection(), &tenant, &event.identity.id).await? {
                    Some(row) => verify_effect(&row, &event.identity)?,
                    None => {
                        sqlx::query("INSERT INTO main.ohc_timecard_event(id,tenant_id,staff_id,event_type,event_time,request_identity) VALUES(?,?,?,?,?,?)")
                            .bind(&event.identity.id).bind(&tenant).bind(&event.identity.staff_id).bind(&event.identity.event_type)
                            .bind(&event.identity.offline_timestamp).bind(&event.encoded).execute(tx.connection()).await.map_err(insert_error)?;
                    }
                }
            }
            for event in events {
                sqlite_subject(tx.connection(), &tenant, &event.identity).await?;
                verify_effect(
                    &sqlite_event(tx.connection(), &tenant, &event.identity.id)
                        .await?
                        .ok_or(Error::Conflict)?,
                    &event.identity,
                )?;
            }
            commit_sqlite_owner(tx).await.map_err(Error::Commit)?;
        }
    }
    Ok(())
}
fn insert_error(error: sqlx::Error) -> Error {
    if error
        .as_database_error()
        .is_some_and(|db| db.is_unique_violation())
    {
        Error::Conflict
    } else {
        Error::Database(error)
    }
}

#[derive(Debug)]
enum Error {
    Invalid(&'static str),
    Conflict,
    NotFound,
    Authority(AuthorityError),
    Database(sqlx::Error),
    Commit(SyncError),
}
impl From<AuthorityError> for Error {
    fn from(e: AuthorityError) -> Self {
        Self::Authority(e)
    }
}
impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::Database(e)
    }
}
fn failure(error: Error, ids: &[String]) -> Response {
    let (code, status, reason) = match error {
        Error::Invalid(reason) => (StatusCode::BAD_REQUEST, "blocked", reason),
        Error::NotFound => (StatusCode::NOT_FOUND, "blocked", "clock_receipt_not_found"),
        Error::Conflict => (
            StatusCode::CONFLICT,
            "blocked",
            "clock_identity_or_effect_conflict",
        ),
        Error::Authority(AuthorityError::Forbidden) => (
            StatusCode::FORBIDDEN,
            "blocked",
            "current_owner_authority_required",
        ),
        Error::Commit(SyncError::Rejected(reason)) => (StatusCode::FORBIDDEN, "blocked", reason),
        Error::Commit(ref error) => (
            StatusCode::SERVICE_UNAVAILABLE,
            error.status(),
            error.reason(),
        ),
        Error::Authority(ref error) => {
            tracing::warn!(%error,"Timecard authority unavailable");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "blocked",
                "clock_storage_unavailable",
            )
        }
        Error::Database(ref error) => {
            tracing::warn!(%error,"Timecard write failed before commit");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "blocked",
                "transaction_not_committed",
            )
        }
    };
    (code, Json(serde_json::json!({"success":false,"error":reason,"outcomes":ids.iter().map(|id| serde_json::json!({"id":id,"route":ROUTE,"status":status,"reason":reason})).collect::<Vec<_>>()}))).into_response()
}
pub async fn sync_timecard_handler(
    access: Option<Extension<TimecardAccess>>,
    claims: Option<Extension<server_common::Claims>>,
    headers: HeaderMap,
    Json(payload): Json<SyncTimecardRequest>,
) -> Response {
    let ids: Vec<String> = payload
        .events
        .iter()
        .map(|event| event.id.clone())
        .collect();
    let result = async {
        let access = access.ok_or(Error::Authority(AuthorityError::Unavailable))?;
        let claims = claims.ok_or(Error::Authority(AuthorityError::Forbidden))?;
        let owner = access.authorize(&claims, &headers).await?;
        let events = validate(payload.events, owner.actor_id())?;
        persist(owner, &events).await
    }
    .await;
    let response = match result {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"success":true,"outcomes":ids.iter().map(|id| serde_json::json!({"id":id,"route":ROUTE,"status":"acknowledged"})).collect::<Vec<_>>()}))).into_response(),
        Err(error) => failure(error, &ids),
    };
    private_response(response)
}
fn private_response(mut response: Response) -> Response {
    response.headers_mut().insert(
        axum::http::header::CACHE_CONTROL,
        axum::http::HeaderValue::from_static("private, no-store"),
    );
    response
}

fn verified_receipt(row: &PersistedEvent, actor: &str, id: &str) -> Result<ReceiptIdentity, Error> {
    // NULL and unreadable historical identities cannot establish an issuer.
    // Do not disclose another actor's clock receipt, even to a tenant owner.
    let identity: ReceiptIdentity = row
        .request_identity
        .as_deref()
        .and_then(|raw| serde_json::from_str(raw).ok())
        .ok_or(Error::NotFound)?;
    if identity.actor_id != actor {
        return Err(Error::NotFound);
    }
    if identity.id != id {
        return Err(Error::Conflict);
    }
    verify_effect(row, &identity)?;
    Ok(identity)
}
async fn read_receipt(owner: Owner, id: &str) -> Result<ReceiptIdentity, Error> {
    let actor = owner.actor_id().to_owned();
    match owner {
        Owner::Postgres(owner) => {
            let tenant = owner.tenant_id().to_owned();
            let mut tx = owner.begin().await?;
            let row = pg_event(tx.connection(), &tenant, id)
                .await?
                .ok_or(Error::NotFound)?;
            let receipt = verified_receipt(&row, &actor, id)?;
            // A read cannot confirm a receipt after losing current authority.
            commit_owner(tx).await.map_err(Error::Commit)?;
            Ok(receipt)
        }
        Owner::Sqlite(owner) => {
            let tenant = owner.tenant_id().to_owned();
            let mut tx = owner.begin().await?;
            let row = sqlite_event(tx.connection(), &tenant, id)
                .await?
                .ok_or(Error::NotFound)?;
            let receipt = verified_receipt(&row, &actor, id)?;
            commit_sqlite_owner(tx).await.map_err(Error::Commit)?;
            Ok(receipt)
        }
    }
}
/// Recovery only: this endpoint never inserts, updates, backfills or resends an
/// event. The same verifier certifies the actual effect for POST and GET.
pub async fn timecard_receipt_handler(
    access: Option<Extension<TimecardAccess>>,
    claims: Option<Extension<server_common::Claims>>,
    headers: HeaderMap,
    OriginalUri(uri): OriginalUri,
    Path(id): Path<String>,
) -> Response {
    let result = async {
        let access = access.ok_or(Error::Authority(AuthorityError::Unavailable))?;
        let claims = claims.ok_or(Error::Authority(AuthorityError::Forbidden))?;
        let owner = access.authorize(&claims, &headers).await?;
        // Path<String> is decoded by Axum. Preserve the canonical raw spelling
        // too: an encoded alias of an otherwise safe ID is not a receipt URL.
        if !safe_event_id(&id) || uri.path() != format!("{ROUTE}/receipts/{id}") {
            return Err(Error::Invalid("invalid_clock_id"));
        }
        read_receipt(owner, &id).await
    }
    .await;
    let response = match result {
        Ok(receipt) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "success":true,
                "outcomes":[{"id":id,"route":ROUTE,"status":"acknowledged"}],
                "receipt":receipt,
            })),
        )
            .into_response(),
        Err(error) => failure(error, &[id]),
    };
    private_response(response)
}
