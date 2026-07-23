use crate::{
    AuthenticationError, LogoutError, MAX_ACCESS_TOKEN_BYTES, MAX_AUTH_EMAIL_BYTES,
    MAX_AUTH_ORGANIZATION_BYTES, MAX_AUTH_ROLE_BYTES, MAX_AUTH_ROLES, MAX_AUTH_USER_ID_BYTES,
    MAX_AUTH_USERNAME_BYTES, Store,
};
use axum::{
    Router,
    body::{Body, to_bytes},
    extract::{ConnectInfo, State},
    http::{HeaderMap, StatusCode, header},
    response::Response,
    routing::post,
    Extension,
};
use base64::Engine as _;
use hmac::{Hmac, Mac};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    net::{IpAddr, SocketAddr},
    sync::{Arc, Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

const MAX_BODY_BYTES: usize = 4096;
const MAX_IDENTIFIER_BYTES: usize = 254;
const MAX_PASSWORD_BYTES: usize = 1024;
const MAX_ORGANIZATION_BYTES: usize = MAX_AUTH_ORGANIZATION_BYTES;
const MAX_USER_ID_BYTES: usize = MAX_AUTH_USER_ID_BYTES;
const MAX_USERNAME_BYTES: usize = MAX_AUTH_USERNAME_BYTES;
const MAX_EMAIL_BYTES: usize = MAX_AUTH_EMAIL_BYTES;
const MAX_ROLES: usize = MAX_AUTH_ROLES;
const MAX_ROLE_BYTES: usize = MAX_AUTH_ROLE_BYTES;
const MAX_RESPONSE_BYTES: usize = 8192;
const RATE_WINDOW_SECONDS: u64 = 300;
const SOURCE_ATTEMPTS: u32 = 5;
const ACCOUNT_ATTEMPTS: u32 = 20;
const MAX_RATE_ENTRIES: usize = 4096;
const MAX_TRUSTED_PROXIES: usize = 32;

type HmacSha256 = Hmac<Sha256>;
type Clock = dyn Fn() -> u64 + Send + Sync;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AuditSource {
    DirectPeer,
    TrustedProxy,
    Unclassified,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AuditEvent {
    LoginSuccess,
    LoginDenied,
    LoginThrottled,
    LoginUnavailable,
    LogoutSuccess,
    LogoutDenied,
    LogoutUnavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RequestSource {
    ip: IpAddr,
    class: AuditSource,
}

fn new_audit_id() -> String {
    let mut bytes = [0_u8; 16];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}

fn emit_audit(event: AuditEvent, request_id: &str, source: AuditSource) {
    match event {
        AuditEvent::LoginSuccess | AuditEvent::LogoutSuccess => {
            tracing::info!(event = ?event, request_id, source = ?source);
        }
        AuditEvent::LoginDenied | AuditEvent::LoginThrottled | AuditEvent::LogoutDenied => {
            tracing::warn!(event = ?event, request_id, source = ?source);
        }
        AuditEvent::LoginUnavailable | AuditEvent::LogoutUnavailable => {
            tracing::error!(event = ?event, request_id, source = ?source);
        }
    }
}

#[derive(Clone)]
struct HttpAuthState {
    store: Arc<Store>,
    limiter: Arc<Mutex<LoginLimiter>>,
    hash_key: [u8; 32],
    trusted_proxies: Arc<HashSet<IpAddr>>,
    now: Arc<Clock>,
    cloud: bool,
}

#[derive(Clone, Copy)]
struct LimitConfig {
    source_attempts: u32,
    account_attempts: u32,
    window_seconds: u64,
    max_entries: usize,
}

#[derive(Clone, Copy)]
struct Bucket {
    started_at: u64,
    last_seen: u64,
    attempts: u32,
}

struct LoginLimiter {
    sources: Buckets,
    accounts: Buckets,
    config: LimitConfig,
}

struct Buckets {
    entries: HashMap<[u8; 32], Bucket>,
    expirations: BTreeSet<(u64, [u8; 32])>,
    least_recent: BTreeSet<(u64, [u8; 32])>,
}

impl Buckets {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
            expirations: BTreeSet::new(),
            least_recent: BTreeSet::new(),
        }
    }

    fn expire(&mut self, now: u64, window: u64) {
        while let Some((expires_at, key)) = self.expirations.first().copied() {
            if expires_at > now {
                break;
            }
            self.expirations.remove(&(expires_at, key));
            if let Some(bucket) = self.entries.remove(&key) {
                self.least_recent.remove(&(bucket.last_seen, key));
                debug_assert_eq!(bucket.started_at.saturating_add(window), expires_at);
            }
        }
    }

    fn retry_after(&self, key: &[u8; 32], now: u64, window: u64, limit: u32) -> Option<u64> {
        let bucket = self.entries.get(key)?;
        (bucket.attempts >= limit).then(|| {
            window
                .saturating_sub(now.saturating_sub(bucket.started_at))
                .max(1)
        })
    }

    fn saturation_retry(&self, key: &[u8; 32], now: u64, window: u64, max: usize) -> Option<u64> {
        if max > 0 && (self.entries.contains_key(key) || self.entries.len() < max) {
            return None;
        }
        self.expirations
            .first()
            .map(|(expires_at, _)| expires_at.saturating_sub(now).max(1))
            .or(Some(window.max(1)))
    }

    fn record(&mut self, key: [u8; 32], now: u64, window: u64) {
        if let Some(bucket) = self.entries.get_mut(&key) {
            self.least_recent.remove(&(bucket.last_seen, key));
            bucket.attempts = bucket.attempts.saturating_add(1);
            bucket.last_seen = now;
            self.least_recent.insert((now, key));
            return;
        }
        let bucket = Bucket {
            started_at: now,
            last_seen: now,
            attempts: 1,
        };
        self.entries.insert(key, bucket);
        self.expirations.insert((now.saturating_add(window), key));
        self.least_recent.insert((now, key));
    }
}

impl LoginLimiter {
    fn new(config: LimitConfig) -> Self {
        Self {
            sources: Buckets::new(),
            accounts: Buckets::new(),
            config,
        }
    }

    fn check_and_record(
        &mut self,
        source: [u8; 32],
        account: [u8; 32],
        now: u64,
    ) -> Result<(), u64> {
        self.sources.expire(now, self.config.window_seconds);
        self.accounts.expire(now, self.config.window_seconds);

        if let Some(retry) = self.sources.retry_after(
            &source,
            now,
            self.config.window_seconds,
            self.config.source_attempts,
        ) {
            return Err(retry);
        }
        if let Some(retry) = self.accounts.retry_after(
            &account,
            now,
            self.config.window_seconds,
            self.config.account_attempts,
        ) {
            return Err(retry);
        }

        let saturation_retry = self
            .sources
            .saturation_retry(
                &source,
                now,
                self.config.window_seconds,
                self.config.max_entries,
            )
            .into_iter()
            .chain(self.accounts.saturation_retry(
                &account,
                now,
                self.config.window_seconds,
                self.config.max_entries,
            ))
            .max();
        if let Some(retry) = saturation_retry {
            return Err(retry);
        }
        self.sources.record(source, now, self.config.window_seconds);
        self.accounts
            .record(account, now, self.config.window_seconds);
        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LoginRequest {
    username: Option<String>,
    email: Option<String>,
    password: String,
    organization_id: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
enum TenantResolution {
    Authenticate(String),
    DummyDeny,
}

#[derive(Debug, PartialEq, Eq)]
enum RateLimitDeployment {
    Standalone,
    SingleInstance,
    UpstreamBounded,
}

#[derive(Serialize)]
struct LoginUser {
    id: String,
    username: String,
    email: String,
    roles: Vec<String>,
    organization_id: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    expires_at: i64,
    user: LoginUser,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
}

/// Cloud uses an in-process limiter only when deployment explicitly guarantees
/// one backend instance or an upstream gateway enforces equivalent bounds.
pub fn router(store: Arc<Store>) -> Result<Router, String> {
    let deployment = rate_limit_deployment(
        std::env::var("OHC_AUTH_RATE_LIMIT_DEPLOYMENT")
            .ok()
            .as_deref(),
        ::server_config::get().multitenant,
    )
    .map_err(|error| {
        tracing::error!(event = "auth.config.invalid");
        error
    })?;
    let trusted_proxies = trusted_proxies_from_env().map_err(|_| {
        tracing::error!(event = "auth.config.invalid_trusted_proxy");
        "invalid trusted proxy configuration".to_string()
    })?;
    tracing::info!(event = "auth.config.ready", deployment = ?deployment);
    Ok(router_with_state(HttpAuthState::new(
        store,
        trusted_proxies,
    )))
}

fn router_with_state(state: HttpAuthState) -> Router {
    let settings_router = Router::new()
        .route("/keys", axum::routing::post(generate_api_key).get(list_api_keys))
        .route("/keys/{id}", axum::routing::delete(revoke_api_key))
        .layer(axum::middleware::from_fn_with_state(state.store.clone(), super::strict_bearer_auth_middleware));

    let admin_router = Router::new()
        .route("/usage", axum::routing::get(list_member_usage_analytics))
        .layer(axum::middleware::from_fn_with_state(state.store.clone(), super::strict_bearer_auth_middleware));

    Router::new()
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/logout", post(logout))
        .route("/api/v1/auth/register", post(register))
        .nest("/api/v1/settings", settings_router)
        .nest("/api/v1/ui/admin", admin_router)
        .with_state(state)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateKeyRequest {
    name: String,
}

#[derive(Serialize)]
struct CreateKeyResponse {
    raw_key: String,
    name: String,
    created_at: String,
}

#[derive(Serialize)]
struct ApiKeyMetadata {
    id: String,
    name: String,
    created_at: String,
    expires_at: Option<String>,
}

fn get_member_uuid(sub: &str) -> uuid::Uuid {
    uuid::Uuid::parse_str(sub)
        .unwrap_or_else(|_| {
            let mut bytes = [0u8; 16];
            let hash = sha2::Sha256::digest(sub.as_bytes());
            bytes.copy_from_slice(&hash[0..16]);
            // set variant 8
            bytes[6] = (bytes[6] & 0x0f) | 0x80;
            // set variant
            bytes[8] = (bytes[8] & 0x3f) | 0x80;
            uuid::Uuid::from_bytes(bytes)
        })
}

#[derive(Clone)]
pub struct InMemoryApiKey {
    pub id: String,
    pub key_hash: String,
    pub name: String,
    pub member_id: String,
    pub organization_id: String,
    pub created_at: String,
}

static IN_MEMORY_API_KEYS: OnceLock<Mutex<Vec<InMemoryApiKey>>> = OnceLock::new();

pub fn get_in_memory_keys() -> &'static Mutex<Vec<InMemoryApiKey>> {
    IN_MEMORY_API_KEYS.get_or_init(|| Mutex::new(Vec::new()))
}

async fn generate_api_key(
    Extension(claims): Extension<::server_common::Claims>,
    axum::Json(payload): axum::Json<CreateKeyRequest>,
) -> Response {
    let mut bytes = [0_u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);

    let raw_key = format!(
        "ohc_gwy_{}",
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&bytes)
    );

    let key_hash = format!("{:x}", Sha256::digest(raw_key.as_bytes()));
    let created_at = chrono::Utc::now().to_rfc3339();
    let key_id = uuid::Uuid::new_v4().to_string();
    let member_id = get_member_uuid(&claims.sub);
    let organization_id = claims.organization_id.clone().unwrap_or_default();

    let has_db = std::env::var("DATABASE_URL").is_ok() || std::env::var("OHC_DATABASE_URL").is_ok();

    if has_db {
        let pool = crate::db::get_pool();
        let insert_res = sqlx::query(
            "INSERT INTO api_keys (id, key_hash, name, member_id, organization_id) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(uuid::Uuid::parse_str(&key_id).unwrap_or_default())
        .bind(&key_hash)
        .bind(&payload.name)
        .bind(member_id)
        .bind(&organization_id)
        .execute(&pool)
        .await;

        if let Err(e) = insert_res {
            return no_store_json(
                StatusCode::INTERNAL_SERVER_ERROR,
                &serde_json::json!({ "error": e.to_string() }),
            );
        }
    }

    let mut keys = get_in_memory_keys().lock().unwrap();
    keys.push(InMemoryApiKey {
        id: key_id.clone(),
        key_hash,
        name: payload.name.clone(),
        member_id: claims.sub.clone(),
        organization_id,
        created_at: created_at.clone(),
    });

    let resp = CreateKeyResponse {
        raw_key,
        name: payload.name,
        created_at,
    };

    no_store_json(StatusCode::CREATED, &resp)
}

async fn list_api_keys(
    Extension(claims): Extension<::server_common::Claims>,
) -> Response {
    let mut api_keys = Vec::new();
    let has_db = std::env::var("DATABASE_URL").is_ok() || std::env::var("OHC_DATABASE_URL").is_ok();

    if has_db {
        let pool = crate::db::get_pool();
        let member_id = get_member_uuid(&claims.sub);
        let organization_id = claims.organization_id.clone().unwrap_or_default();

        let query_res = sqlx::query(
            "SELECT id, name, created_at, expires_at FROM api_keys WHERE member_id = $1 AND organization_id = $2"
        )
        .bind(member_id)
        .bind(&organization_id)
        .fetch_all(&pool)
        .await;

        match query_res {
            Ok(rows) => {
                for row in rows {
                    use sqlx::Row;
                    let id: uuid::Uuid = row.get(0);
                    let name: String = row.get(1);
                    let created_at: chrono::DateTime<chrono::Utc> = row.get(2);
                    let expires_at: Option<chrono::DateTime<chrono::Utc>> = row.get(3);

                    api_keys.push(ApiKeyMetadata {
                        id: id.to_string(),
                        name,
                        created_at: created_at.to_rfc3339(),
                        expires_at: expires_at.map(|dt| dt.to_rfc3339()),
                    });
                }
            }
            Err(e) => {
                return no_store_json(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &serde_json::json!({ "error": e.to_string() }),
                );
            }
        }
    } else {
        let keys = get_in_memory_keys().lock().unwrap();
        let org_id = claims.organization_id.clone().unwrap_or_default();
        for k in keys.iter() {
            if k.member_id == claims.sub && k.organization_id == org_id {
                api_keys.push(ApiKeyMetadata {
                    id: k.id.clone(),
                    name: k.name.clone(),
                    created_at: k.created_at.clone(),
                    expires_at: None,
                });
            }
        }
    }

    no_store_json(StatusCode::OK, &api_keys)
}

async fn revoke_api_key(
    Extension(claims): Extension<::server_common::Claims>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Response {
    let has_db = std::env::var("DATABASE_URL").is_ok() || std::env::var("OHC_DATABASE_URL").is_ok();

    if has_db {
        let pool = crate::db::get_pool();
        let key_uuid = match uuid::Uuid::parse_str(&id) {
            Ok(u) => u,
            Err(_) => {
                return no_store_json(
                    StatusCode::BAD_REQUEST,
                    &serde_json::json!({ "error": "invalid key id" }),
                );
            }
        };
        let member_id = get_member_uuid(&claims.sub);
        let organization_id = claims.organization_id.clone().unwrap_or_default();

        let delete_res = sqlx::query(
            "DELETE FROM api_keys WHERE id = $1 AND member_id = $2 AND organization_id = $3"
        )
        .bind(key_uuid)
        .bind(member_id)
        .bind(&organization_id)
        .execute(&pool)
        .await;

        if let Err(e) = delete_res {
            return no_store_json(
                StatusCode::INTERNAL_SERVER_ERROR,
                &serde_json::json!({ "error": e.to_string() }),
            );
        }
    }

    let mut keys = get_in_memory_keys().lock().unwrap();
    keys.retain(|k| k.id != id);

    no_store_json(StatusCode::OK, &serde_json::json!({ "ok": true }))
}

#[derive(Serialize)]
struct MemberUsageAggregate {
    username: String,
    feature: String,
    tokens_used: i32,
    computed_cost: f64,
}

#[derive(Clone)]
struct InMemoryUsageLog {
    username: String,
    feature: String,
    tokens_used: i32,
    computed_cost: f64,
    organization_id: String,
}

static IN_MEMORY_USAGE_LOGS: OnceLock<Mutex<Vec<InMemoryUsageLog>>> = OnceLock::new();

fn get_in_memory_usage_logs() -> &'static Mutex<Vec<InMemoryUsageLog>> {
    IN_MEMORY_USAGE_LOGS.get_or_init(|| Mutex::new(Vec::new()))
}

async fn list_member_usage_analytics(
    Extension(claims): Extension<::server_common::Claims>,
) -> Response {
    if !claims.roles.iter().any(|role| role.eq_ignore_ascii_case("admin")) {
        return error(StatusCode::FORBIDDEN, "admin access required");
    }

    let mut analytics = Vec::new();
    let has_db = std::env::var("DATABASE_URL").is_ok() || std::env::var("OHC_DATABASE_URL").is_ok();
    let organization_id = claims.organization_id.clone().unwrap_or_default();

    if has_db {
        let pool = crate::db::get_pool();
        let query_res = sqlx::query(
            "SELECT u.username, l.feature, SUM(l.tokens_used), CAST(SUM(l.computed_cost) AS double precision) \
             FROM user_usage_logs l \
             JOIN users u ON l.user_id = u.id \
             WHERE l.organization_id = $1 \
             GROUP BY u.username, l.feature"
        )
        .bind(&organization_id)
        .fetch_all(&pool)
        .await;

        match query_res {
            Ok(rows) => {
                for row in rows {
                    use sqlx::Row;
                    let username: String = row.get(0);
                    let feature: String = row.get(1);
                    let tokens_used_sum: i64 = row.get(2);
                    let computed_cost_sum: f64 = row.get(3);

                    analytics.push(MemberUsageAggregate {
                        username,
                        feature,
                        tokens_used: tokens_used_sum as i32,
                        computed_cost: computed_cost_sum,
                    });
                }
            }
            Err(e) => {
                return no_store_json(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &serde_json::json!({ "error": e.to_string() }),
                );
            }
        }
    } else {
        let logs = get_in_memory_usage_logs().lock().unwrap();
        for l in logs.iter() {
            if l.organization_id == organization_id {
                analytics.push(MemberUsageAggregate {
                    username: l.username.clone(),
                    feature: l.feature.clone(),
                    tokens_used: l.tokens_used,
                    computed_cost: l.computed_cost,
                });
            }
        }
    }

    no_store_json(StatusCode::OK, &analytics)
}

impl HttpAuthState {
    fn new(store: Arc<Store>, trusted_proxies: HashSet<IpAddr>) -> Self {
        let mut hash_key = [0_u8; 32];
        rand::thread_rng().fill_bytes(&mut hash_key);
        Self {
            store,
            limiter: Arc::new(Mutex::new(LoginLimiter::new(LimitConfig {
                source_attempts: SOURCE_ATTEMPTS,
                account_attempts: ACCOUNT_ATTEMPTS,
                window_seconds: RATE_WINDOW_SECONDS,
                max_entries: MAX_RATE_ENTRIES,
            }))),
            hash_key,
            trusted_proxies: Arc::new(trusted_proxies),
            now: Arc::new(|| {
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
            }),
            cloud: ::server_config::get().multitenant,
        }
    }
}

fn is_registration_enabled() -> bool {
    #[cfg(test)]
    {
        if let Ok(val) = std::env::var("OHC_REGISTRATION_ENABLED") {
            return val == "true";
        }
    }
    ::server_config::get().registration_enabled
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

async fn register(
    State(state): State<HttpAuthState>,
    headers: HeaderMap,
    request: axum::extract::Request,
) -> Response {
    if !is_registration_enabled() {
        return error(StatusCode::FORBIDDEN, "registration closed");
    }
    if !has_exact_json_content_type(&headers) {
        return error(StatusCode::UNSUPPORTED_MEDIA_TYPE, "invalid request");
    }
    let bytes = match to_bytes(request.into_body(), MAX_BODY_BYTES).await {
        Ok(bytes) => bytes,
        Err(_) => {
            return error(StatusCode::PAYLOAD_TOO_LARGE, "invalid request");
        }
    };
    let payload: RegisterRequest = match serde_json::from_slice(&bytes) {
        Ok(payload) => payload,
        Err(_) => {
            return error(StatusCode::BAD_REQUEST, "invalid request");
        }
    };
    let username = payload.username.trim();
    let email = payload.email.trim();
    if username.is_empty() || email.is_empty() || payload.password.is_empty() {
        return error(StatusCode::BAD_REQUEST, "invalid request");
    }

    let organization_id = uuid::Uuid::new_v4().to_string();

    let create_result = state
        .store
        .create_user(
            username.to_string(),
            email.to_string(),
            payload.password,
            vec![super::ROLE_ADMIN.to_string()],
            organization_id,
        )
        .await;

    let user = match create_result {
        Ok(user) => user,
        Err(err) => {
            return no_store_json(StatusCode::BAD_REQUEST, &serde_json::json!({ "error": err }));
        }
    };

    let response_user = LoginUser {
        id: user.id.clone(),
        username: user.username.clone(),
        email: user.email.clone(),
        roles: user.roles.clone(),
        organization_id: user.organization_id.clone().unwrap_or_default(),
    };

    let (token, expires_at) = match state.store.issue_token_with_expiry(&user) {
        Ok(result) => result,
        Err(_) => {
            return error(
                StatusCode::SERVICE_UNAVAILABLE,
                "authentication unavailable",
            );
        }
    };

    let response = LoginResponse {
        token,
        expires_at,
        user: response_user,
    };

    no_store_json(StatusCode::CREATED, &response)
}

async fn login(
    State(state): State<HttpAuthState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    request: axum::extract::Request,
) -> Response {
    let request_id = new_audit_id();
    if !has_exact_json_content_type(&headers) {
        emit_audit(
            AuditEvent::LoginDenied,
            &request_id,
            AuditSource::Unclassified,
        );
        return error(StatusCode::UNSUPPORTED_MEDIA_TYPE, "invalid request");
    }
    let source = match request_source(&headers, peer.ip(), &state.trusted_proxies) {
        Ok(source) => source,
        Err(()) => {
            emit_audit(
                AuditEvent::LoginDenied,
                &request_id,
                AuditSource::Unclassified,
            );
            return error(StatusCode::BAD_REQUEST, "invalid request");
        }
    };
    let bytes = match to_bytes(request.into_body(), MAX_BODY_BYTES).await {
        Ok(bytes) => bytes,
        Err(_) => {
            emit_audit(AuditEvent::LoginDenied, &request_id, source.class);
            return error(StatusCode::PAYLOAD_TOO_LARGE, "invalid request");
        }
    };
    let payload: LoginRequest = match serde_json::from_slice(&bytes) {
        Ok(payload) => payload,
        Err(_) => {
            emit_audit(AuditEvent::LoginDenied, &request_id, source.class);
            return error(StatusCode::BAD_REQUEST, "invalid request");
        }
    };
    let identifier = match (&payload.username, &payload.email) {
        (Some(username), None) | (None, Some(username)) => username.trim(),
        _ => {
            emit_audit(AuditEvent::LoginDenied, &request_id, source.class);
            return error(StatusCode::BAD_REQUEST, "invalid request");
        }
    };
    let default_tenant = std::env::var("OHC_DEFAULT_TENANT_ID").ok();
    let organization = resolve_organization(
        payload.organization_id.as_deref(),
        default_tenant.as_deref(),
        state.cloud,
    );
    let organization_id = match &organization {
        TenantResolution::Authenticate(value) => value.as_str(),
        TenantResolution::DummyDeny => "",
    };
    if identifier.is_empty()
        || identifier.len() > MAX_IDENTIFIER_BYTES
        || payload.password.is_empty()
        || payload.password.len() > MAX_PASSWORD_BYTES
        || organization_id.len() > MAX_ORGANIZATION_BYTES
    {
        emit_audit(AuditEvent::LoginDenied, &request_id, source.class);
        return error(StatusCode::BAD_REQUEST, "invalid request");
    }

    let source_key = keyed_hash(&state.hash_key, source.ip.to_string().as_bytes());
    let normalized_account = format!(
        "{}\0{}",
        organization_id.to_lowercase(),
        identifier.to_lowercase()
    );
    let account_key = keyed_hash(&state.hash_key, normalized_account.as_bytes());
    let retry_after = {
        let mut limiter = state.limiter.lock().expect("login limiter lock poisoned");
        limiter
            .check_and_record(source_key, account_key, (state.now)())
            .err()
    };
    if let Some(retry_after) = retry_after {
        emit_audit(AuditEvent::LoginThrottled, &request_id, source.class);
        return error_with_retry(
            StatusCode::TOO_MANY_REQUESTS,
            "too many requests",
            retry_after,
        );
    }

    let authentication = match &organization {
        TenantResolution::Authenticate(_) => {
            state
                .store
                .authenticate(identifier, &payload.password, organization_id)
                .await
        }
        TenantResolution::DummyDeny => state.store.authenticate_dummy(&payload.password).await,
    };
    let user = match authentication {
        Ok(user) => user,
        Err(AuthenticationError::InvalidCredentials) => {
            emit_audit(AuditEvent::LoginDenied, &request_id, source.class);
            return error(StatusCode::UNAUTHORIZED, "invalid credentials");
        }
        Err(AuthenticationError::Unavailable(_)) => {
            emit_audit(AuditEvent::LoginUnavailable, &request_id, source.class);
            return error(
                StatusCode::SERVICE_UNAVAILABLE,
                "authentication unavailable",
            );
        }
    };
    let response_user = LoginUser {
        id: user.id.clone(),
        username: user.username.clone(),
        email: user.email.clone(),
        roles: user.roles.clone(),
        organization_id: user.organization_id.clone().unwrap_or_default(),
    };
    if !bounded_login_user(&response_user, state.cloud) || !Store::user_access_token_fits(&user) {
        emit_audit(AuditEvent::LoginUnavailable, &request_id, source.class);
        return error(
            StatusCode::SERVICE_UNAVAILABLE,
            "authentication unavailable",
        );
    }
    let (token, expires_at) = match state.store.issue_token_with_expiry(&user) {
        Ok(result) if bounded_access_token(&result.0) => result,
        Ok(_) | Err(_) => {
            emit_audit(AuditEvent::LoginUnavailable, &request_id, source.class);
            return error(
                StatusCode::SERVICE_UNAVAILABLE,
                "authentication unavailable",
            );
        }
    };
    let response = LoginResponse {
        token,
        expires_at,
        user: response_user,
    };
    let encoded = serde_json::to_vec(&response).expect("bounded login response serializes");
    if encoded.len() > MAX_RESPONSE_BYTES {
        emit_audit(AuditEvent::LoginUnavailable, &request_id, source.class);
        return error(
            StatusCode::SERVICE_UNAVAILABLE,
            "authentication unavailable",
        );
    }
    emit_audit(AuditEvent::LoginSuccess, &request_id, source.class);
    no_store_json(StatusCode::OK, &response)
}

async fn logout(
    State(state): State<HttpAuthState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    request: axum::extract::Request,
) -> Response {
    let request_id = new_audit_id();
    let source = match request_source(&headers, peer.ip(), &state.trusted_proxies) {
        Ok(source) => source.class,
        Err(()) => {
            emit_audit(
                AuditEvent::LogoutDenied,
                &request_id,
                AuditSource::Unclassified,
            );
            return error(StatusCode::BAD_REQUEST, "invalid request");
        }
    };
    let body = match to_bytes(request.into_body(), 1).await {
        Ok(body) => body,
        Err(_) => {
            emit_audit(AuditEvent::LogoutDenied, &request_id, source);
            return error(StatusCode::BAD_REQUEST, "invalid request");
        }
    };
    if !body.is_empty() {
        emit_audit(AuditEvent::LogoutDenied, &request_id, source);
        return error(StatusCode::BAD_REQUEST, "invalid request");
    }
    let token = match bearer_token(&headers) {
        Some(token) => token.to_string(),
        None => {
            emit_audit(AuditEvent::LogoutDenied, &request_id, source);
            return error(StatusCode::UNAUTHORIZED, "invalid token");
        }
    };
    match state.store.logout_token(&token).await {
        Ok(()) => {
            emit_audit(AuditEvent::LogoutSuccess, &request_id, source);
            no_store_json(StatusCode::OK, &serde_json::json!({ "ok": true }))
        }
        Err(LogoutError::ValidationUnavailable) => {
            emit_audit(AuditEvent::LogoutUnavailable, &request_id, source);
            error(StatusCode::SERVICE_UNAVAILABLE, "logout unavailable")
        }
        Err(LogoutError::RevocationUnavailable) => {
            emit_audit(AuditEvent::LogoutUnavailable, &request_id, source);
            error(StatusCode::SERVICE_UNAVAILABLE, "logout unavailable")
        }
        Err(LogoutError::InvalidToken) => {
            emit_audit(AuditEvent::LogoutDenied, &request_id, source);
            error(StatusCode::UNAUTHORIZED, "invalid token")
        }
    }
}

fn resolve_organization(
    explicit: Option<&str>,
    default: Option<&str>,
    cloud: bool,
) -> TenantResolution {
    if let Some(value) = explicit.map(str::trim).filter(|value| !value.is_empty()) {
        return TenantResolution::Authenticate(value.to_string());
    }
    if cloud {
        return TenantResolution::DummyDeny;
    }
    TenantResolution::Authenticate(default.map(str::trim).unwrap_or_default().to_string())
}

fn rate_limit_deployment(value: Option<&str>, cloud: bool) -> Result<RateLimitDeployment, String> {
    if !cloud {
        return Ok(RateLimitDeployment::Standalone);
    }
    match value {
        Some("single-instance") => Ok(RateLimitDeployment::SingleInstance),
        Some("upstream-bounded") => Ok(RateLimitDeployment::UpstreamBounded),
        _ => Err("cloud authentication requires OHC_AUTH_RATE_LIMIT_DEPLOYMENT=single-instance or upstream-bounded".into()),
    }
}

fn bounded_login_user(user: &LoginUser, cloud: bool) -> bool {
    !user.id.is_empty()
        && user.id.len() <= MAX_USER_ID_BYTES
        && !user.username.is_empty()
        && user.username.len() <= MAX_USERNAME_BYTES
        && !user.email.is_empty()
        && user.email.len() <= MAX_EMAIL_BYTES
        && (!cloud || !user.organization_id.is_empty())
        && user.organization_id.len() <= MAX_ORGANIZATION_BYTES
        && user.roles.len() <= MAX_ROLES
        && user
            .roles
            .iter()
            .all(|role| !role.is_empty() && role.len() <= MAX_ROLE_BYTES)
}

fn bounded_access_token(token: &str) -> bool {
    !token.is_empty() && token.len() <= MAX_ACCESS_TOKEN_BYTES
}

fn has_exact_json_content_type(headers: &HeaderMap) -> bool {
    if headers.get_all(header::CONTENT_TYPE).iter().count() != 1 {
        return false;
    }
    headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.eq_ignore_ascii_case("application/json"))
}

fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    if headers.get_all(header::AUTHORIZATION).iter().count() != 1 {
        return None;
    }
    let value = headers.get(header::AUTHORIZATION)?.to_str().ok()?;
    let token = value.strip_prefix("Bearer ")?;
    (!token.is_empty()
        && token.len() <= 4096
        && !token.bytes().any(|byte| byte.is_ascii_whitespace()))
    .then_some(token)
}

fn request_source(
    headers: &HeaderMap,
    peer: IpAddr,
    trusted_proxies: &HashSet<IpAddr>,
) -> Result<RequestSource, ()> {
    let forwarded_count = headers.get_all("forwarded").iter().count();
    let xff_count = headers.get_all("x-forwarded-for").iter().count();
    if !trusted_proxies.contains(&peer) {
        return Ok(RequestSource {
            ip: peer,
            class: AuditSource::DirectPeer,
        });
    }
    if forwarded_count + xff_count == 0 {
        return Ok(RequestSource {
            ip: peer,
            class: AuditSource::DirectPeer,
        });
    }
    if forwarded_count + xff_count != 1 {
        return Err(());
    }
    if xff_count == 1 {
        let value = headers
            .get("x-forwarded-for")
            .and_then(|value| value.to_str().ok())
            .ok_or(())?;
        if value.contains(',') || value.trim() != value {
            return Err(());
        }
        return value
            .parse()
            .map(|ip| RequestSource {
                ip,
                class: AuditSource::TrustedProxy,
            })
            .map_err(|_| ());
    }
    let value = headers
        .get("forwarded")
        .and_then(|value| value.to_str().ok())
        .ok_or(())?;
    if value.contains(',') || value.contains(';') || !value.starts_with("for=") {
        return Err(());
    }
    value[4..]
        .parse()
        .map(|ip| RequestSource {
            ip,
            class: AuditSource::TrustedProxy,
        })
        .map_err(|_| ())
}

fn trusted_proxies_from_env() -> Result<HashSet<IpAddr>, String> {
    let Some(value) = std::env::var_os("OHC_AUTH_TRUSTED_PROXY_IPS") else {
        return Ok(HashSet::new());
    };
    let value = value
        .into_string()
        .map_err(|_| "trusted proxy setting is not UTF-8".to_string())?;
    if value.is_empty() {
        return Ok(HashSet::new());
    }
    let parts: Vec<_> = value.split(',').collect();
    if parts.len() > MAX_TRUSTED_PROXIES {
        return Err("too many trusted proxy addresses".to_string());
    }
    parts
        .into_iter()
        .map(|part| {
            if part.trim() != part || part.is_empty() {
                return Err("invalid trusted proxy address".to_string());
            }
            part.parse()
                .map_err(|_| "invalid trusted proxy address".to_string())
        })
        .collect()
}

fn keyed_hash(key: &[u8; 32], value: &[u8]) -> [u8; 32] {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC accepts 32-byte keys");
    mac.update(value);
    mac.finalize().into_bytes().into()
}

fn no_store_json<T: Serialize>(status: StatusCode, value: &T) -> Response {
    let body = serde_json::to_vec(value).expect("response serialization cannot fail");
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::CACHE_CONTROL, "no-store")
        .header(header::PRAGMA, "no-cache")
        .body(Body::from(body))
        .expect("valid authentication response")
}

fn error(status: StatusCode, message: &'static str) -> Response {
    no_store_json(status, &ErrorResponse { error: message })
}

fn error_with_retry(status: StatusCode, message: &'static str, retry_after: u64) -> Response {
    let mut response = error(status, message);
    response.headers_mut().insert(
        header::RETRY_AFTER,
        retry_after
            .to_string()
            .parse()
            .expect("integer retry-after is a valid header"),
    );
    response
}

#[cfg(test)]
fn router_for_test() -> Router {
    router_with_state(HttpAuthState::new(Arc::new(Store::new()), HashSet::new()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{User, user_repository::UserRepository};
    use async_trait::async_trait;
    use axum::{
        body::{Body, to_bytes},
        http::Request,
    };
    use chrono::{DateTime, Utc};
    use std::net::{Ipv4Addr, SocketAddr};
    use tower::ServiceExt;

    fn with_peer(mut request: Request<Body>, peer: IpAddr) -> Request<Body> {
        request
            .extensions_mut()
            .insert(ConnectInfo(SocketAddr::new(peer, 1234)));
        request
    }

    fn json_request(path: &str, body: impl Into<Body>) -> Request<Body> {
        with_peer(
            Request::post(path)
                .header("content-type", "application/json")
                .body(body.into())
                .unwrap(),
            IpAddr::V4(Ipv4Addr::LOCALHOST),
        )
    }

    async fn app_with_user() -> (Router, Arc<Store>, User) {
        let store = Arc::new(Store::new());
        let user = store
            .create_user(
                "Alice".into(),
                "alice@example.test".into(),
                "correct horse".into(),
                vec!["ADMIN".into()],
                "tenant-7".into(),
            )
            .await
            .unwrap();
        (
            router_with_state(HttpAuthState::new(store.clone(), HashSet::new())),
            store,
            user,
        )
    }

    #[tokio::test]
    async fn login_rejects_non_json_and_sets_no_store() {
        let request = with_peer(
            Request::post("/api/v1/auth/login")
                .header("content-type", "text/plain")
                .body(Body::from(
                    r#"{"username":"admin","password":"admin","organization_id":"tenant"}"#,
                ))
                .unwrap(),
            IpAddr::V4(Ipv4Addr::LOCALHOST),
        );

        let response = router_for_test().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
        assert_eq!(response.headers().get("cache-control").unwrap(), "no-store");
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(body.as_ref(), br#"{"error":"invalid request"}"#);
    }

    #[tokio::test]
    async fn login_accepts_email_through_store_and_returns_bounded_user_contract() {
        let (app, _, _) = app_with_user().await;
        let request = json_request(
            "/api/v1/auth/login",
            r#"{"email":"alice@example.test","password":"correct horse","organization_id":"tenant-7"}"#,
        );

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers().get("cache-control").unwrap(), "no-store");
        let body: serde_json::Value = serde_json::from_slice(
            &to_bytes(response.into_body(), MAX_BODY_BYTES)
                .await
                .unwrap(),
        )
        .unwrap();
        assert!(
            body["token"]
                .as_str()
                .is_some_and(|token| !token.is_empty())
        );
        assert_eq!(body["user"]["username"], "Alice");
        assert_eq!(body["user"]["organization_id"], "tenant-7");
    }

    #[tokio::test]
    async fn login_rejects_unknown_fields_missing_tenant_and_oversized_bodies_generically() {
        let bodies = [
            r#"{"username":"admin","password":"admin","organization_id":"tenant","extra":true}"#
                .to_string(),
            format!(
                r#"{{"username":"admin","password":"{}","organization_id":"tenant"}}"#,
                "p".repeat(MAX_PASSWORD_BYTES + 1)
            ),
        ];
        for body in bodies {
            let response = router_for_test()
                .oneshot(json_request("/api/v1/auth/login", body))
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
            let body = to_bytes(response.into_body(), 1024).await.unwrap();
            assert_eq!(body.as_ref(), br#"{"error":"invalid request"}"#);
        }

        let response = router_for_test()
            .oneshot(json_request(
                "/api/v1/auth/login",
                vec![b' '; MAX_BODY_BYTES + 1],
            ))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[tokio::test]
    async fn login_uses_the_same_generic_denial_for_unknown_user_and_wrong_password() {
        let (app, _, _) = app_with_user().await;
        for body in [
            r#"{"username":"nobody","password":"wrong","organization_id":"tenant-7"}"#,
            r#"{"username":"Alice","password":"wrong","organization_id":"tenant-7"}"#,
        ] {
            let response = app
                .clone()
                .oneshot(json_request("/api/v1/auth/login", body))
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
            let body = to_bytes(response.into_body(), 1024).await.unwrap();
            assert_eq!(body.as_ref(), br#"{"error":"invalid credentials"}"#);
        }
    }

    #[tokio::test]
    async fn cloud_missing_tenant_uses_generic_dummy_denial() {
        let store = Arc::new(Store::new());
        store
            .create_user(
                "NoTenant".into(),
                "no-tenant@example.test".into(),
                "correct horse".into(),
                vec!["ADMIN".into()],
                String::new(),
            )
            .await
            .unwrap();
        let mut state = HttpAuthState::new(store, HashSet::new());
        state.cloud = true;
        let response = router_with_state(state)
            .oneshot(json_request(
                "/api/v1/auth/login",
                r#"{"username":"NoTenant","password":"correct horse"}"#,
            ))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(body.as_ref(), br#"{"error":"invalid credentials"}"#);
    }

    #[tokio::test]
    async fn router_throttles_normalized_account_across_sources_with_retry_after() {
        let store = Arc::new(Store::new());
        let mut state = HttpAuthState::new(store, HashSet::new());
        state.limiter = Arc::new(Mutex::new(LoginLimiter::new(LimitConfig {
            source_attempts: 10,
            account_attempts: 1,
            window_seconds: 60,
            max_entries: 100,
        })));
        state.now = Arc::new(|| 100);
        let app = router_with_state(state);

        let first = with_peer(
            Request::post("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"username":" Alice ","password":"wrong","organization_id":" Tenant-7 "}"#,
                ))
                .unwrap(),
            "192.0.2.1".parse().unwrap(),
        );
        assert_eq!(
            app.clone().oneshot(first).await.unwrap().status(),
            StatusCode::UNAUTHORIZED
        );

        let second = with_peer(
            Request::post("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"username":"alice","password":"wrong","organization_id":"tenant-7"}"#,
                ))
                .unwrap(),
            "192.0.2.2".parse().unwrap(),
        );
        let response = app.oneshot(second).await.unwrap();
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(response.headers().get(header::RETRY_AFTER).unwrap(), "60");
    }

    #[test]
    fn limiter_keeps_source_and_account_buckets_independent_with_retry_and_expiry() {
        let mut limiter = LoginLimiter::new(LimitConfig {
            source_attempts: 2,
            account_attempts: 1,
            window_seconds: 10,
            max_entries: 10,
        });
        let source_a = [1; 32];
        let source_b = [2; 32];
        let account_a = [3; 32];
        let account_b = [4; 32];

        assert_eq!(limiter.check_and_record(source_a, account_a, 100), Ok(()));
        assert_eq!(limiter.check_and_record(source_b, account_a, 101), Err(9));
        assert_eq!(limiter.check_and_record(source_a, account_b, 101), Ok(()));
        assert_eq!(limiter.check_and_record(source_a, [5; 32], 102), Err(8));
        assert_eq!(limiter.check_and_record(source_a, account_a, 110), Ok(()));
    }

    #[test]
    fn tenant_resolution_fails_cloud_missing_through_dummy_path_and_allows_explicit_standalone_authority()
     {
        assert_eq!(
            resolve_organization(None, None, true),
            TenantResolution::DummyDeny
        );
        assert_eq!(
            resolve_organization(Some(" "), Some("tenant-default"), true),
            TenantResolution::DummyDeny
        );
        assert_eq!(
            resolve_organization(Some("tenant-7"), None, true),
            TenantResolution::Authenticate("tenant-7".into())
        );
        assert_eq!(
            resolve_organization(None, Some("tenant-default"), false),
            TenantResolution::Authenticate("tenant-default".into())
        );
        assert_eq!(
            resolve_organization(None, None, false),
            TenantResolution::Authenticate(String::new())
        );
    }

    #[test]
    fn source_limit_is_lower_and_cannot_exhaust_one_account_bucket() {
        assert!(SOURCE_ATTEMPTS < ACCOUNT_ATTEMPTS);
        let mut limiter = LoginLimiter::new(LimitConfig {
            source_attempts: SOURCE_ATTEMPTS,
            account_attempts: ACCOUNT_ATTEMPTS,
            window_seconds: 60,
            max_entries: 100,
        });
        for account in 0..SOURCE_ATTEMPTS {
            assert_eq!(
                limiter.check_and_record([1; 32], [account as u8; 32], 10),
                Ok(())
            );
        }
        assert!(limiter.check_and_record([1; 32], [99; 32], 10).is_err());
        assert_eq!(limiter.accounts.entries.get(&[0; 32]).unwrap().attempts, 1);
    }

    #[test]
    fn cloud_rate_limit_deployment_requires_an_explicit_supported_contract() {
        assert!(rate_limit_deployment(None, true).is_err());
        assert!(rate_limit_deployment(Some(""), true).is_err());
        assert!(rate_limit_deployment(Some("redis"), true).is_err());
        assert_eq!(
            rate_limit_deployment(Some("single-instance"), true).unwrap(),
            RateLimitDeployment::SingleInstance
        );
        assert_eq!(
            rate_limit_deployment(Some("upstream-bounded"), true).unwrap(),
            RateLimitDeployment::UpstreamBounded
        );
        assert_eq!(
            rate_limit_deployment(None, false).unwrap(),
            RateLimitDeployment::Standalone
        );
    }

    #[test]
    fn response_user_contract_accepts_exact_bounds_and_rejects_every_overage() {
        let exact = LoginUser {
            id: "i".repeat(MAX_USER_ID_BYTES),
            username: "u".repeat(MAX_USERNAME_BYTES),
            email: "e".repeat(MAX_EMAIL_BYTES),
            roles: vec!["r".repeat(MAX_ROLE_BYTES); MAX_ROLES],
            organization_id: "o".repeat(MAX_ORGANIZATION_BYTES),
        };
        assert!(bounded_login_user(&exact, true));
        let mut over = exact;
        over.id.push('i');
        assert!(!bounded_login_user(&over, true));
        over.id.truncate(MAX_USER_ID_BYTES);
        over.roles.push("role".into());
        assert!(!bounded_login_user(&over, true));
        over.roles.truncate(MAX_ROLES);
        over.organization_id.clear();
        assert!(bounded_login_user(&over, false));
        assert!(!bounded_login_user(&over, true));

        assert!(bounded_access_token(&"t".repeat(MAX_ACCESS_TOKEN_BYTES)));
        assert!(!bounded_access_token(""));
        assert!(!bounded_access_token(
            &"t".repeat(MAX_ACCESS_TOKEN_BYTES + 1)
        ));
    }

    #[test]
    fn rust_and_next_share_the_same_access_token_ceiling() {
        let limits: serde_json::Value =
            serde_json::from_str(include_str!("../../ui/next/src/lib/auth/authLimits.json"))
                .unwrap();
        assert_eq!(
            limits["maxAccessTokenBytes"].as_u64(),
            Some(MAX_ACCESS_TOKEN_BYTES as u64)
        );
    }

    #[tokio::test]
    async fn accepted_user_claims_always_fit_the_shared_token_ceiling() {
        let store = Arc::new(Store::new());
        let roles = vec!["r".repeat(MAX_ROLE_BYTES); 12];
        let user = store
            .create_user(
                "bounded-user".into(),
                "bounded@example.test".into(),
                "correct horse".into(),
                roles.clone(),
                "tenant-7".into(),
            )
            .await
            .unwrap();
        assert!(Store::user_access_token_fits(&user));
        let (token, _) = store.issue_token_with_expiry(&user).unwrap();
        assert!(token.len() <= MAX_ACCESS_TOKEN_BYTES);

        let oversized_roles = vec!["r".repeat(MAX_ROLE_BYTES); MAX_ROLES];
        assert!(
            store
                .update_user(
                    &user.id,
                    None,
                    Some(oversized_roles.clone()),
                    None,
                    "tenant-7",
                )
                .await
                .is_err()
        );
        assert_eq!(
            store.get_user(&user.id, "tenant-7").await.unwrap().roles,
            roles
        );
        assert!(
            store
                .create_user(
                    "oversized-user".into(),
                    "oversized@example.test".into(),
                    "correct horse".into(),
                    oversized_roles,
                    "tenant-7".into(),
                )
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn token_expiry_in_login_response_exactly_matches_signed_claim() {
        let (app, store, _) = app_with_user().await;
        let response = app
            .oneshot(json_request(
                "/api/v1/auth/login",
                r#"{"username":"Alice","password":"correct horse","organization_id":"tenant-7"}"#,
            ))
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(
            &to_bytes(response.into_body(), MAX_RESPONSE_BYTES)
                .await
                .unwrap(),
        )
        .unwrap();
        let claims = store
            .validate_token(body["token"].as_str().unwrap())
            .await
            .unwrap();
        assert_eq!(body["expires_at"].as_i64().unwrap(), claims.exp);
    }

    #[tokio::test]
    async fn logout_returns_typed_invalid_token() {
        let store = Store::new();
        assert_eq!(
            store.logout_token("not-a-token").await,
            Err(crate::LogoutError::InvalidToken)
        );
    }

    #[test]
    fn limiter_uses_indexed_expiry_and_fails_closed_at_capacity() {
        let mut limiter = LoginLimiter::new(LimitConfig {
            source_attempts: 10,
            account_attempts: 10,
            window_seconds: 100,
            max_entries: 2,
        });
        limiter.check_and_record([1; 32], [11; 32], 1).unwrap();
        limiter.check_and_record([2; 32], [12; 32], 2).unwrap();
        assert_eq!(limiter.check_and_record([3; 32], [13; 32], 3), Err(98));
        assert_eq!(limiter.sources.entries.len(), 2);
        assert!(limiter.sources.entries.contains_key(&[1; 32]));
        assert_eq!(limiter.sources.expirations.len(), 2);
        assert_eq!(limiter.sources.least_recent.len(), 2);
        limiter.check_and_record([3; 32], [13; 32], 101).unwrap();
        assert!(!limiter.sources.entries.contains_key(&[1; 32]));
        assert!(limiter.sources.entries.contains_key(&[3; 32]));

        let key = [9; 32];
        assert_eq!(
            keyed_hash(&key, b" tenant-7\0Alice "),
            keyed_hash(&key, b" tenant-7\0Alice ")
        );
        assert_ne!(
            keyed_hash(&key, b" tenant-7\0Alice "),
            keyed_hash(&key, b"tenant-7\0alice")
        );
    }

    #[tokio::test]
    async fn login_fails_fast_when_password_hash_capacity_is_exhausted() {
        let (base_app, store, _) = app_with_user().await;
        drop(base_app);
        let permits = store
            .password_slots
            .clone()
            .acquire_many_owned(crate::MAX_PASSWORD_HASH_CONCURRENCY as u32)
            .await
            .unwrap();
        let response = router_with_state(HttpAuthState::new(store, HashSet::new()))
            .oneshot(json_request(
                "/api/v1/auth/login",
                r#"{"username":"Alice","password":"correct horse","organization_id":"tenant-7"}"#,
            ))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(
            response.headers().get(header::CACHE_CONTROL).unwrap(),
            "no-store"
        );
        drop(permits);
    }

    #[test]
    fn forwarding_headers_are_ignored_by_default_and_bounded_for_exact_trusted_peers() {
        let peer: IpAddr = "10.0.0.2".parse().unwrap();
        let client: IpAddr = "203.0.113.9".parse().unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "203.0.113.9".parse().unwrap());
        assert_eq!(
            request_source(&headers, peer, &HashSet::new()),
            Ok(RequestSource {
                ip: peer,
                class: AuditSource::DirectPeer,
            })
        );

        let trusted = HashSet::from([peer]);
        assert_eq!(
            request_source(&headers, peer, &trusted),
            Ok(RequestSource {
                ip: client,
                class: AuditSource::TrustedProxy,
            })
        );
        headers.insert("x-forwarded-for", "203.0.113.9, 10.0.0.1".parse().unwrap());
        assert_eq!(request_source(&headers, peer, &trusted), Err(()));
        headers.insert("forwarded", "for=203.0.113.9".parse().unwrap());
        assert_eq!(request_source(&headers, peer, &trusted), Err(()));
    }

    #[test]
    fn duplicate_content_type_and_authorization_headers_are_rejected() {
        let mut headers = HeaderMap::new();
        headers.append(header::CONTENT_TYPE, "application/json".parse().unwrap());
        headers.append(header::CONTENT_TYPE, "application/json".parse().unwrap());
        assert!(!has_exact_json_content_type(&headers));

        headers.append(header::AUTHORIZATION, "Bearer first".parse().unwrap());
        headers.append(header::AUTHORIZATION, "Bearer second".parse().unwrap());
        assert_eq!(bearer_token(&headers), None);
    }

    #[test]
    fn audit_context_is_random_coarse_and_cannot_carry_request_identity() {
        let first = new_audit_id();
        let second = new_audit_id();
        assert_eq!(first.len(), 32);
        assert!(first.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert_ne!(first, second);

        let context = format!(
            "{:?}:{:?}",
            AuditEvent::LoginDenied,
            AuditSource::TrustedProxy
        );
        for raw in ["Alice", "tenant-7", "correct horse", "Bearer"] {
            assert!(!context.contains(raw));
        }
    }

    #[tokio::test]
    async fn logout_requires_strict_bearer_and_is_idempotent() {
        let (app, store, user) = app_with_user().await;
        let token = store.issue_token(&user).unwrap();

        let missing = with_peer(
            Request::post("/api/v1/auth/logout")
                .body(Body::empty())
                .unwrap(),
            IpAddr::V4(Ipv4Addr::LOCALHOST),
        );
        assert_eq!(
            app.clone().oneshot(missing).await.unwrap().status(),
            StatusCode::UNAUTHORIZED
        );

        for authorization in ["bearer token", "Bearer has space", "Basic token"] {
            let malformed = with_peer(
                Request::post("/api/v1/auth/logout")
                    .header("authorization", authorization)
                    .body(Body::empty())
                    .unwrap(),
                IpAddr::V4(Ipv4Addr::LOCALHOST),
            );
            assert_eq!(
                app.clone().oneshot(malformed).await.unwrap().status(),
                StatusCode::UNAUTHORIZED
            );
        }

        let (prefix, signature) = token.rsplit_once('.').unwrap();
        let replacement = if signature.starts_with('A') { "B" } else { "A" };
        let forged = format!("{prefix}.{replacement}{}", &signature[1..]);
        let forged_request = with_peer(
            Request::post("/api/v1/auth/logout")
                .header("authorization", format!("Bearer {forged}"))
                .body(Body::empty())
                .unwrap(),
            IpAddr::V4(Ipv4Addr::LOCALHOST),
        );
        let forged_response = app.clone().oneshot(forged_request).await.unwrap();
        assert_eq!(forged_response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(
            forged_response
                .headers()
                .get(header::CACHE_CONTROL)
                .unwrap(),
            "no-store"
        );
        let forged_body = to_bytes(forged_response.into_body(), 1024).await.unwrap();
        assert_eq!(forged_body.as_ref(), br#"{"error":"invalid token"}"#);

        for _ in 0..2 {
            let request = with_peer(
                Request::post("/api/v1/auth/logout")
                    .header("authorization", format!("Bearer {token}"))
                    .body(Body::empty())
                    .unwrap(),
                IpAddr::V4(Ipv4Addr::LOCALHOST),
            );
            let response = app.clone().oneshot(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(response.headers().get("cache-control").unwrap(), "no-store");
        }
        assert!(store.validate_token(&token).await.is_err());
    }

    static REG_ENV_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[tokio::test]
    async fn test_register_rejected_when_disabled() {
        let _lock = REG_ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        
        temp_env::async_with_vars([("OHC_REGISTRATION_ENABLED", Some("false"))], async {
            let app = router_for_test();
            let request = json_request(
                "/api/v1/auth/register",
                r#"{"username":"testuser","email":"test@example.com","password":"mypassword"}"#,
            );
            let response = app.oneshot(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::FORBIDDEN);
            let body = to_bytes(response.into_body(), 1024).await.unwrap();
            assert_eq!(body.as_ref(), br#"{"error":"registration closed"}"#);
        })
        .await;
    }

    #[tokio::test]
    async fn test_register_succeeds_when_enabled() {
        let _lock = REG_ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

        temp_env::async_with_vars([("OHC_REGISTRATION_ENABLED", Some("true"))], async {
            let store = Arc::new(Store::new());
            let app = router_with_state(HttpAuthState::new(store.clone(), HashSet::new()));
            
            let request = json_request(
                "/api/v1/auth/register",
                r#"{"username":"newuser","email":"newuser@example.com","password":"newpassword"}"#,
            );
            
            let response = app.oneshot(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::CREATED);
            
            let body: serde_json::Value = serde_json::from_slice(
                &to_bytes(response.into_body(), MAX_BODY_BYTES)
                    .await
                    .unwrap(),
            )
            .unwrap();
            
            assert!(
                body["token"]
                    .as_str()
                    .is_some_and(|token| !token.is_empty())
            );
            assert_eq!(body["user"]["username"], "newuser");
            assert_eq!(body["user"]["email"], "newuser@example.com");
            assert_eq!(body["user"]["roles"], serde_json::json!(["ADMIN"]));
            assert!(!body["user"]["organization_id"].as_str().unwrap().is_empty());
        })
        .await;
    }

    #[tokio::test]
    async fn test_register_validation_and_duplicate() {
        let _lock = REG_ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

        temp_env::async_with_vars([("OHC_REGISTRATION_ENABLED", Some("true"))], async {
            let store = Arc::new(Store::new());
            let app = router_with_state(HttpAuthState::new(store.clone(), HashSet::new()));

            // 1. Validation error: empty fields
            let bad_request = json_request(
                "/api/v1/auth/register",
                r#"{"username":"","email":"test@example.com","password":"password"}"#,
            );
            let response = app.clone().oneshot(bad_request).await.unwrap();
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);

            // 2. Successful first registration
            let request = json_request(
                "/api/v1/auth/register",
                r#"{"username":"user1","email":"user1@example.com","password":"password"}"#,
            );
            let response = app.clone().oneshot(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::CREATED);

            // 3. Since we generated a new UUID for organization_id, we can register the same username again.
            // But let's check password length requirement!
            let short_pw = json_request(
                "/api/v1/auth/register",
                r#"{"username":"user2","email":"user2@example.com","password":"123"}"#,
            );
            let response = app.clone().oneshot(short_pw).await.unwrap();
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
            let body: serde_json::Value = serde_json::from_slice(
                &to_bytes(response.into_body(), MAX_BODY_BYTES)
                    .await
                    .unwrap(),
            )
            .unwrap();
            assert_eq!(body["error"], "password must be at least 6 characters");
        })
        .await;
    }

    struct FailingRepo {
        lookup_fails: bool,
        revocation_fails: bool,
    }

    #[async_trait]
    impl UserRepository for FailingRepo {
        async fn create_user(&self, _: User, _: &str) -> Result<(), String> {
            Err("unavailable".into())
        }
        async fn get_by_id(&self, _: &str, _: &str) -> Result<User, String> {
            Err("unavailable".into())
        }
        async fn get_by_username(&self, _: &str, _: &str) -> Result<User, String> {
            Err("unavailable".into())
        }
        async fn get_by_email(&self, _: &str, _: &str) -> Result<User, String> {
            Err("unavailable".into())
        }
        async fn get_by_login_identifier(&self, _: &str, _: &str) -> Result<Option<User>, String> {
            if self.lookup_fails {
                Err("database unavailable".into())
            } else {
                Ok(None)
            }
        }
        async fn get_by_oidc_subject(&self, _: &str, _: &str) -> Result<User, String> {
            Err("unavailable".into())
        }
        async fn list_users(&self, _: &str) -> Result<Vec<User>, String> {
            Err("unavailable".into())
        }
        async fn update_user(&self, _: User, _: &str) -> Result<(), String> {
            Err("unavailable".into())
        }
        async fn delete_user(&self, _: &str, _: &str) -> Result<(), String> {
            Err("unavailable".into())
        }
        async fn revoke_token(&self, _: String, _: DateTime<Utc>, _: &str) -> Result<(), String> {
            if self.revocation_fails {
                Err("database unavailable".into())
            } else {
                Ok(())
            }
        }
        async fn is_revoked(&self, _: &str, _: &str) -> Result<bool, String> {
            Ok(false)
        }
    }

    #[tokio::test]
    async fn backend_failures_are_generic_and_fail_closed() {
        let store = Arc::new(Store::with_repo(Arc::new(FailingRepo {
            lookup_fails: true,
            revocation_fails: true,
        })));
        let app = router(store.clone()).unwrap();
        let response = app
            .clone()
            .oneshot(json_request(
                "/api/v1/auth/login",
                r#"{"username":"Alice","password":"wrong","organization_id":"tenant-7"}"#,
            ))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(body.as_ref(), br#"{"error":"authentication unavailable"}"#);

        let user = User {
            id: "user-7".into(),
            username: "Alice".into(),
            email: "alice@example.test".into(),
            password_hash: String::new(),
            roles: vec![],
            active: true,
            organization_id: Some("tenant-7".into()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            oidc_subject: None,
        };
        let token = store.issue_token(&user).unwrap();
        let request = with_peer(
            Request::post("/api/v1/auth/logout")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
            IpAddr::V4(Ipv4Addr::LOCALHOST),
        );
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(body.as_ref(), br#"{"error":"logout unavailable"}"#);
    }

    #[tokio::test]
    async fn test_api_keys_workflow() {
        let (app, store, user) = app_with_user().await;
        let token = store.issue_token(&user).unwrap();

        // 1. Creating a key requires authentication (unauthorized returns 401)
        let unauthorized_request = with_peer(
            Request::post("/api/v1/settings/keys")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"Test Key"}"#))
                .unwrap(),
            IpAddr::V4(Ipv4Addr::LOCALHOST),
        );
        let response = app.clone().oneshot(unauthorized_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        // 2. Successfully creates a key for authenticated users
        let authorized_request = with_peer(
            Request::post("/api/v1/settings/keys")
                .header("authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"My Test Key"}"#))
                .unwrap(),
            IpAddr::V4(Ipv4Addr::LOCALHOST),
        );
        let response = app.clone().oneshot(authorized_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body: serde_json::Value = serde_json::from_slice(
            &to_bytes(response.into_body(), MAX_BODY_BYTES)
                .await
                .unwrap(),
        )
        .unwrap();

        let raw_key = body["raw_key"].as_str().unwrap();
        assert!(raw_key.starts_with("ohc_gwy_"));
        assert_eq!(body["name"], "My Test Key");
        assert!(body["created_at"].as_str().is_some());

        // Verify correctly hashing it in the database
        // SHA256 of raw_key
        let expected_hash = format!("{:x}", Sha256::digest(raw_key.as_bytes()));

        // Check our in-memory fallback list to verify the hash is correct
        {
            let keys = get_in_memory_keys().lock().unwrap();
            let key = keys.iter().find(|k| k.name == "My Test Key").unwrap();
            assert_eq!(key.key_hash, expected_hash);
            assert_eq!(key.member_id, user.id);
        }

        // 3. Listing keys returns all created keys
        let list_request = with_peer(
            Request::get("/api/v1/settings/keys")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
            IpAddr::V4(Ipv4Addr::LOCALHOST),
        );
        let response = app.clone().oneshot(list_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let list_body: serde_json::Value = serde_json::from_slice(
            &to_bytes(response.into_body(), MAX_BODY_BYTES)
                .await
                .unwrap(),
        )
        .unwrap();

        let list_array = list_body.as_array().unwrap();
        assert!(!list_array.is_empty());
        let created_key_meta = list_array.iter().find(|k| k["name"] == "My Test Key").unwrap();
        let key_id = created_key_meta["id"].as_str().unwrap().to_string();

        // 4. Revoking a key deletes it
        let delete_request = with_peer(
            Request::delete(format!("/api/v1/settings/keys/{}", key_id))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
            IpAddr::V4(Ipv4Addr::LOCALHOST),
        );
        let response = app.clone().oneshot(delete_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let delete_body: serde_json::Value = serde_json::from_slice(
            &to_bytes(response.into_body(), MAX_BODY_BYTES)
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(delete_body["ok"], true);

        // Verify it is no longer listed
        let list_request2 = with_peer(
            Request::get("/api/v1/settings/keys")
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
            IpAddr::V4(Ipv4Addr::LOCALHOST),
        );
        let response2 = app.clone().oneshot(list_request2).await.unwrap();
        assert_eq!(response2.status(), StatusCode::OK);

        let list_body2: serde_json::Value = serde_json::from_slice(
            &to_bytes(response2.into_body(), MAX_BODY_BYTES)
                .await
                .unwrap(),
        )
        .unwrap();
        let list_array2 = list_body2.as_array().unwrap();
        assert!(list_array2.iter().all(|k| k["id"] != key_id));
    }

    #[tokio::test]
    async fn test_member_usage_analytics_auth_and_restriction() {
        let _lock = REG_ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let (app, store, user) = app_with_user().await;
        
        // 1. Unauthenticated request returns 401
        let unauthorized_request = with_peer(
            Request::get("/api/v1/ui/admin/usage")
                .body(Body::empty())
                .unwrap(),
            IpAddr::V4(Ipv4Addr::LOCALHOST),
        );
        let response = app.clone().oneshot(unauthorized_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        // 2. Authenticated non-admin user returns 403
        let viewer_user = store
            .create_user(
                "ViewerBob".into(),
                "viewer@example.test".into(),
                "password".into(),
                vec!["VIEWER".into()],
                "tenant-7".into(),
            )
            .await
            .unwrap();
        let viewer_token = store.issue_token(&viewer_user).unwrap();

        let forbidden_request = with_peer(
            Request::get("/api/v1/ui/admin/usage")
                .header("authorization", format!("Bearer {viewer_token}"))
                .body(Body::empty())
                .unwrap(),
            IpAddr::V4(Ipv4Addr::LOCALHOST),
        );
        let response = app.clone().oneshot(forbidden_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(body.as_ref(), br#"{"error":"admin access required"}"#);

        // 3. Authenticated admin user returns 200 and listed analytics
        let admin_token = store.issue_token(&user).unwrap();
        
        // Populate a mock log
        {
            let mut logs = get_in_memory_usage_logs().lock().unwrap();
            logs.push(InMemoryUsageLog {
                username: "Alice".to_string(),
                feature: "gateway_run".to_string(),
                tokens_used: 1250,
                computed_cost: 0.0025,
                organization_id: "tenant-7".to_string(),
            });
        }

        let success_request = with_peer(
            Request::get("/api/v1/ui/admin/usage")
                .header("authorization", format!("Bearer {admin_token}"))
                .body(Body::empty())
                .unwrap(),
            IpAddr::V4(Ipv4Addr::LOCALHOST),
        );
        let response = app.clone().oneshot(success_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        
        let body: serde_json::Value = serde_json::from_slice(
            &to_bytes(response.into_body(), MAX_BODY_BYTES)
                .await
                .unwrap(),
        )
        .unwrap();
        
        let array = body.as_array().unwrap();
        assert!(!array.is_empty());
        assert_eq!(array[0]["username"], "Alice");
        assert_eq!(array[0]["feature"], "gateway_run");
        assert_eq!(array[0]["tokens_used"], 1250);
        assert_eq!(array[0]["computed_cost"], 0.0025);
    }
}
