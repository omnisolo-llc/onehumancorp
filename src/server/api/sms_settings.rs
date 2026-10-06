//! Verified actor-owned SMS preferences and explicit provider-acceptance receipts.
//! Every request uses the configured identity pool; no legacy global phone fallback.
use axum::{
    Json, Router,
    extract::{Extension, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use hmac::{Hmac, Mac};
use rand::Rng;
use sea_orm::ConnectionTrait;
use serde::{Deserialize, Serialize};
use server_auth::commit_authority::{
    AuthorityError, AuthorizedPgOwner, AuthorizedSqliteOwner, CanonicalPgAuthority,
    CanonicalSqliteAuthority, OwnerPgTransaction, OwnerSqliteTransaction,
};
use server_common::Claims;
use server_integrations_twilio::client::{
    MessageReceipt, MessageSendError, RealTwilioClient, TwilioClientWrapper,
};
use sha2::{Digest, Sha256};
use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicI64, Ordering},
};

#[cfg(test)]
#[path = "sms_settings_test.rs"]
mod tests;

const CODE_SECONDS: i64 = 300;
const RESEND_SECONDS: i64 = 60;
const MAX_SENDS_PER_HOUR: i64 = 5;
static GLOBAL_SERVICE: OnceLock<SmsService> = OnceLock::new();

#[derive(Clone)]
pub struct SmsService {
    store: Arc<server_auth::Store>,
    postgres: Option<CanonicalPgAuthority>,
    sqlite: Option<CanonicalSqliteAuthority>,
    provider: Arc<dyn TwilioClientWrapper>,
    from_phone: String,
    configured: bool,
    key: [u8; 32],
    order_worker_healthy_at: Arc<AtomicI64>,
    order_discovery_offset: Arc<AtomicI64>,
}
impl SmsService {
    pub fn configured(store: Arc<server_auth::Store>) -> Self {
        let sid = std::env::var("TWILIO_ACCOUNT_SID").unwrap_or_default();
        let token = std::env::var("TWILIO_AUTH_TOKEN").unwrap_or_default();
        let from = std::env::var("TWILIO_FROM_NUMBER").unwrap_or_default();
        let configured = !sid.trim().is_empty() && !token.trim().is_empty() && valid_phone(&from);
        Self::with_provider(
            store,
            Arc::new(RealTwilioClient::new(sid, token)),
            from,
            configured,
        )
    }
    fn with_provider(
        store: Arc<server_auth::Store>,
        provider: Arc<dyn TwilioClientWrapper>,
        from_phone: String,
        configured: bool,
    ) -> Self {
        let (mut postgres, mut sqlite) = (None, None);
        if let Some(repository) = store.portable_repo() {
            let connection = repository.connection();
            match connection.get_database_backend() {
                sea_orm::DatabaseBackend::Postgres => {
                    postgres = CanonicalPgAuthority::bind(
                        store.clone(),
                        connection.get_postgres_connection_pool(),
                    )
                    .ok()
                }
                sea_orm::DatabaseBackend::Sqlite => {
                    sqlite = CanonicalSqliteAuthority::bind(
                        store.clone(),
                        connection.get_sqlite_connection_pool(),
                    )
                    .ok()
                }
                // There is no canonical MySQL commit-authority implementation yet.
                sea_orm::DatabaseBackend::MySql => {}
            }
        }
        Self {
            key: store.registration_hash_key(),
            store,
            postgres,
            sqlite,
            provider,
            from_phone,
            configured,
            order_worker_healthy_at: Arc::new(AtomicI64::new(0)),
            order_discovery_offset: Arc::new(AtomicI64::new(0)),
        }
    }
    async fn authorize(&self, claims: &Claims, headers: &HeaderMap) -> Result<Owner, Failure> {
        for (header, expected) in [
            ("x-ohc-expected-user", Some(claims.sub.as_str())),
            ("x-ohc-expected-tenant", claims.organization_id.as_deref()),
        ] {
            let values = headers.get_all(header);
            let mut values = values.iter();
            if let Some(value) = values.next()
                && (value.to_str().ok() != expected || values.next().is_some())
            {
                return Err(Failure::Forbidden);
            }
        }
        // Verify even when storage binding is unavailable; never accept forged Claims.
        server_auth::commit_authority::verify_owner(&self.store, claims, headers).await?;
        if let Some(authority) = &self.postgres {
            return Ok(Owner::Postgres(authority.authorize(claims, headers).await?));
        }
        if let Some(authority) = &self.sqlite {
            return Ok(Owner::Sqlite(authority.authorize(claims, headers).await?));
        }
        Err(Failure::Unavailable)
    }
}
enum Owner {
    Postgres(AuthorizedPgOwner),
    Sqlite(AuthorizedSqliteOwner),
}
impl Owner {
    fn binding(&self) -> (String, String) {
        match self {
            Self::Postgres(o) => (o.tenant_id().into(), o.actor_id().into()),
            Self::Sqlite(o) => (o.tenant_id().into(), o.actor_id().into()),
        }
    }
    async fn begin(self) -> Result<Transaction, Failure> {
        Ok(match self {
            Self::Postgres(o) => Transaction::Postgres(o.begin().await?),
            Self::Sqlite(o) => Transaction::Sqlite(o.begin().await?),
        })
    }
}
enum Transaction {
    Postgres(OwnerPgTransaction),
    Sqlite(OwnerSqliteTransaction),
}
impl Transaction {
    async fn commit(self) -> Result<(), Failure> {
        match self {
            Self::Postgres(tx) => tx.commit().await,
            Self::Sqlite(tx) => tx.commit().await,
        }
        .map_err(|error| match error {
            AuthorityError::Forbidden => Failure::Forbidden,
            _ => Failure::Unconfirmed,
        })
    }
}
macro_rules! database {
    ($tx:expr, $connection:ident, $body:block) => {{
        match &mut $tx {
            Transaction::Postgres(transaction) => {
                let $connection = transaction.connection();
                $body
            }
            Transaction::Sqlite(transaction) => {
                let $connection = transaction.connection();
                $body
            }
        }
    }};
}
#[derive(Debug)]
enum Failure {
    Invalid(&'static str),
    Forbidden,
    Unavailable,
    Unconfirmed,
    ProviderRejected,
    RateLimited,
    VerificationRequired,
}
impl From<AuthorityError> for Failure {
    fn from(error: AuthorityError) -> Self {
        match error {
            AuthorityError::Forbidden => Self::Forbidden,
            _ => Self::Unavailable,
        }
    }
}
impl From<sqlx::Error> for Failure {
    fn from(_: sqlx::Error) -> Self {
        Self::Unavailable
    }
}
impl IntoResponse for Failure {
    fn into_response(self) -> Response {
        let (status, reason) = match self {
            Self::Invalid(reason) => (StatusCode::BAD_REQUEST, reason),
            Self::Forbidden => (StatusCode::FORBIDDEN, "current_owner_authority_required"),
            Self::Unavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "sms_storage_or_provider_unavailable",
            ),
            Self::Unconfirmed => (StatusCode::CONFLICT, "sms_outcome_requires_reconciliation"),
            Self::ProviderRejected => (StatusCode::BAD_GATEWAY, "sms_provider_rejected"),
            Self::RateLimited => (StatusCode::TOO_MANY_REQUESTS, "sms_rate_limited"),
            Self::VerificationRequired => (StatusCode::CONFLICT, "verified_phone_required"),
        };
        (
            status,
            [("cache-control", "private, no-store")],
            Json(serde_json::json!({"success":false,"error":reason})),
        )
            .into_response()
    }
}
fn response(value: impl Serialize) -> Response {
    ([("cache-control", "private, no-store")], Json(value)).into_response()
}
fn checked_acceptance(
    result: Result<MessageReceipt, MessageSendError>,
) -> Result<MessageReceipt, MessageSendError> {
    match result {
        Ok(receipt)
            if receipt.sid.len() == 34
                && (receipt.sid.starts_with("SM") || receipt.sid.starts_with("MM"))
                && receipt.sid[2..]
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit()) =>
        {
            Ok(receipt)
        }
        Ok(_) => Err(MessageSendError::UnknownOutcome {
            reason: "invalid message receipt",
        }),
        Err(error) => Err(error),
    }
}
fn valid_phone(value: &str) -> bool {
    value.starts_with('+')
        && (9..=16).contains(&value.len())
        && value.as_bytes()[1] != b'0'
        && value.as_bytes()[1..].iter().all(u8::is_ascii_digit)
}
fn code_mac(
    key: &[u8; 32],
    tenant: &str,
    actor: &str,
    challenge: &str,
    phone: &str,
    code: &str,
) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("fixed HMAC key length");
    // Length-delimited tuple prevents ambiguous concatenation across identities.
    mac.update(
        &serde_json::to_vec(&("sms-phone-proof-v1", tenant, actor, challenge, phone, code))
            .expect("string tuple"),
    );
    mac.finalize().into_bytes().to_vec()
}
fn code_matches(
    key: &[u8; 32],
    tenant: &str,
    actor: &str,
    challenge: &Challenge,
    code: &str,
) -> bool {
    let Ok(stored) = hex::decode(&challenge.code_mac) else {
        return false;
    };
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("fixed HMAC key length");
    mac.update(
        &serde_json::to_vec(&(
            "sms-phone-proof-v1",
            tenant,
            actor,
            &challenge.challenge_id,
            &challenge.phone,
            code,
        ))
        .expect("string tuple"),
    );
    mac.verify_slice(&stored).is_ok()
}
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Preferences {
    urgent_booking: bool,
    failed_payment: bool,
    new_order: bool,
}
#[derive(sqlx::FromRow)]
struct StoredPreferences {
    phone: Option<String>,
    verification_id: Option<String>,
    urgent_booking: bool,
    failed_payment: bool,
    new_order: bool,
    send_window: i64,
    send_count: i32,
    last_requested_at: i64,
}
#[derive(Clone, sqlx::FromRow)]
struct Challenge {
    challenge_id: String,
    phone: String,
    code_mac: String,
    state: String,
    provider_sid: Option<String>,
    expires_at: i64,
    attempts: i32,
}
#[derive(Serialize)]
struct PublicChallenge {
    challenge_id: String,
    phone: String,
    state: String,
    expires_at: i64,
}
#[derive(Serialize)]
struct Snapshot {
    success: bool,
    organization_id: String,
    user_id: String,
    status: &'static str,
    phone: Option<String>,
    verification_id: Option<String>,
    preferences: Preferences,
    challenge: Option<PublicChallenge>,
    provider_configured: bool,
    order_notifications_available: bool,
}
async fn preferences(
    tx: &mut Transaction,
    tenant: &str,
    actor: &str,
) -> Result<Option<StoredPreferences>, Failure> {
    Ok(database!(*tx, connection, {
        sqlx::query_as::<_,StoredPreferences>("SELECT phone,verification_id,urgent_booking,failed_payment,new_order,send_window,send_count,last_requested_at FROM sms_notification_preferences WHERE tenant_id=$1 AND actor_id=$2").bind(tenant).bind(actor).fetch_optional(connection).await?
    }))
}
async fn latest_challenge(
    tx: &mut Transaction,
    tenant: &str,
    actor: &str,
) -> Result<Option<Challenge>, Failure> {
    Ok(database!(*tx, connection, {
        sqlx::query_as::<_,Challenge>("SELECT c.challenge_id,c.phone,c.code_mac,c.state,c.provider_sid,c.created_at,c.expires_at,c.attempts FROM sms_verification_challenges c JOIN sms_notification_preferences p ON p.tenant_id=c.tenant_id AND p.actor_id=c.actor_id AND p.current_challenge_id=c.challenge_id WHERE c.tenant_id=$1 AND c.actor_id=$2").bind(tenant).bind(actor).fetch_optional(connection).await?
    }))
}
async fn snapshot(
    tx: &mut Transaction,
    tenant: &str,
    actor: &str,
    service: &SmsService,
) -> Result<Snapshot, Failure> {
    let prefs = preferences(tx, tenant, actor).await?;
    let challenge = latest_challenge(tx, tenant, actor)
        .await?
        .map(|c| PublicChallenge {
            challenge_id: c.challenge_id,
            phone: c.phone,
            state: c.state,
            expires_at: c.expires_at,
        });
    let (phone, verification_id, preferences) = match prefs {
        Some(p) => (
            p.phone,
            p.verification_id,
            Preferences {
                urgent_booking: p.urgent_booking,
                failed_payment: p.failed_payment,
                new_order: p.new_order,
            },
        ),
        None => (None, None, Preferences::default()),
    };
    let now = chrono::Utc::now().timestamp();
    let checked = service.order_worker_healthy_at.load(Ordering::SeqCst);
    let worker_ready = service.configured && checked > 0 && now.saturating_sub(checked) < 30;
    let installed = if worker_ready {
        match tx {
            Transaction::Postgres(tx) => sqlx::query_scalar::<_,bool>("SELECT EXISTS(SELECT 1 FROM pg_trigger WHERE tgrelid=to_regclass('orders') AND tgname='orders_admit_sms' AND tgenabled IN ('O','A') AND NOT tgisinternal)").fetch_one(tx.connection()).await?,
            Transaction::Sqlite(tx) => sqlx::query_scalar::<_,bool>("SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='trigger' AND name='orders_admit_sms' AND tbl_name='orders')").fetch_one(tx.connection()).await?,
        }
    } else {
        false
    };
    Ok(Snapshot {
        success: true,
        organization_id: tenant.into(),
        user_id: actor.into(),
        status: if verification_id.is_some() {
            "verified"
        } else {
            "unverified"
        },
        phone,
        verification_id,
        preferences,
        challenge,
        provider_configured: service.configured,
        order_notifications_available: installed,
    })
}
async fn lock_preferences(tx: &mut Transaction, tenant: &str, actor: &str) -> Result<(), Failure> {
    database!(*tx, connection, {
        sqlx::query("INSERT INTO sms_notification_preferences(tenant_id,actor_id) VALUES($1,$2) ON CONFLICT(tenant_id,actor_id) DO NOTHING").bind(tenant).bind(actor).execute(connection).await?;
    });
    if let Transaction::Postgres(transaction) = tx {
        sqlx::query("SELECT actor_id FROM sms_notification_preferences WHERE tenant_id=$1 AND actor_id=$2 FOR UPDATE").bind(tenant).bind(actor).fetch_one(transaction.connection()).await?;
    }
    Ok(())
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SendRequest {
    request_id: uuid::Uuid,
    phone: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfirmRequest {
    challenge_id: uuid::Uuid,
    phone: String,
    otp: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PreferenceRequest {
    verification_id: uuid::Uuid,
    phone: String,
    urgent_booking: bool,
    failed_payment: bool,
    new_order: bool,
}

pub fn router<S: Clone + Send + Sync + 'static>(service: SmsService) -> Router<S> {
    Router::new()
        .route("/api/v1/settings/sms-verify", post(send))
        .route("/api/v1/settings/sms-confirm", post(confirm))
        .route("/api/v1/settings/sms-preferences", get(read).post(update))
        .with_state(service)
}
pub fn install_global(service: SmsService) -> Result<(), String> {
    GLOBAL_SERVICE
        .set(service)
        .map_err(|_| "SMS service was already configured".into())
}
async fn read(
    State(service): State<SmsService>,
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
) -> Result<Response, Failure> {
    let owner = service.authorize(&claims, &headers).await?;
    let (tenant, actor) = owner.binding();
    let mut tx = owner.begin().await?;
    let result = snapshot(&mut tx, &tenant, &actor, &service).await?;
    tx.commit().await?;
    Ok(response(result))
}
fn accepted_receipt(tenant: &str, actor: &str, c: &Challenge) -> Response {
    response(
        serde_json::json!({"success":true,"organization_id":tenant,"user_id":actor,"status":"provider_accepted","challenge_id":c.challenge_id,"phone":c.phone,"expires_at":c.expires_at}),
    )
}
async fn send(
    State(service): State<SmsService>,
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
    Json(req): Json<SendRequest>,
) -> Result<Response, Failure> {
    if !valid_phone(&req.phone) || req.request_id.is_nil() {
        return Err(Failure::Invalid("invalid_sms_request"));
    }
    let owner = service.authorize(&claims, &headers).await?;
    let (tenant, actor) = owner.binding();
    if !service.configured {
        return Err(Failure::Unavailable);
    }
    let mut tx = owner.begin().await?;
    lock_preferences(&mut tx, &tenant, &actor).await?;
    let id = req.request_id.to_string();
    let now = chrono::Utc::now().timestamp();
    let existing = database!(tx, connection, {
        sqlx::query_as::<_,Challenge>("SELECT challenge_id,phone,code_mac,state,provider_sid,created_at,expires_at,attempts FROM sms_verification_challenges WHERE tenant_id=$1 AND actor_id=$2 AND challenge_id=$3").bind(&tenant).bind(&actor).bind(&id).fetch_optional(connection).await?
    });
    if let Some(existing) = existing {
        if existing.phone != req.phone {
            return Err(Failure::Invalid("sms_request_identity_conflict"));
        }
        tx.commit().await?;
        return if existing.state == "accepted"
            && existing.provider_sid.is_some()
            && existing.expires_at > now
        {
            Ok(accepted_receipt(&tenant, &actor, &existing))
        } else {
            Err(Failure::Unconfirmed)
        };
    }
    if latest_challenge(&mut tx, &tenant, &actor)
        .await?
        .is_some_and(|c| matches!(c.state.as_str(), "sending" | "unknown"))
    {
        return Err(Failure::Unconfirmed);
    }
    let prefs = preferences(&mut tx, &tenant, &actor)
        .await?
        .ok_or(Failure::Unavailable)?;
    if prefs.verification_id.is_some() {
        return Err(Failure::Invalid("phone_already_verified"));
    }
    let in_window = now.saturating_sub(prefs.send_window) < 3600;
    if now.saturating_sub(prefs.last_requested_at) < RESEND_SECONDS
        || in_window && i64::from(prefs.send_count) >= MAX_SENDS_PER_HOUR
    {
        return Err(Failure::RateLimited);
    }
    let code = format!("{:06}", rand::thread_rng().gen_range(0_u32..1_000_000));
    let digest = hex::encode(code_mac(
        &service.key,
        &tenant,
        &actor,
        &id,
        &req.phone,
        &code,
    ));
    // A later request retires every older code atomically with the new generation.
    database!(tx, connection, {
        sqlx::query("UPDATE sms_verification_challenges SET state='superseded',code_mac='' WHERE tenant_id=$1 AND actor_id=$2 AND state IN ('accepted','rejected')").bind(&tenant).bind(&actor).execute(connection).await?;
    });
    database!(tx, connection, {
        sqlx::query("INSERT INTO sms_verification_challenges(tenant_id,actor_id,challenge_id,phone,code_mac,state,created_at,expires_at) VALUES($1,$2,$3,$4,$5,'sending',$6,$7)").bind(&tenant).bind(&actor).bind(&id).bind(&req.phone).bind(digest).bind(now).bind(now+CODE_SECONDS).execute(connection).await?;
    });
    database!(tx, connection, {
        sqlx::query("UPDATE sms_notification_preferences SET send_window=$3,send_count=$4,last_requested_at=$5,current_challenge_id=$6 WHERE tenant_id=$1 AND actor_id=$2").bind(&tenant).bind(&actor).bind(if in_window {prefs.send_window}else{now}).bind(if in_window {prefs.send_count+1}else{1}).bind(now).bind(&id).execute(connection).await?;
    });
    tx.commit().await?;
    // The durable sending claim precedes provider I/O; cancellation/restart never resends it.
    let result = checked_acceptance(
        service
            .provider
            .send_sms(
                &req.phone,
                &service.from_phone,
                &format!("Your OmniSolo verification code is {code}"),
            )
            .await,
    );
    let (state, sid) = match &result {
        Ok(receipt) => ("accepted", Some(receipt.sid.clone())),
        Err(MessageSendError::Rejected { .. } | MessageSendError::OptedOut) => ("rejected", None),
        Err(MessageSendError::UnknownOutcome { .. }) => ("unknown", None),
    };
    let owner = service.authorize(&claims, &headers).await?;
    let mut tx = owner.begin().await?;
    let updated = database!(tx, connection, {
        sqlx::query("UPDATE sms_verification_challenges SET state=$4,provider_sid=$5 WHERE tenant_id=$1 AND actor_id=$2 AND challenge_id=$3 AND state='sending'").bind(&tenant).bind(&actor).bind(&id).bind(state).bind(&sid).execute(connection).await?.rows_affected()
    });
    if updated != 1 {
        return Err(Failure::Unconfirmed);
    }
    tx.commit().await?;
    match result {
        Ok(_) => Ok(response(
            serde_json::json!({"success":true,"organization_id":tenant,"user_id":actor,"status":"provider_accepted","challenge_id":id,"phone":req.phone,"expires_at":now+CODE_SECONDS}),
        )),
        Err(MessageSendError::Rejected { .. } | MessageSendError::OptedOut) => {
            Err(Failure::ProviderRejected)
        }
        Err(_) => Err(Failure::Unconfirmed),
    }
}
async fn confirm(
    State(service): State<SmsService>,
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
    Json(req): Json<ConfirmRequest>,
) -> Result<Response, Failure> {
    if !valid_phone(&req.phone)
        || req.otp.len() != 6
        || !req.otp.bytes().all(|b| b.is_ascii_digit())
    {
        return Err(Failure::Invalid("invalid_or_expired_code"));
    }
    let owner = service.authorize(&claims, &headers).await?;
    let (tenant, actor) = owner.binding();
    let mut tx = owner.begin().await?;
    lock_preferences(&mut tx, &tenant, &actor).await?;
    let current: bool = database!(tx, connection, {
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM sms_notification_preferences WHERE tenant_id=$1 AND actor_id=$2 AND current_challenge_id=$3 AND verification_id IS NULL)").bind(&tenant).bind(&actor).bind(req.challenge_id.to_string()).fetch_one(connection).await?
    });
    if !current {
        return Err(Failure::Invalid("invalid_or_expired_code"));
    }
    let challenge=database!(tx,connection,{sqlx::query_as::<_,Challenge>("SELECT challenge_id,phone,code_mac,state,provider_sid,created_at,expires_at,attempts FROM sms_verification_challenges WHERE tenant_id=$1 AND actor_id=$2 AND challenge_id=$3").bind(&tenant).bind(&actor).bind(req.challenge_id.to_string()).fetch_optional(connection).await?}).ok_or(Failure::Invalid("invalid_or_expired_code"))?;
    let now = chrono::Utc::now().timestamp();
    if challenge.phone != req.phone
        || challenge.state != "accepted"
        || challenge.provider_sid.is_none()
        || challenge.expires_at <= now
        || challenge.attempts >= 5
    {
        return Err(Failure::Invalid("invalid_or_expired_code"));
    }
    if !code_matches(&service.key, &tenant, &actor, &challenge, &req.otp) {
        database!(tx, connection, {
            sqlx::query("UPDATE sms_verification_challenges SET attempts=attempts+1 WHERE tenant_id=$1 AND actor_id=$2 AND challenge_id=$3 AND attempts<5").bind(&tenant).bind(&actor).bind(&challenge.challenge_id).execute(connection).await?;
        });
        tx.commit().await?;
        return Err(Failure::Invalid("invalid_or_expired_code"));
    }
    database!(tx, connection, {
        sqlx::query("UPDATE sms_verification_challenges SET state='verified',code_mac='' WHERE tenant_id=$1 AND actor_id=$2 AND challenge_id=$3 AND state='accepted'").bind(&tenant).bind(&actor).bind(&challenge.challenge_id).execute(connection).await?;
    });
    let verified = database!(tx, connection, {
        sqlx::query("UPDATE sms_notification_preferences SET phone=$3,verification_id=$4,urgent_booking=FALSE,failed_payment=FALSE,new_order=FALSE,updated_at=$5 WHERE tenant_id=$1 AND actor_id=$2 AND current_challenge_id=$4 AND verification_id IS NULL").bind(&tenant).bind(&actor).bind(&req.phone).bind(&challenge.challenge_id).bind(now).execute(connection).await?.rows_affected()
    });
    if verified != 1 {
        return Err(Failure::Unconfirmed);
    }
    let result = snapshot(&mut tx, &tenant, &actor, &service).await?;
    tx.commit().await?;
    Ok(response(result))
}
async fn update(
    State(service): State<SmsService>,
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
    Json(req): Json<PreferenceRequest>,
) -> Result<Response, Failure> {
    if !valid_phone(&req.phone) {
        return Err(Failure::Invalid("invalid_phone"));
    }
    let owner = service.authorize(&claims, &headers).await?;
    let (tenant, actor) = owner.binding();
    let mut tx = owner.begin().await?;
    lock_preferences(&mut tx, &tenant, &actor).await?;
    let changed = database!(tx, connection, {
        sqlx::query("UPDATE sms_notification_preferences SET urgent_booking=$5,failed_payment=$6,new_order=$7,updated_at=$8 WHERE tenant_id=$1 AND actor_id=$2 AND phone=$3 AND verification_id=$4 AND EXISTS(SELECT 1 FROM sms_verification_challenges c WHERE c.tenant_id=$1 AND c.actor_id=$2 AND c.challenge_id=$4 AND c.phone=$3 AND c.state='verified')").bind(&tenant).bind(&actor).bind(&req.phone).bind(req.verification_id.to_string()).bind(req.urgent_booking).bind(req.failed_payment).bind(req.new_order).bind(chrono::Utc::now().timestamp()).execute(connection).await?.rows_affected()
    });
    if changed != 1 {
        return Err(Failure::VerificationRequired);
    }
    let result = snapshot(&mut tx, &tenant, &actor, &service).await?;
    tx.commit().await?;
    Ok(response(result))
}

// Background notifications consume the same selected private records. Callers must
// supply the tenant and stable source event ID, after their business commit.
#[derive(Serialize, Debug)]
pub struct DispatchReceipt {
    pub status: &'static str,
    pub provider_message_ids: Vec<String>,
    pub skipped_recipients: usize,
}
#[derive(sqlx::FromRow)]
struct NotificationEvent {
    message: String,
    message_hash: String,
    status: String,
}
#[derive(sqlx::FromRow)]
struct Recipient {
    actor_id: String,
    phone: String,
    verification_id: String,
}
#[derive(sqlx::FromRow)]
struct Dispatch {
    actor_id: String,
    phone: String,
    verification_id: String,
    message_hash: String,
    state: String,
    provider_sid: Option<String>,
}
enum BackgroundTransaction {
    Postgres(sqlx::Transaction<'static, sqlx::Postgres>),
    Sqlite(sqlx::Transaction<'static, sqlx::Sqlite>),
}
macro_rules! background {
    ($tx:expr,$connection:ident,$body:block) => {{
        match &mut $tx {
            BackgroundTransaction::Postgres(tx) => {
                let $connection = &mut **tx;
                $body
            }
            BackgroundTransaction::Sqlite(tx) => {
                let $connection = &mut **tx;
                $body
            }
        }
    }};
}
impl BackgroundTransaction {
    async fn commit(self) -> Result<(), Failure> {
        match self {
            Self::Postgres(tx) => tx.commit().await,
            Self::Sqlite(tx) => tx.commit().await,
        }
        .map_err(|_| Failure::Unconfirmed)
    }
}
async fn lock_notification_authority(
    tx: &mut BackgroundTransaction,
    tenant: &str,
    event_type: &str,
    claim: &Dispatch,
) -> Result<bool, Failure> {
    if let BackgroundTransaction::Postgres(transaction) = tx {
        let current:Option<String>=sqlx::query_scalar("SELECT p.actor_id FROM sms_notification_preferences p JOIN users u ON u.id=p.actor_id AND u.tenant_id=p.tenant_id JOIN identity_user_roles r ON r.user_id=u.id AND r.tenant_id=u.tenant_id JOIN sms_verification_challenges c ON c.tenant_id=p.tenant_id AND c.actor_id=p.actor_id AND c.challenge_id=p.verification_id AND c.phone=p.phone WHERE p.tenant_id=$1 AND p.actor_id=$2 AND p.phone=$3 AND p.verification_id=$4 AND u.active=TRUE AND lower(r.role_name) IN ('owner','admin') AND c.state='verified' AND (($5='urgent_booking' AND p.urgent_booking) OR ($5='failed_payment' AND p.failed_payment) OR ($5='new_order' AND p.new_order)) FOR SHARE OF u,r,c FOR UPDATE OF p")
            .bind(tenant).bind(&claim.actor_id).bind(&claim.phone).bind(&claim.verification_id).bind(event_type).fetch_optional(&mut **transaction).await?;
        return Ok(current.is_some());
    }
    Ok(background!(*tx, connection, {
        sqlx::query_scalar::<_,bool>("SELECT EXISTS(SELECT 1 FROM sms_notification_preferences p JOIN users u ON u.id=p.actor_id AND u.tenant_id=p.tenant_id JOIN identity_user_roles r ON r.user_id=u.id AND r.tenant_id=u.tenant_id JOIN sms_verification_challenges c ON c.tenant_id=p.tenant_id AND c.actor_id=p.actor_id AND c.challenge_id=p.verification_id AND c.phone=p.phone WHERE p.tenant_id=$1 AND p.actor_id=$2 AND p.phone=$3 AND p.verification_id=$4 AND u.active=TRUE AND lower(r.role_name) IN ('owner','admin') AND c.state='verified' AND (($5='urgent_booking' AND p.urgent_booking) OR ($5='failed_payment' AND p.failed_payment) OR ($5='new_order' AND p.new_order)))")
            .bind(tenant).bind(&claim.actor_id).bind(&claim.phone).bind(&claim.verification_id).bind(event_type).fetch_one(connection).await?
    }))
}
async fn lock_committed_order(
    tx: &mut BackgroundTransaction,
    tenant: &str,
    event_id: &str,
) -> Result<bool, Failure> {
    // The worker has already left its routing-discovery role. Tenant and order
    // identity are rechecked under RLS; a current order lock fences deletion or
    // reassignment through the durable send claim, alongside preference locks.
    let found: Option<String> = match tx {
        BackgroundTransaction::Postgres(tx) => {
            sqlx::query_scalar("SELECT id FROM orders WHERE tenant_id=$1 AND id=$2 FOR SHARE")
                .bind(tenant)
                .bind(event_id)
                .fetch_optional(&mut **tx)
                .await?
        }
        BackgroundTransaction::Sqlite(tx) => {
            sqlx::query_scalar("SELECT id FROM orders WHERE tenant_id=$1 AND id=$2")
                .bind(tenant)
                .bind(event_id)
                .fetch_optional(&mut **tx)
                .await?
        }
    };
    Ok(found.is_some())
}
struct OrderWorkerHealth(Arc<AtomicI64>);
impl Drop for OrderWorkerHealth {
    fn drop(&mut self) {
        self.0.store(0, Ordering::SeqCst);
    }
}
impl SmsService {
    async fn background_transaction(&self, tenant: &str) -> Result<BackgroundTransaction, Failure> {
        let repository = self.store.portable_repo().ok_or(Failure::Unavailable)?;
        let connection = repository.connection();
        match connection.get_database_backend() {
            sea_orm::DatabaseBackend::Postgres => {
                let mut tx = connection.get_postgres_connection_pool().begin().await?;
                sqlx::query("SELECT pg_catalog.set_config('role','none',true),pg_catalog.set_config('app.current_tenant',$1,true)").bind(tenant).execute(&mut *tx).await?;
                sqlx::query("SET LOCAL statement_timeout='3000ms'")
                    .execute(&mut *tx)
                    .await?;
                Ok(BackgroundTransaction::Postgres(tx))
            }
            sea_orm::DatabaseBackend::Sqlite => {
                let mut tx = connection.get_sqlite_connection_pool().begin().await?;
                // Acquire write intent before the first read; nothing is sent on a conflict.
                sqlx::query("UPDATE sms_notification_dispatches SET state=state WHERE 0")
                    .execute(&mut *tx)
                    .await?;
                Ok(BackgroundTransaction::Sqlite(tx))
            }
            _ => Err(Failure::Unavailable),
        }
    }
    async fn order_notification_receipt(
        &self,
        tenant: &str,
        event_id: &str,
    ) -> Result<DispatchReceipt, Failure> {
        let mut tx = self.background_transaction(tenant).await?;
        if !lock_committed_order(&mut tx, tenant, event_id).await? {
            return Err(Failure::Invalid("persistent_order_receipt_required"));
        }
        let event = background!(tx, connection, {
            sqlx::query_as::<_,NotificationEvent>("SELECT message,message_hash,status FROM sms_notification_events WHERE tenant_id=$1 AND event_id=$2 AND event_type='new_order'").bind(tenant).bind(event_id).fetch_optional(connection).await?
        })
        .ok_or(Failure::Invalid("transactional_order_admission_required"))?;
        let states = background!(tx, connection, {
            sqlx::query_as::<_,(String,Option<String>)>("SELECT state,provider_sid FROM sms_notification_dispatches WHERE tenant_id=$1 AND event_id=$2 AND event_type='new_order'").bind(tenant).bind(event_id).fetch_all(connection).await?
        });
        tx.commit().await?;
        let ids = states
            .iter()
            .filter(|(state, _)| state == "accepted")
            .map(|(_, sid)| sid.clone().ok_or(Failure::Unconfirmed))
            .collect::<Result<Vec<_>, _>>()?;
        let skipped = states
            .iter()
            .filter(|(state, _)| state == "cancelled")
            .count();
        let has = |wanted: &str| states.iter().any(|(state, _)| state == wanted);
        let status = if event.status == "no_recipients" && states.is_empty() {
            "no_recipients"
        } else if has("sending") || has("unknown") {
            "requires_reconciliation"
        } else if has("rejected") {
            "provider_rejected"
        } else if has("prepared") {
            "queued"
        } else if !ids.is_empty() {
            "provider_accepted"
        } else if skipped > 0 {
            "no_eligible_recipients"
        } else {
            return Err(Failure::Unconfirmed);
        };
        Ok(DispatchReceipt {
            status,
            provider_message_ids: ids,
            skipped_recipients: skipped,
        })
    }
    // Discovery returns routing identities only, using the existing background
    // role. No message, phone, preference or provider effect is read under it.
    async fn discover_order_notifications(&self) -> Result<Vec<(String, String)>, Failure> {
        let repository = self.store.portable_repo().ok_or(Failure::Unavailable)?;
        let connection = repository.connection();
        let mut tx = match connection.get_database_backend() {
            sea_orm::DatabaseBackend::Postgres => {
                let mut tx = connection.get_postgres_connection_pool().begin().await?;
                sqlx::query("SET LOCAL ROLE ohc_bypassrls")
                    .execute(&mut *tx)
                    .await?;
                sqlx::query("SET LOCAL statement_timeout='3000ms'")
                    .execute(&mut *tx)
                    .await?;
                BackgroundTransaction::Postgres(tx)
            }
            sea_orm::DatabaseBackend::Sqlite => BackgroundTransaction::Sqlite(
                connection.get_sqlite_connection_pool().begin().await?,
            ),
            _ => return Err(Failure::Unavailable),
        };
        let pending = background!(tx, connection, {
            sqlx::query_as::<_,(String,String)>("SELECT e.tenant_id,e.event_id FROM sms_notification_events e JOIN orders o ON o.tenant_id=e.tenant_id AND o.id=e.event_id WHERE e.event_type='new_order' AND e.status='prepared' AND e.next_attempt_at<=$1 AND EXISTS(SELECT 1 FROM sms_notification_dispatches d WHERE d.tenant_id=e.tenant_id AND d.event_id=e.event_id AND d.event_type=e.event_type AND d.state='prepared') ORDER BY e.next_attempt_at,e.created_at,e.tenant_id,e.event_id LIMIT 32 OFFSET $2").bind(chrono::Utc::now().timestamp()).bind(self.order_discovery_offset.load(Ordering::SeqCst)).fetch_all(connection).await?
        });
        tx.commit().await?;
        Ok(pending)
    }
    async fn reserve_order_notification(
        &self,
        tenant: &str,
        event_id: &str,
    ) -> Result<bool, Failure> {
        let mut tx = self.background_transaction(tenant).await?;
        let now = chrono::Utc::now().timestamp();
        let claimed = background!(tx, connection, {
            sqlx::query("UPDATE sms_notification_events SET next_attempt_at=$3 WHERE tenant_id=$1 AND event_id=$2 AND event_type='new_order' AND status='prepared' AND next_attempt_at<=$4").bind(tenant).bind(event_id).bind(now+30).bind(now).execute(connection).await?.rows_affected()
        });
        tx.commit().await?;
        Ok(claimed == 1)
    }
    async fn drain_order_notifications(&self) -> Result<usize, Failure> {
        if !self.configured {
            return Ok(0);
        }
        let pending = self.discover_order_notifications().await?;
        let page_len = pending.len();
        let mut reservation_failed = false;
        let mut count = 0;
        let mut unavailable = false;
        for (tenant, event_id) in pending {
            // Reserve the retry interval before processing. A failed reservation
            // is isolated to its event; it never aborts other tenants' work.
            match self.reserve_order_notification(&tenant, &event_id).await {
                Ok(true) => count += 1,
                Ok(false) => continue,
                Err(_) => {
                    reservation_failed = true;
                    unavailable = true;
                    continue;
                }
            }
            // A timeout after provider I/O leaves the durable sending claim in
            // place. It is held for reconciliation, never turned into a retry.
            match tokio::time::timeout(
                std::time::Duration::from_secs(20),
                self.dispatch(&tenant, &event_id, "new_order", ""),
            )
            .await
            {
                Ok(Ok(_)) => {}
                Ok(Err(Failure::Unavailable)) | Err(_) => {
                    unavailable = true;
                    tracing::warn!(
                        "Order SMS storage/provider claim is unavailable; retry is scheduled"
                    );
                }
                Ok(Err(_)) => {
                    tracing::warn!("Order SMS remains pending or requires provider reconciliation")
                }
            }
        }
        // If an entire leading page cannot even reserve its retry schedule,
        // rotate the bounded routing scan on the next poll. Durable identities
        // and send claims stay in SQL; this cursor carries no send authority.
        if reservation_failed && page_len == 32 {
            let offset = self.order_discovery_offset.load(Ordering::SeqCst);
            self.order_discovery_offset
                .store(offset.saturating_add(32), Ordering::SeqCst);
        } else {
            self.order_discovery_offset.store(0, Ordering::SeqCst);
        }
        if unavailable {
            return Err(Failure::Unavailable);
        }
        Ok(count)
    }
    pub fn start_order_notifications(&self) -> tokio::task::JoinHandle<()> {
        let service = self.clone();
        tokio::spawn(async move {
            let _health = OrderWorkerHealth(service.order_worker_healthy_at.clone());
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            loop {
                interval.tick().await;
                match service.drain_order_notifications().await {
                    Ok(_) => service
                        .order_worker_healthy_at
                        .store(chrono::Utc::now().timestamp(), Ordering::SeqCst),
                    Err(_) => {
                        service.order_worker_healthy_at.store(0, Ordering::SeqCst);
                        tracing::warn!(
                            "Durable order SMS worker cannot access its configured outbox"
                        );
                    }
                }
            }
        })
    }
    async fn dispatch(
        &self,
        tenant: &str,
        event_id: &str,
        event_type: &str,
        message: &str,
    ) -> Result<DispatchReceipt, Failure> {
        if tenant.is_empty()
            || tenant.trim() != tenant
            || tenant.len() > 512
            || tenant.eq_ignore_ascii_case("system")
            || event_id.is_empty()
            || event_id.trim() != event_id
            || event_id.len() > 255
            || event_id.chars().any(char::is_control)
            || !["urgent_booking", "failed_payment", "new_order"].contains(&event_type)
        {
            return Err(Failure::Invalid("tenant_and_source_event_required"));
        }
        let mut tx = self.background_transaction(tenant).await?;
        if let BackgroundTransaction::Postgres(transaction) = &mut tx {
            sqlx::query("SELECT pg_catalog.pg_advisory_xact_lock(pg_catalog.hashtextextended(pg_catalog.jsonb_build_array('ohc-sms-event-v1',$1::text,$2::text,$3::text)::text,0))")
                .bind(tenant).bind(event_id).bind(event_type).execute(&mut **transaction).await?;
        }
        if event_type == "new_order" && !lock_committed_order(&mut tx, tenant, event_id).await? {
            return Err(Failure::Invalid("persistent_order_receipt_required"));
        }
        let event = background!(tx, connection, {
            sqlx::query_as::<_,NotificationEvent>("SELECT message,message_hash,status FROM sms_notification_events WHERE tenant_id=$1 AND event_id=$2 AND event_type=$3")
                .bind(tenant).bind(event_id).bind(event_type).fetch_optional(connection).await?
        });
        let event = if let Some(event) = event {
            event
        } else {
            // Only admission in the order INSERT transaction can create these.
            // A volatile event or a historical order cannot manufacture an SMS.
            if event_type == "new_order" {
                return Err(Failure::Invalid("transactional_order_admission_required"));
            }
            if message.trim().is_empty() || message.len() > 1600 {
                return Err(Failure::Invalid("invalid_sms_message"));
            }
            let hash = hex::encode(Sha256::digest(message.as_bytes()));
            let recipients = background!(tx, connection, {
                sqlx::query_as::<_,Recipient>("SELECT p.actor_id,p.phone,p.verification_id FROM sms_notification_preferences p JOIN users u ON u.id=p.actor_id AND u.tenant_id=p.tenant_id WHERE p.tenant_id=$1 AND u.active=TRUE AND p.phone IS NOT NULL AND p.verification_id IS NOT NULL AND EXISTS(SELECT 1 FROM identity_user_roles r WHERE r.user_id=u.id AND r.tenant_id=u.tenant_id AND lower(r.role_name) IN ('owner','admin')) AND EXISTS(SELECT 1 FROM sms_verification_challenges c WHERE c.tenant_id=p.tenant_id AND c.actor_id=p.actor_id AND c.challenge_id=p.verification_id AND c.phone=p.phone AND c.state='verified') AND (($2='urgent_booking' AND p.urgent_booking) OR ($2='failed_payment' AND p.failed_payment) OR ($2='new_order' AND p.new_order)) ORDER BY p.actor_id LIMIT 101")
                    .bind(tenant).bind(event_type).fetch_all(connection).await?
            });
            if recipients.len() > 100 {
                return Err(Failure::Unavailable);
            }
            let status = if recipients.is_empty() {
                "no_recipients"
            } else {
                "prepared"
            };
            // The event identity, first generated content and complete original
            // audience (including empty) settle atomically. Replays never rebind them.
            background!(tx, connection, {
                sqlx::query("INSERT INTO sms_notification_events(tenant_id,event_id,event_type,message,message_hash,status,created_at) VALUES($1,$2,$3,$4,$5,$6,$7)")
                    .bind(tenant).bind(event_id).bind(event_type).bind(message).bind(&hash).bind(status).bind(chrono::Utc::now().timestamp()).execute(connection).await?;
            });
            for recipient in recipients {
                background!(tx, connection, {
                    sqlx::query("INSERT INTO sms_notification_dispatches(tenant_id,actor_id,event_id,event_type,phone,verification_id,message_hash,state,created_at) VALUES($1,$2,$3,$4,$5,$6,$7,'prepared',$8)")
                        .bind(tenant).bind(&recipient.actor_id).bind(event_id).bind(event_type).bind(&recipient.phone).bind(&recipient.verification_id).bind(&hash).bind(chrono::Utc::now().timestamp()).execute(connection).await?;
                });
            }
            NotificationEvent {
                message: message.into(),
                message_hash: hash,
                status: status.into(),
            }
        };
        if event.status == "no_recipients" {
            tx.commit().await?;
            return Ok(DispatchReceipt {
                status: "no_recipients",
                provider_message_ids: vec![],
                skipped_recipients: 0,
            });
        }
        if event.message_hash != hex::encode(Sha256::digest(event.message.as_bytes())) {
            return Err(Failure::Unavailable);
        }
        let claims = background!(tx, connection, {
            sqlx::query_as::<_,Dispatch>("SELECT actor_id,phone,verification_id,message_hash,state,provider_sid FROM sms_notification_dispatches WHERE tenant_id=$1 AND event_id=$2 AND event_type=$3 ORDER BY CASE WHEN state='prepared' THEN 0 ELSE 1 END,actor_id LIMIT 100")
                .bind(tenant).bind(event_id).bind(event_type).fetch_all(connection).await?
        });
        if claims.is_empty()
            || claims
                .iter()
                .any(|claim| claim.message_hash != event.message_hash)
        {
            return Err(Failure::Unavailable);
        }
        tx.commit().await?;
        let mut ids = Vec::new();
        let mut skipped = 0;
        let mut failure = None;
        for claim in claims {
            if claim.state == "accepted" {
                ids.push(claim.provider_sid.ok_or(Failure::Unconfirmed)?);
                continue;
            }
            if claim.state == "cancelled" {
                skipped += 1;
                continue;
            }
            if claim.state != "prepared" {
                failure = Some(Failure::Unconfirmed);
                continue;
            }
            if !self.configured {
                return Err(Failure::Unavailable);
            }
            let mut tx = self.background_transaction(tenant).await?;
            let order_exists = event_type != "new_order"
                || lock_committed_order(&mut tx, tenant, event_id).await?;
            let eligible = order_exists
                && lock_notification_authority(&mut tx, tenant, event_type, &claim).await?;
            if !eligible {
                let cancelled = background!(tx, connection, {
                    sqlx::query("UPDATE sms_notification_dispatches SET state='cancelled' WHERE tenant_id=$1 AND actor_id=$2 AND event_id=$3 AND event_type=$4 AND state='prepared'")
                        .bind(tenant).bind(&claim.actor_id).bind(event_id).bind(event_type).execute(connection).await?.rows_affected()
                });
                if cancelled != 1 {
                    return Err(Failure::Unconfirmed);
                }
                tx.commit().await?;
                skipped += 1;
                continue;
            }
            // PG current user/role and preference row locks survive through this
            // claim's COMMIT. Concurrent demotion, deactivation or opt-out cannot
            // commit first after this eligibility snapshot has been accepted.
            let claimed = background!(tx, connection, {
                sqlx::query("UPDATE sms_notification_dispatches SET state='sending' WHERE tenant_id=$1 AND actor_id=$2 AND event_id=$3 AND event_type=$4 AND state='prepared' AND phone=$5 AND verification_id=$6 AND EXISTS(SELECT 1 FROM sms_notification_preferences p JOIN users u ON u.id=p.actor_id AND u.tenant_id=p.tenant_id WHERE p.tenant_id=$1 AND p.actor_id=$2 AND p.phone=$5 AND p.verification_id=$6 AND u.active=TRUE AND EXISTS(SELECT 1 FROM identity_user_roles r WHERE r.user_id=u.id AND r.tenant_id=u.tenant_id AND lower(r.role_name) IN ('owner','admin')) AND (($4='urgent_booking' AND p.urgent_booking) OR ($4='failed_payment' AND p.failed_payment) OR ($4='new_order' AND p.new_order)))")
                    .bind(tenant).bind(&claim.actor_id).bind(event_id).bind(event_type).bind(&claim.phone).bind(&claim.verification_id).execute(connection).await?.rows_affected()
            });
            if claimed != 1 {
                return Err(Failure::Unconfirmed);
            }
            tx.commit().await?;
            let sent = checked_acceptance(
                self.provider
                    .send_sms(&claim.phone, &self.from_phone, &event.message)
                    .await,
            );
            let (state, sid) = match &sent {
                Ok(receipt) => ("accepted", Some(receipt.sid.clone())),
                Err(MessageSendError::Rejected { .. } | MessageSendError::OptedOut) => {
                    ("rejected", None)
                }
                Err(_) => ("unknown", None),
            };
            let mut tx = self.background_transaction(tenant).await?;
            let changed = background!(tx, connection, {
                sqlx::query("UPDATE sms_notification_dispatches SET state=$5,provider_sid=$6 WHERE tenant_id=$1 AND actor_id=$2 AND event_id=$3 AND event_type=$4 AND state='sending'")
                    .bind(tenant).bind(&claim.actor_id).bind(event_id).bind(event_type).bind(state).bind(&sid).execute(connection).await?.rows_affected()
            });
            if changed != 1 {
                return Err(Failure::Unconfirmed);
            }
            tx.commit().await?;
            match sent {
                Ok(receipt) => ids.push(receipt.sid),
                Err(MessageSendError::Rejected { .. } | MessageSendError::OptedOut) => {
                    failure = Some(Failure::ProviderRejected)
                }
                Err(_) => failure = Some(Failure::Unconfirmed),
            }
        }
        let mut tx = self.background_transaction(tenant).await?;
        let totals = background!(tx, connection, {
            sqlx::query_as::<_,(i64,i64,i64,i64)>("SELECT COUNT(CASE WHEN state='prepared' THEN 1 END),COUNT(CASE WHEN state IN ('sending','unknown') THEN 1 END),COUNT(CASE WHEN state='rejected' THEN 1 END),COUNT(CASE WHEN state='accepted' THEN 1 END) FROM sms_notification_dispatches WHERE tenant_id=$1 AND event_id=$2 AND event_type=$3").bind(tenant).bind(event_id).bind(event_type).fetch_one(connection).await?
        });
        if totals.0 > 0 {
            tx.commit().await?;
            return Ok(DispatchReceipt {
                status: "queued",
                provider_message_ids: ids,
                skipped_recipients: skipped,
            });
        }
        if totals.1 > 0 {
            return Err(Failure::Unconfirmed);
        }
        if totals.2 > 0 {
            return Err(Failure::ProviderRejected);
        }
        if let Some(error) = failure {
            return Err(error);
        }
        let status = if totals.3 == 0 {
            "no_eligible_recipients"
        } else {
            "provider_accepted"
        };
        background!(tx, connection, {
            sqlx::query("UPDATE sms_notification_events SET status=$4 WHERE tenant_id=$1 AND event_id=$2 AND event_type=$3 AND status='prepared'")
                .bind(tenant).bind(event_id).bind(event_type).bind(status).execute(connection).await?;
        });
        tx.commit().await?;
        Ok(DispatchReceipt {
            status,
            provider_message_ids: ids,
            skipped_recipients: skipped,
        })
    }
}
pub async fn dispatch_critical_sms(
    tenant: &str,
    event_id: &str,
    event_type: &str,
    message: &str,
) -> Result<DispatchReceipt, String> {
    let service = GLOBAL_SERVICE
        .get()
        .ok_or_else(|| "SMS service is unavailable".to_string())?;
    service
        .dispatch(tenant, event_id, event_type, message)
        .await
        .map_err(|error| format!("SMS was not confirmed: {error:?}"))
}

/// A read-only receipt for a tenant-owned persisted order. Calling a webhook or
/// receiving an event-bus acknowledgement never creates notification authority.
pub async fn order_notification_status(
    tenant: &str,
    order_id: &str,
) -> Result<DispatchReceipt, String> {
    let service = GLOBAL_SERVICE
        .get()
        .ok_or_else(|| "SMS service is unavailable".to_string())?;
    service
        .order_notification_receipt(tenant, order_id)
        .await
        .map_err(|error| format!("Order SMS receipt is unavailable: {error:?}"))
}
