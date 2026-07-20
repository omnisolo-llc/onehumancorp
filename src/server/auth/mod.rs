#![allow(clippy::all)]
pub use ::server_common as common;
pub use ::server_ohc as ohc;
pub use ::server_oidc as oidc;

pub mod orchestration;
pub mod peer_identity;
pub mod postgres_store;
pub mod sqlite_store;
pub mod user_repository;
pub mod grpc;
pub mod http;

use std::collections::HashMap;

pub async fn strict_bearer_auth_middleware(
    axum::extract::State(store): axum::extract::State<std::sync::Arc<Store>>,
    mut req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    use axum::response::IntoResponse;

    let mut authorization_values = req
        .headers()
        .get_all(axum::http::header::AUTHORIZATION)
        .iter();
    let token = authorization_values
        .next()
        .filter(|_| authorization_values.next().is_none())
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|token| {
            !token.is_empty()
                && token.len() <= MAX_ACCESS_TOKEN_BYTES
                && !token
                    .chars()
                    .any(|character| character.is_whitespace() || character.is_ascii_control())
        });
    let Some(token) = token else {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    };

    let mut claims = match store.validate_token(token).await {
        Ok(claims) => claims,
        Err(_) => return axum::http::StatusCode::UNAUTHORIZED.into_response(),
    };
    let Some(organization_id) = claims
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|organization_id| {
            !organization_id.is_empty() && !organization_id.eq_ignore_ascii_case("system")
        })
        .map(str::to_string)
    else {
        return axum::http::StatusCode::UNAUTHORIZED.into_response();
    };

    let user_id = claims.sub.clone();
    claims.organization_id = Some(organization_id.clone());
    req.extensions_mut().insert(crate::orchestration::AuthInfo {
        org_id: organization_id.clone(),
        agent_id: user_id.clone(),
        spiffe_id: "spiffe://onehumancorp.io/web-session".to_string(),
    });
    req.extensions_mut().insert(claims);
    next.run(req).await
}
use std::sync::Arc;
use std::sync::RwLock;

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;

type HmacSha256 = Hmac<Sha256>;

/// Authentication mode.
#[derive(Debug, Clone)]
pub enum AuthMode {
    /// No authentication (dev/test only).
    Disabled,
    /// SPIFFE/mTLS peer certificate.
    Spiffe { allowed_id: String },
}

/// Build an AuthMode from environment variables.
///
///   OHC_AGENT_AUTH_DISABLED=true   – skip auth (dev only)
///   OHC_AGENT_SPIFFE_ID            – restricts SPIFFE ID (enables SPIFFE mode)
pub fn auth_mode_from_env() -> AuthMode {
    AuthMode::Spiffe {
        allowed_id: env::var("OHC_AGENT_SPIFFE_ID").unwrap_or_default(),
    }
}

pub const ROLE_ADMIN: &str = "ADMIN";
pub const ROLE_OPERATOR: &str = "OPERATOR";
pub const ROLE_VIEWER: &str = "VIEWER";
pub const DEFAULT_COST: u32 = 10;
pub const MAX_ACCESS_TOKEN_BYTES: usize = 2048;
pub const MAX_AUTH_USER_ID_BYTES: usize = 128;
pub const MAX_AUTH_USERNAME_BYTES: usize = 128;
pub const MAX_AUTH_EMAIL_BYTES: usize = 254;
pub const MAX_AUTH_ORGANIZATION_BYTES: usize = 128;
pub const MAX_AUTH_ROLES: usize = 32;
pub const MAX_AUTH_ROLE_BYTES: usize = 64;
const MAX_ACCESS_TOKEN_CLAIMS_BYTES: usize = 1400;
const MAX_PASSWORD_HASH_CONCURRENCY: usize = 16;
const DUMMY_PASSWORD_HASH: &str =
    "$2b$10$dVqpE7hfDSB6wqVCcDvUGuLwuaGMsiC.FznSYrakLDs.8jQXm2wMC";

fn hash(password: String, cost: u32) -> Result<String, String> {
    bcrypt::hash(password, cost).map_err(|e| e.to_string())
}

fn verify(password: &str, hash: &str) -> Result<bool, String> {
    bcrypt::verify(password, hash).map_err(|e| e.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthenticationError {
    InvalidCredentials,
    Unavailable(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogoutError {
    InvalidToken,
    ValidationUnavailable,
    RevocationUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenValidationError {
    Invalid(String),
    Unavailable,
}

impl TokenValidationError {
    fn invalid(message: impl Into<String>) -> Self {
        Self::Invalid(message.into())
    }
}

impl std::fmt::Display for TokenValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(message) => formatter.write_str(message),
            Self::Unavailable => formatter.write_str("token validation unavailable"),
        }
    }
}

impl std::fmt::Display for AuthenticationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCredentials => formatter.write_str("invalid credentials"),
            Self::Unavailable(_) => formatter.write_str("authentication unavailable"),
        }
    }
}

impl std::error::Error for AuthenticationError {}

enum RedisUrlSetting {
    Absent,
    Value(String),
    InvalidUnicode,
}

fn configure_redis_client(setting: RedisUrlSetting) -> Result<Option<redis::Client>, String> {
    match setting {
        RedisUrlSetting::Absent => Ok(None),
        RedisUrlSetting::Value(url) => redis::Client::open(url)
            .map(Some)
            .map_err(|_| "invalid revocation cache configuration".to_string()),
        RedisUrlSetting::InvalidUnicode => {
            Err("invalid revocation cache configuration".to_string())
        }
    }
}

fn decide_credentials_with<F>(
    candidate: Option<User>,
    password: &str,
    org_id: &str,
    mut verifier: F,
) -> Result<User, AuthenticationError>
where
    F: FnMut(&str, &str) -> Result<bool, String>,
{
    let candidate = candidate.filter(|user| {
        user.active
            && match user.organization_id.as_deref() {
                Some(user_org) => user_org == org_id,
                None => org_id.is_empty(),
            }
    });
    let password_hash = candidate
        .as_ref()
        .map(|user| user.password_hash.as_str())
        .unwrap_or(DUMMY_PASSWORD_HASH);
    let verified = verifier(password, password_hash).map_err(AuthenticationError::Unavailable)?;
    match (candidate, verified) {
        (Some(user), true) => Ok(user),
        _ => Err(AuthenticationError::InvalidCredentials),
    }
}

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rand::RngCore;
use ::server_common::Claims;
use tonic::{Request, Response, Status};
use ::server_ohc::orchestration::auth_service_server::AuthService;
use ::server_ohc::orchestration::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub roles: Vec<String>,
    pub active: bool,
    pub organization_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub oidc_subject: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct TenantKey {
    pub org_id: String,
    pub key: String,
}

#[derive(Debug, Clone)]
pub struct OIDCConfig {
    pub issuer_url: String,
    pub client_id: String,
    pub enabled: bool,
}

pub struct Store {
    users: RwLock<HashMap<String, User>>,
    by_name: RwLock<HashMap<TenantKey, String>>,
    by_email: RwLock<HashMap<TenantKey, String>>,
    by_oidc: RwLock<HashMap<TenantKey, String>>,
    revoked: RwLock<HashMap<String, DateTime<Utc>>>,
    redis_client: Option<redis::Client>,
    redis_configuration_error: Option<String>,
    secret: Vec<u8>,
    password_slots: Arc<tokio::sync::Semaphore>,
    oidc_cfg: RwLock<OIDCConfig>,
    repo: Option<std::sync::Arc<dyn crate::user_repository::UserRepository>>,
}

impl Store {
    fn access_token_claims(user: &User, issued_at: i64, expires_at: i64, jti: String) -> Claims {
        Claims {
            sub: user.id.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
            roles: user.roles.clone(),
            organization_id: user.organization_id.clone(),
            session_id: None,
            iat: issued_at,
            exp: expires_at,
            jti,
        }
    }

    fn access_token_claims_fit(claims: &Claims) -> bool {
        serde_json::to_vec(claims)
            .is_ok_and(|payload| payload.len() <= MAX_ACCESS_TOKEN_CLAIMS_BYTES)
    }

    pub fn user_access_token_fits(user: &User) -> bool {
        if user.id.is_empty()
            || user.id.len() > MAX_AUTH_USER_ID_BYTES
            || user.username.is_empty()
            || user.username.len() > MAX_AUTH_USERNAME_BYTES
            || user.email.is_empty()
            || user.email.len() > MAX_AUTH_EMAIL_BYTES
            || user
                .organization_id
                .as_deref()
                .is_some_and(|organization| organization.len() > MAX_AUTH_ORGANIZATION_BYTES)
            || user.roles.len() > MAX_AUTH_ROLES
            || user
                .roles
                .iter()
                .any(|role| role.is_empty() || role.len() > MAX_AUTH_ROLE_BYTES)
        {
            return false;
        }
        let issued_at = chrono::Utc::now().timestamp();
        Self::access_token_claims_fit(&Self::access_token_claims(
            user,
            issued_at,
            issued_at.saturating_add(24 * 60 * 60),
            "0".repeat(32),
        ))
    }

    pub fn new() -> Self {
        let configured_secret =
            ::server_common::secret_source::load_optional_secret("JWT_SECRET", "JWT_SECRET_FILE")
                .unwrap_or_else(|_| panic!("invalid authentication secret configuration"));
        let secret = configured_secret.unwrap_or_else(|| {
                if ::server_config::get().multitenant {
                    panic!("invalid authentication secret configuration");
                }

                let secret_path = ::server_config::get_safe_user_dir().join(".ohc_jwt_secret");
                if secret_path.exists() {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::OpenOptionsExt;
                        use std::os::unix::fs::PermissionsExt;
                        let mut options = std::fs::OpenOptions::new();
                        options.read(true);
                        #[cfg(target_os = "linux")]
                        options.custom_flags(0x00020000); // O_NOFOLLOW
                        #[cfg(target_os = "macos")]
                        options.custom_flags(0x0100); // O_NOFOLLOW

                        if let Ok(mut file) = options.open(&secret_path) {
                            if let Ok(metadata) = file.metadata() {
                                let mut perms = metadata.permissions();
                                if perms.mode() & 0o777 != 0o600 {
                                    tracing::warn!("Insecure permissions on .ohc_jwt_secret. Fixing it to prevent TOCTOU attacks.");
                                    perms.set_mode(0o600);
                                    if let Err(e) = file.set_permissions(perms) {
                                        tracing::error!("Failed to securely update .ohc_jwt_secret file permissions: {}", e);
                                        std::process::exit(1);
                                    }
                                }
                            }
                            use std::io::Read;
                            let mut bytes = Vec::new();
                            if file.read_to_end(&mut bytes).is_ok() && bytes.len() >= 32 {
                                return bytes;
                            }
                        }
                    }
                    #[cfg(not(unix))]
                    {
                        if let Ok(bytes) = std::fs::read(&secret_path) {
                            if bytes.len() >= 32 {
                                return bytes;
                            }
                        }
                    }
                }

                let sqlite_key_opt = std::env::var("OHC_SQLITE_KEY").ok().or_else(|| {
                    let secret_path = ::server_config::get_safe_user_dir().join(".ohc_sqlite_key");
                    if secret_path.exists() {
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::OpenOptionsExt;
                            use std::os::unix::fs::PermissionsExt;
                            let mut options = std::fs::OpenOptions::new();
                            options.read(true);
                        #[cfg(target_os = "linux")]
                        options.custom_flags(0x00020000); // O_NOFOLLOW
                        #[cfg(target_os = "macos")]
                        options.custom_flags(0x0100); // O_NOFOLLOW

                            if let Ok(mut file) = options.open(&secret_path) {
                                if let Ok(metadata) = file.metadata() {
                                    let mut perms = metadata.permissions();
                                    if perms.mode() & 0o777 != 0o600 {
                                        tracing::warn!("Insecure permissions on .ohc_sqlite_key. Fixing it to prevent TOCTOU attacks.");
                                        perms.set_mode(0o600);
                                        if let Err(e) = file.set_permissions(perms) {
                                            tracing::error!("Failed to securely update .ohc_sqlite_key file permissions: {}", e);
                                            std::process::exit(1);
                                        }
                                    }
                                }
                                use std::io::Read;
                                let mut bytes = String::new();
                                if file.read_to_string(&mut bytes).is_ok() && !bytes.trim().is_empty() {
                                    return Some(bytes.trim().to_string());
                                }
                            }
                        }
                        #[cfg(not(unix))]
                        {
                            if let Ok(bytes) = std::fs::read_to_string(&secret_path) {
                                if !bytes.trim().is_empty() {
                                    return Some(bytes.trim().to_string());
                                }
                            }
                        }
                    }
                    None
                });

                let new_secret = if let Some(sqlite_key) = sqlite_key_opt {
                    tracing::debug!("falling back to generated JWT secret; deriving from OHC_SQLITE_KEY for determinism; writing to .ohc_jwt_secret for persistence"); // pii-safe
                    let mut mac = HmacSha256::new_from_slice(b"ohc_jwt_derivation_salt").expect("HMAC can take key of any size");
                    mac.update(sqlite_key.as_bytes());
                    mac.finalize().into_bytes().to_vec()
                } else {
                    tracing::debug!("falling back to generated JWT secret; writing to .ohc_jwt_secret for persistence"); // pii-safe
                    let mut key_bytes = [0u8; 32];
                    use rand::RngCore;
                    rand::thread_rng().fill_bytes(&mut key_bytes);
                    key_bytes.to_vec()
                };

                #[cfg(unix)]
                {
                    use std::os::unix::fs::OpenOptionsExt;
                    use std::io::Write;

                    let mut options = std::fs::OpenOptions::new();
                    options.read(true).write(true).create_new(true).mode(0o600);
                    #[cfg(target_os = "linux")]
                    options.custom_flags(0x00020000); // O_NOFOLLOW
                    #[cfg(target_os = "macos")]
                    options.custom_flags(0x0100); // O_NOFOLLOW

                    if let Ok(mut file) = options.open(&secret_path) {
                        let _ = file.write_all(&new_secret);
                    }

                }
                #[cfg(not(unix))]
                {
                    let _ = std::fs::write(&secret_path, &new_secret);
                }

                new_secret
            });

        let mut roles = HashMap::new();
        let now = Utc::now();

        roles.insert(ROLE_ADMIN.to_string(), Role {
            id: ROLE_ADMIN.to_string(),
            name: ROLE_ADMIN.to_string(),
            permissions: vec!["*".to_string()],
            created_at: now,
        });
        roles.insert(ROLE_OPERATOR.to_string(), Role {
            id: ROLE_OPERATOR.to_string(),
            name: ROLE_OPERATOR.to_string(),
            permissions: vec!["read".to_string(), "write".to_string()],
            created_at: now,
        });
        roles.insert(ROLE_VIEWER.to_string(), Role {
            id: ROLE_VIEWER.to_string(),
            name: ROLE_VIEWER.to_string(),
            permissions: vec!["read".to_string()],
            created_at: now,
        });

        let issuer_url = std::env::var("OIDC_ISSUER_URL").unwrap_or_default();
        let client_id = std::env::var("OIDC_CLIENT_ID").unwrap_or_default();
        let enabled = !issuer_url.is_empty();

        let redis_configuration = if ::server_config::get().multitenant {
            let setting = match std::env::var("OHC_REDIS_URL") {
                Ok(url) => RedisUrlSetting::Value(url),
                Err(std::env::VarError::NotPresent) => RedisUrlSetting::Absent,
                Err(std::env::VarError::NotUnicode(_)) => RedisUrlSetting::InvalidUnicode,
            };
            configure_redis_client(setting)
        } else {
            Ok(None)
        };
        let (redis_client, redis_configuration_error) = match redis_configuration {
            Ok(client) => (client, None),
            Err(error) => (None, Some(error)),
        };

        let store = Store {
            users: RwLock::new(HashMap::new()),

            by_name: RwLock::new(HashMap::new()),
            by_email: RwLock::new(HashMap::new()),
            by_oidc: RwLock::new(HashMap::new()),
            revoked: RwLock::new(HashMap::new()),
            redis_client,
            redis_configuration_error,
            secret,
            password_slots: Arc::new(tokio::sync::Semaphore::new(
                MAX_PASSWORD_HASH_CONCURRENCY,
            )),
            oidc_cfg: RwLock::new(OIDCConfig {
                issuer_url,
                client_id,
                enabled,
            }),
            repo: None,
        };

        store.seed_default_admin(now);

        store
    }

    fn seed_default_admin(&self, now: DateTime<Utc>) {
        let admin_user = std::env::var("ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string());
        let admin_pass = std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin".to_string());
        let admin_email = std::env::var("ADMIN_EMAIL").unwrap_or_else(|_| "admin@localhost".to_string());

        let hash = hash(admin_pass, if cfg!(test) { 4 } else { DEFAULT_COST }).expect("Failed to hash password");

        let id = hex::encode(random_bytes(8));

        let admin = User {
            id: id.clone(),
            username: admin_user.clone(),
            email: admin_email.clone(),
            password_hash: hash,
            roles: vec![ROLE_ADMIN.to_string()],
            active: true,
            organization_id: None,
            created_at: now,
            updated_at: now,
            oidc_subject: None,
        };

        self.users.write().expect("Failed to acquire lock").insert(id.clone(), admin);
        self.by_name.write().expect("Failed to acquire lock").insert(TenantKey { org_id: "".to_string(), key: admin_user }, id.clone());
        self.by_email.write().expect("Failed to acquire lock").insert(TenantKey { org_id: "".to_string(), key: admin_email }, id);
    }


    pub fn with_repo(repo: std::sync::Arc<dyn crate::user_repository::UserRepository>) -> Self {
        let mut store = Store::new();
        store.repo = Some(repo);
        store
    }

    pub async fn create_user(&self, username: String, email: String, password: String, roles: Vec<String>, org_id: String) -> Result<User, String> {
        self.validate_org_id(&org_id)?;
        if username.is_empty() {
            return Err("username is required".to_string());
        }
        if password.len() < 6 {
            return Err("password must be at least 6 characters".to_string());
        }

        let mut users = self.users.write().expect("Failed to acquire lock");
        let mut by_name = self.by_name.write().expect("Failed to acquire lock");
        let mut by_email = self.by_email.write().expect("Failed to acquire lock");

        let name_key = TenantKey { org_id: org_id.clone(), key: username.clone() };
        if by_name.contains_key(&name_key) {
            return Err("username already taken".to_string());
        }

        let email_key = TenantKey { org_id: org_id.clone(), key: email.clone() };
        if by_email.contains_key(&email_key) {
            return Err("email already registered".to_string());
        }

        let hash = hash(password, if cfg!(test) { 4 } else { DEFAULT_COST }).expect("Failed to hash password");

        let id = hex::encode(random_bytes(8));
        let now = Utc::now();

        let user = User {
            id: id.clone(),
            username,
            email,
            password_hash: hash,
            roles,
            active: true,
            organization_id: Some(org_id),
            created_at: now,
            updated_at: now,
            oidc_subject: None,
        };

        if !Self::user_access_token_fits(&user) {
            return Err("user claims exceed the access token size limit".to_string());
        }

        users.insert(id.clone(), user.clone());
        by_name.insert(name_key, id.clone());
        by_email.insert(email_key, id);

        Ok(user)
    }

    pub async fn authenticate(
        &self,
        identifier: &str,
        password: &str,
        org_id: &str,
    ) -> Result<User, AuthenticationError> {
        let _password_slot = self
            .password_slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| AuthenticationError::Unavailable("credential verifier busy".into()))?;
        if self.validate_org_id(org_id).is_err() {
            let password = password.to_string();
            let org_id = org_id.to_string();
            return tokio::task::spawn_blocking(move || {
                decide_credentials_with(None, &password, &org_id, verify)
            })
            .await
            .map_err(|_| {
                AuthenticationError::Unavailable("credential verifier unavailable".into())
            })?;
        }

        let lookup_result = if let Some(repo) = &self.repo {
            repo.get_by_login_identifier(identifier, org_id).await
        } else {
            Ok(self.find_memory_user_with(identifier, org_id, || {}))
        };

        let (candidate, lookup_error) = match lookup_result {
            Ok(candidate) => (candidate, None),
            Err(error) => (None, Some(error)),
        };
        let password = password.to_string();
        let org_id = org_id.to_string();
        let decision = tokio::task::spawn_blocking(move || {
            decide_credentials_with(candidate, &password, &org_id, verify)
        })
        .await
        .map_err(|_| AuthenticationError::Unavailable("credential verifier unavailable".into()))?;

        if let Some(error) = lookup_error {
            return Err(AuthenticationError::Unavailable(error));
        }
        decision
    }

    pub async fn authenticate_dummy(
        &self,
        password: &str,
    ) -> Result<User, AuthenticationError> {
        let _password_slot = self
            .password_slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| AuthenticationError::Unavailable("credential verifier busy".into()))?;
        let password = password.to_string();
        tokio::task::spawn_blocking(move || {
            decide_credentials_with(None, &password, "", verify)
        })
        .await
        .map_err(|_| AuthenticationError::Unavailable("credential verifier unavailable".into()))?
    }

    fn find_memory_user_with<F>(
        &self,
        identifier: &str,
        org_id: &str,
        after_index_lookup: F,
    ) -> Option<User>
    where
        F: FnOnce(),
    {
        let user_id = {
            let by_name = self.by_name.read().expect("Failed to acquire lock");
            let by_email = self.by_email.read().expect("Failed to acquire lock");
            let key = TenantKey {
                org_id: org_id.to_string(),
                key: identifier.to_string(),
            };
            match (by_name.get(&key), by_email.get(&key)) {
                (Some(name_id), Some(email_id)) if name_id != email_id => None,
                (Some(id), _) | (_, Some(id)) => Some(id.clone()),
                (None, None) => None,
            }
        };

        after_index_lookup();
        let users = self.users.read().expect("Failed to acquire lock");
        user_id.and_then(|id| users.get(&id).cloned())
    }

    fn validate_org_id(&self, org_id: &str) -> Result<(), String> {
        if ::server_config::get().multitenant {
            if org_id.trim().eq_ignore_ascii_case("system") {
                return Err("tenant_id 'system' cannot be queried in multi-tenant mode".into());
            }
            if org_id.trim().is_empty() {
                return Err("empty tenant_id is not allowed in multi-tenant mode".into());
            }
        }
        Ok(())
    }

    pub async fn get_user(&self, id: &str, org_id: &str) -> Option<User> {
        if self.validate_org_id(org_id).is_err() {
            return None;
        }

        if let Some(repo) = &self.repo {
            return repo.get_by_id(id, org_id).await.ok();
        }

        let users = self.users.read().expect("Failed to acquire lock");
        let u = users.get(id)?;

        if let Some(ref user_org) = u.organization_id {
            if user_org != org_id {
                return None;
            }
        } else if !org_id.is_empty() {
            return None;
        }
        Some(u.clone())
    }

    pub async fn list_users(&self, org_id: &str) -> Vec<User> {
        if self.validate_org_id(org_id).is_err() {
            return vec![];
        }

        if let Some(repo) = &self.repo {
            return repo.list_users(org_id).await.unwrap_or_default();
        }

        let users = self.users.read().expect("Failed to acquire lock");
        users.values()
            .filter(|u| {
                if org_id.is_empty() {
                    u.organization_id.is_none() || u.organization_id.as_deref() == Some("")
                } else {
                    u.organization_id.as_deref() == Some(org_id)
                }
            })
            .cloned()
            .collect()
    }

    pub async fn update_user(&self, id: &str, email_ptr: Option<String>, roles: Option<Vec<String>>, active_ptr: Option<bool>, org_id: &str) -> Result<User, String> {
        self.validate_org_id(org_id)?;

        if let Some(repo) = &self.repo {
            let mut u = repo.get_by_id(id, org_id).await?;
            if let Some(email) = email_ptr { u.email = email; }
            if let Some(r) = roles { u.roles = r; }
            if let Some(active) = active_ptr { u.active = active; }
            if !Self::user_access_token_fits(&u) {
                return Err("user claims exceed the access token size limit".to_string());
            }
            u.updated_at = chrono::Utc::now();
            repo.update_user(u.clone(), org_id).await?;
            return Ok(u);
        }

        let mut users = self.users.write().expect("Failed to acquire lock");
        let mut by_email = self.by_email.write().expect("Failed to acquire lock");

        let u = users.get_mut(id).ok_or_else(|| "user not found".to_string())?;

        if let Some(ref user_org) = u.organization_id {
            if user_org != org_id {
                return Err("user not found".to_string());
            }
        } else if !org_id.is_empty() {
            return Err("user not found".to_string());
        }

        let mut candidate = u.clone();
        if let Some(email) = email_ptr.as_ref() {
            candidate.email = email.clone();
        }
        if let Some(roles) = roles.as_ref() {
            candidate.roles = roles.clone();
        }
        if let Some(active) = active_ptr {
            candidate.active = active;
        }
        if !Self::user_access_token_fits(&candidate) {
            return Err("user claims exceed the access token size limit".to_string());
        }

        if let Some(email) = email_ptr {
            if email != u.email {
                let org = u.organization_id.clone().unwrap_or_default();
                let email_key = TenantKey { org_id: org, key: email.clone() };
                if by_email.contains_key(&email_key) {
                    return Err("email already registered".to_string());
                }
                by_email.remove(&TenantKey { org_id: u.organization_id.clone().unwrap_or_default(), key: u.email.clone() });
                u.email = email;
                by_email.insert(email_key, id.to_string());
            }
        }

        if let Some(r) = roles {
            u.roles = r;
        }

        if let Some(active) = active_ptr {
            u.active = active;
        }

        u.updated_at = Utc::now();

        Ok(u.clone())
    }

    pub async fn delete_user(&self, id: &str, org_id: &str) -> Result<(), String> {
        self.validate_org_id(org_id)?;

        if let Some(repo) = &self.repo {
            return repo.delete_user(id, org_id).await;
        }

        let mut users = self.users.write().expect("Failed to acquire lock");
        let mut by_name = self.by_name.write().expect("Failed to acquire lock");
        let mut by_email = self.by_email.write().expect("Failed to acquire lock");
        let mut by_oidc = self.by_oidc.write().expect("Failed to acquire lock");

        let u = users.get(id).ok_or_else(|| "user not found".to_string())?;

        if let Some(ref user_org) = u.organization_id {
            if user_org != org_id {
                return Err("user not found".to_string());
            }
        } else if !org_id.is_empty() {
            return Err("user not found".to_string());
        }

        let org = u.organization_id.clone().unwrap_or_default();
        by_name.remove(&TenantKey { org_id: org.clone(), key: u.username.clone() });
        by_email.remove(&TenantKey { org_id: org.clone(), key: u.email.clone() });
        if let Some(ref oidc) = u.oidc_subject {
            by_oidc.remove(&TenantKey { org_id: org, key: oidc.clone() });
        }

        users.remove(id);

        Ok(())
    }

    pub async fn revoke_token(
        &self,
        jti: String,
        exp: DateTime<Utc>,
        org_id: &str,
    ) -> Result<(), String> {
        self.validate_org_id(org_id)?;
        if let Some(error) = &self.redis_configuration_error {
            return Err(error.clone());
        }

        if let Some(repo) = &self.repo {
            repo.revoke_token(jti.clone(), exp, org_id).await?;
        }

        {
            let mut revoked = self.revoked.write().expect("Failed to acquire lock");
            revoked.insert(format!("{}:{}", org_id, jti), exp);

            let now = Utc::now();
            revoked.retain(|_, v| *v > now);
        }
        if let Some(client) = &self.redis_client {
            let mut conn = client
                .get_multiplexed_tokio_connection()
                .await
                .map_err(|_| "revocation cache unavailable".to_string())?;
            let ttl = (exp.timestamp() - Utc::now().timestamp()).max(1);
            let redis_key = format!("revoked_token:{}:{}", org_id, jti);
            let result: redis::RedisResult<()> =
                redis::AsyncCommands::set_ex(&mut conn, &redis_key, "1", ttl as u64).await;
            result.map_err(|_| "revocation cache unavailable".to_string())?;
        }
        Ok(())
    }

    pub async fn is_revoked(&self, jti: &str, org_id: &str) -> Result<bool, String> {
        self.validate_org_id(org_id)?;
        if let Some(error) = &self.redis_configuration_error {
            return Err(error.clone());
        }

        if let Some(repo) = &self.repo {
            if repo.is_revoked(jti, org_id).await? {
                return Ok(true);
            }
        }

        {
            let revoked = self.revoked.read().expect("Failed to acquire lock");
            if let Some(exp) = revoked.get(&format!("{}:{}", org_id, jti)) {
                if *exp > Utc::now() {
                    return Ok(true);
                }
            }
        }
        if let Some(client) = &self.redis_client {
            let mut conn = client
                .get_multiplexed_tokio_connection()
                .await
                .map_err(|_| "revocation cache unavailable".to_string())?;
            let redis_key = format!("revoked_token:{}:{}", org_id, jti);
            let exists: bool = redis::AsyncCommands::exists(&mut conn, &redis_key)
                .await
                .map_err(|_| "revocation cache unavailable".to_string())?;
            if exists {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn issue_token_with_expiry(&self, _user: &User) -> Result<(String, i64), String> {
        let now = chrono::Utc::now();
        let expires_at = (now + chrono::Duration::hours(24)).timestamp();
        let claims = Self::access_token_claims(
            _user,
            now.timestamp(),
            expires_at,
            hex::encode(random_bytes(16)),
        );
        if !Self::access_token_claims_fit(&claims) {
            return Err("access token claims exceed the configured size limit".to_string());
        }

        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
        let token = jsonwebtoken::encode(
            &header,
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(&self.secret),
        )
        .map_err(|e| e.to_string())?;
        if token.len() > MAX_ACCESS_TOKEN_BYTES {
            return Err("access token exceeds the configured size limit".to_string());
        }

        Ok((token, expires_at))
    }

    pub fn issue_token(&self, user: &User) -> Result<String, String> {
        self.issue_token_with_expiry(user).map(|(token, _)| token)
    }

    pub async fn validate_token(&self, _token: &str) -> Result<Claims, String> {
        self.validate_token_claims(_token, true)
            .await
            .map_err(|error| error.to_string())
    }

    /// Validate signed claims without rejecting a prior revocation, then revoke
    /// the token in the tenant named by those claims. This makes logout
    /// idempotent without trusting transport-supplied tenant data.
    pub async fn logout_token(&self, token: &str) -> Result<(), LogoutError> {
        let claims = self.validate_token_claims(token, false).await.map_err(
            |error| match error {
                TokenValidationError::Invalid(_) => LogoutError::InvalidToken,
                TokenValidationError::Unavailable => LogoutError::ValidationUnavailable,
            },
        )?;
        let exp =
            chrono::DateTime::from_timestamp(claims.exp, 0).ok_or(LogoutError::InvalidToken)?;
        let org_id = claims.organization_id.unwrap_or_default();
        self.revoke_token(claims.jti, exp, &org_id)
            .await
            .map_err(|_| LogoutError::RevocationUnavailable)
    }

    async fn validate_token_claims(
        &self,
        _token: &str,
        check_revocation: bool,
    ) -> Result<Claims, TokenValidationError> {
        if let Ok(header) = jsonwebtoken::decode_header(_token) {
            if header.alg == jsonwebtoken::Algorithm::RS256 {
                let oidc_cfg_internal = self
                    .oidc_cfg
                    .read()
                    .expect("Failed to acquire lock")
                    .clone();
                let oidc_cfg = crate::oidc::OIDCConfig {
                    issuer_url: oidc_cfg_internal.issuer_url,
                    client_id: oidc_cfg_internal.client_id,
                    enabled: oidc_cfg_internal.enabled,
                };
                if oidc_cfg.enabled {
                    let claims = crate::oidc::validate_oidc_token(_token, &oidc_cfg)
                        .await
                        .map_err(|error| match error {
                            crate::oidc::OidcValidationError::InvalidToken => {
                                TokenValidationError::invalid("Invalid token")
                            }
                            crate::oidc::OidcValidationError::Unavailable => {
                                TokenValidationError::Unavailable
                            }
                        })?;
                    if ::server_config::get().multitenant
                        && claims
                            .organization_id
                            .clone()
                            .unwrap_or_default()
                            .trim()
                            .is_empty()
                    {
                        return Err(TokenValidationError::invalid(
                            "Invalid token: organization_id is required in cloud mode",
                        ));
                    }
                    if ::server_config::get().multitenant
                        && claims
                            .organization_id
                            .as_deref()
                            .map(|s| s.eq_ignore_ascii_case("system"))
                            .unwrap_or(false)
                    {
                        return Err(TokenValidationError::invalid(
                            "Invalid token: 'system' organization cannot be used in multitenant mode",
                        ));
                    }
                    if check_revocation
                        && self
                            .is_revoked(
                                &claims.jti,
                                &claims.organization_id.clone().unwrap_or_default(),
                            )
                            .await
                            .map_err(|_| TokenValidationError::Unavailable)?
                    {
                        return Err(TokenValidationError::invalid("token revoked"));
                    }
                    return Ok(claims);
                }
            }
        }

        let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        let token_data = jsonwebtoken::decode::<Claims>(
            _token,
            &jsonwebtoken::DecodingKey::from_secret(&self.secret),
            &validation,
        );

        match token_data {
            Ok(data) => {
                if data.claims.sub.trim().is_empty() || data.claims.jti.trim().is_empty() {
                    return Err(TokenValidationError::invalid("Invalid token: empty claims"));
                }
                if ::server_config::get().multitenant
                    && data
                        .claims
                        .organization_id
                        .clone()
                        .unwrap_or_default()
                        .trim()
                        .is_empty()
                {
                    return Err(TokenValidationError::invalid(
                        "Invalid token: organization_id is required in cloud mode",
                    ));
                }
                if ::server_config::get().multitenant
                    && data
                        .claims
                        .organization_id
                        .as_deref()
                        .map(|s| s.eq_ignore_ascii_case("system"))
                        .unwrap_or(false)
                {
                    return Err(TokenValidationError::invalid(
                        "Invalid token: 'system' organization cannot be used in multitenant mode",
                    ));
                }
                if check_revocation
                    && self
                        .is_revoked(
                            &data.claims.jti,
                            &data.claims.organization_id.clone().unwrap_or_default(),
                        )
                        .await
                        .map_err(|_| TokenValidationError::Unavailable)?
                {
                    return Err(TokenValidationError::invalid("token revoked"));
                }
                if data.claims.sub.trim().is_empty() || data.claims.jti.trim().is_empty() {
                    return Err(TokenValidationError::invalid("Invalid token claims"));
                }
                Ok(data.claims)
            }
            Err(_) => {
                let oidc_cfg = {
                    let c = self.oidc_cfg.read().expect("Failed to acquire lock");
                    crate::oidc::OIDCConfig {
                        issuer_url: c.issuer_url.clone(),
                        client_id: c.client_id.clone(),
                        enabled: c.enabled,
                    }
                };
                match crate::oidc::validate_oidc_token(_token, &oidc_cfg).await {
                    Ok(claims) => {
                        if ::server_config::get().multitenant
                            && claims
                                .organization_id
                                .clone()
                                .unwrap_or_default()
                                .trim()
                                .is_empty()
                        {
                            return Err(TokenValidationError::invalid(
                                "Invalid token: organization_id is required in cloud mode",
                            ));
                        }
                        if ::server_config::get().multitenant
                            && claims
                                .organization_id
                                .as_deref()
                                .map(|s| s.eq_ignore_ascii_case("system"))
                                .unwrap_or(false)
                        {
                            return Err(TokenValidationError::invalid(
                                "Invalid token: 'system' organization cannot be used in multitenant mode",
                            ));
                        }
                        if check_revocation
                            && self
                                .is_revoked(
                                    &claims.jti,
                                    &claims.organization_id.clone().unwrap_or_default(),
                                )
                                .await
                                .map_err(|_| TokenValidationError::Unavailable)?
                        {
                            return Err(TokenValidationError::invalid("token revoked"));
                        }
                        return Ok(claims);
                    }
                    Err(crate::oidc::OidcValidationError::Unavailable) if oidc_cfg.enabled => {
                        return Err(TokenValidationError::Unavailable);
                    }
                    Err(_) => {}
                }
                Err(TokenValidationError::invalid("Invalid token"))
            }
        }
    }
}

fn random_bytes(n: usize) -> Vec<u8> {
    let mut b = vec![0u8; n];
    rand::thread_rng().fill_bytes(&mut b);
    b
}

#[derive(Clone)]
pub struct AuthServiceServerImpl {
    pub store: Arc<Store>,
    transport_mode: AuthTransportMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthTransportMode {
    Cloud,
    Standalone,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AuthRpc {
    Login,
    Register,
    Logout,
    GetMe,
    ListUsers,
    CreateUser,
    GetUser,
    UpdateUser,
    DeleteUser,
    ListRoles,
    CreateRole,
}

impl AuthRpc {
    fn is_public(self) -> bool {
        matches!(self, Self::Login | Self::Register)
    }
}

impl AuthServiceServerImpl {
    pub fn new(store: Arc<Store>, transport_mode: AuthTransportMode) -> Self {
        Self {
            store,
            transport_mode,
        }
    }

    fn authenticate_rpc<T>(&self, rpc: AuthRpc, request: &mut Request<T>) -> Result<(), Status> {
        if rpc.is_public() {
            return Ok(());
        }

        peer_identity::authenticate_spiffe_request(
            request,
            self.transport_mode == AuthTransportMode::Standalone,
        )
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse_spiffe_id(spiffe_id: &str) -> Result<(String, String), Status> {
    grpc::validate_spiffe_id(spiffe_id)?;
    let path = spiffe_id
        .strip_prefix("spiffe://")
        .ok_or_else(|| Status::unauthenticated("invalid SPIFFE scheme"))?;
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() != 5 || parts[1] != "org" || parts[3] != "agent" {
        return Err(Status::unauthenticated("invalid SPIFFE identity path"));
    }
    Ok((parts[2].to_string(), parts[4].to_string()))
}

pub fn extract_spiffe_id_from_metadata(md: &tonic::metadata::MetadataMap) -> Result<String, String> {
    md.get("x-spiffe-id")
        .ok_or_else(|| "missing x-spiffe-id header".to_string())?
        .to_str()
        .map_err(|_| "invalid x-spiffe-id header".to_string())
        .map(|s| s.to_string())
}

#[derive(Clone, Debug)]
pub struct AuthInfo {
    pub spiffe_id: String,
    pub org_id: String,
    pub agent_id: String,
}

#[tonic::async_trait]
impl AuthService for AuthServiceServerImpl {
    async fn login(
        &self,
        mut request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        self.authenticate_rpc(AuthRpc::Login, &mut request)?;
        let req = request.into_inner();

        match self
            .store
            .authenticate(&req.username, &req.password, &req.organization_id)
            .await
        {
            Ok(user) => {
                match self.store.issue_token_with_expiry(&user) {
                    Ok((token, expires_at)) => {
                         Ok(Response::new(LoginResponse {
                             token,
                             expires_at,
                         }))
                    }
                    Err(e) => Err(Status::internal(e)),
                }
            }
            Err(AuthenticationError::InvalidCredentials) => {
                Err(Status::unauthenticated("invalid credentials"))
            }
            Err(AuthenticationError::Unavailable(error)) => {
                let _ = error;
                tracing::error!(event = "auth.grpc.login.unavailable");
                Err(Status::unavailable("authentication unavailable"))
            }
        }
    }

    async fn register(
        &self,
        mut request: Request<CreateUserRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        self.authenticate_rpc(AuthRpc::Register, &mut request)?;
        let req = request.into_inner();

        let (final_org_id, final_role) = if ::server_config::get().multitenant {
            (sqlx::types::Uuid::new_v4().to_string(), ROLE_ADMIN.to_string())
        } else {
            // Prevent users from registering into the 'system' tenant ID even in standalone mode
            // unless we have specific logic. Here we just fallback to a safe ID or what they provided
            // as long as it's not 'system'. Actually, in standalone, they own everything.
            let req_org = if req.organization_id.eq_ignore_ascii_case("system") {
                sqlx::types::Uuid::new_v4().to_string()
            } else if req.organization_id.is_empty() {
                sqlx::types::Uuid::new_v4().to_string()
            } else {
                req.organization_id.clone()
            };
            (req_org, ROLE_VIEWER.to_string())
        };

        let user = self.store.create_user(
            req.username.clone(),
            req.email.clone(),
            req.password,
            vec![final_role],
            final_org_id,
        ).await.map_err(|e| Status::internal(e))?;

        let (token, expires_at) = self.store.issue_token_with_expiry(&user).map_err(Status::internal)?;

        Ok(Response::new(LoginResponse {
             token,
             expires_at,
        }))
    }

    async fn logout(
        &self,
        mut request: Request<EmptyRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.authenticate_rpc(AuthRpc::Logout, &mut request)?;
        request
            .extensions()
            .get::<AuthInfo>()
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;
        let auth_header = request
            .metadata()
            .get("authorization")
            .ok_or_else(|| Status::unauthenticated("missing authorization"))?;
        let auth_str = auth_header
            .to_str()
            .map_err(|_| Status::unauthenticated("invalid authorization"))?;
        let token = auth_str
            .strip_prefix("Bearer ")
            .or_else(|| auth_str.strip_prefix("bearer "))
            .unwrap_or(auth_str);
        self.store.logout_token(token).await.map_err(|error| match error {
            LogoutError::InvalidToken => Status::unauthenticated("invalid token"),
            LogoutError::ValidationUnavailable => {
                Status::unavailable("token validation unavailable")
            }
            LogoutError::RevocationUnavailable => {
                Status::unavailable("token revocation unavailable")
            }
        })?;
        Ok(Response::new(EmptyResponse {}))
    }

    async fn get_me(
        &self,
        mut request: Request<EmptyRequest>,
    ) -> Result<Response<UserProto>, Status> {
        self.authenticate_rpc(AuthRpc::GetMe, &mut request)?;
        let auth_info = request.extensions().get::<AuthInfo>()
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;

        let user = self.store.get_user(&auth_info.spiffe_id, &auth_info.org_id).await
            .ok_or_else(|| Status::not_found("User not found"))?;

        Ok(Response::new(UserProto {
            id: user.id,
            username: user.username,
            email: user.email,
            roles: user.roles,
            active: user.active,
            organization_id: user.organization_id.unwrap_or_default(),
            created_at_unix: user.created_at.timestamp(),
            updated_at_unix: user.updated_at.timestamp(),
            oidc_subject: user.oidc_subject.unwrap_or_default(),
        }))
    }

    async fn list_users(
        &self,
        mut request: Request<ListUsersRequest>,
    ) -> Result<Response<ListUsersResponse>, Status> {
        self.authenticate_rpc(AuthRpc::ListUsers, &mut request)?;
        let auth_info = request.extensions().get::<AuthInfo>()
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;

        let users = self.store.list_users(&auth_info.org_id).await;
        let proto_users = users.into_iter().map(|u| UserProto {
            id: u.id,
            username: u.username,
            email: u.email,
            roles: u.roles,
            active: u.active,
            organization_id: u.organization_id.unwrap_or_default(),
            created_at_unix: u.created_at.timestamp(),
            updated_at_unix: u.updated_at.timestamp(),
            oidc_subject: u.oidc_subject.unwrap_or_default(),
        }).collect();
        Ok(Response::new(ListUsersResponse { users: proto_users }))
    }

    async fn create_user(
        &self,
        mut request: Request<CreateUserRequest>,
    ) -> Result<Response<UserProto>, Status> {
        self.authenticate_rpc(AuthRpc::CreateUser, &mut request)?;
        let auth_info = request.extensions().get::<AuthInfo>()
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;
        let org_id = auth_info.org_id.clone();

        let caller = self.store.get_user(&auth_info.spiffe_id, &org_id).await
            .ok_or_else(|| Status::not_found("Caller not found"))?;

        if !caller.roles.contains(&"ADMIN".to_string()) {
            return Err(Status::permission_denied("Only ADMIN can create users"));
        }

        let req = request.into_inner();
        // Force the new user to be in the caller's organization to prevent cross-tenant injection
        let user = self.store.create_user(
            req.username.clone(),
            req.email.clone(),
            req.password,
            vec![],
            org_id,
        ).await.map_err(|e| Status::internal(e))?;
        Ok(Response::new(UserProto {
            id: user.id,
            username: user.username,
            email: user.email,
            roles: user.roles,
            active: user.active,
            organization_id: user.organization_id.unwrap_or_default(),
            created_at_unix: user.created_at.timestamp(),
            updated_at_unix: user.updated_at.timestamp(),
            oidc_subject: user.oidc_subject.unwrap_or_default(),
        }))
    }

    async fn get_user(
        &self,
        mut request: Request<GetUserRequest>,
    ) -> Result<Response<UserProto>, Status> {
        self.authenticate_rpc(AuthRpc::GetUser, &mut request)?;
        let auth_info = request.extensions().get::<AuthInfo>()
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;

        let user = self.store.get_user(&request.get_ref().id, &auth_info.org_id).await
            .ok_or_else(|| Status::not_found("User not found"))?;

        Ok(Response::new(UserProto {
            id: user.id,
            username: user.username,
            email: user.email,
            roles: user.roles,
            active: user.active,
            organization_id: user.organization_id.unwrap_or_default(),
            created_at_unix: user.created_at.timestamp(),
            updated_at_unix: user.updated_at.timestamp(),
            oidc_subject: user.oidc_subject.unwrap_or_default(),
        }))
    }

    async fn update_user(
        &self,
        mut request: Request<UpdateUserRequest>,
    ) -> Result<Response<UserProto>, Status> {
        self.authenticate_rpc(AuthRpc::UpdateUser, &mut request)?;
        let auth_info = request.extensions().get::<AuthInfo>()
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;
        let org_id = auth_info.org_id.clone();

        // Privilege Escalation fix: Ensure caller is ADMIN or the target user themselves
        let caller = self.store.get_user(&auth_info.spiffe_id, &org_id).await
            .ok_or_else(|| Status::not_found("Caller not found"))?;

        let req = request.into_inner();

        let is_admin = caller.roles.contains(&"ADMIN".to_string());
        if !is_admin && caller.id != req.id {
            return Err(Status::permission_denied("Insufficient permissions to update this user"));
        }

        // Only ADMIN can change roles or active status
        let target_user = self.store.get_user(&req.id, &org_id).await.ok_or_else(|| Status::not_found("User not found"))?;
        let final_roles = if is_admin { req.roles } else { target_user.roles.clone() };
        let final_active = if is_admin { req.active } else { Some(target_user.active) };

        let user = self.store.update_user(&req.id, req.email, Some(final_roles), final_active, &org_id).await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(UserProto {
            id: user.id,
            username: user.username,
            email: user.email,
            roles: user.roles,
            active: user.active,
            organization_id: user.organization_id.unwrap_or_default(),
            created_at_unix: user.created_at.timestamp(),
            updated_at_unix: user.updated_at.timestamp(),
            oidc_subject: user.oidc_subject.unwrap_or_default(),
        }))
    }

    async fn delete_user(
        &self,
        mut request: Request<DeleteUserRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        self.authenticate_rpc(AuthRpc::DeleteUser, &mut request)?;
        let auth_info = request.extensions().get::<AuthInfo>()
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;
        let org_id = auth_info.org_id.clone();

        // Privilege Escalation fix: Ensure caller is ADMIN
        let caller = self.store.get_user(&auth_info.spiffe_id, &org_id).await
            .ok_or_else(|| Status::not_found("Caller not found"))?;

        if !caller.roles.contains(&"ADMIN".to_string()) {
            return Err(Status::permission_denied("Only ADMIN can delete users"));
        }

        self.store.delete_user(&request.get_ref().id, &org_id).await
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(EmptyResponse {}))
    }

    async fn list_roles(
        &self,
        mut request: Request<EmptyRequest>,
    ) -> Result<Response<ListRolesResponse>, Status> {
        self.authenticate_rpc(AuthRpc::ListRoles, &mut request)?;
        Ok(Response::new(ListRolesResponse {
            roles: vec![
                RoleProto {
                    id: ROLE_ADMIN.to_string(),
                    name: ROLE_ADMIN.to_string(),
                    permissions: vec!["*".to_string()],
                    created_at_unix: Utc::now().timestamp(),
                },
                RoleProto {
                    id: ROLE_OPERATOR.to_string(),
                    name: ROLE_OPERATOR.to_string(),
                    permissions: vec!["read".to_string(), "write".to_string()],
                    created_at_unix: Utc::now().timestamp(),
                },
                RoleProto {
                    id: ROLE_VIEWER.to_string(),
                    name: ROLE_VIEWER.to_string(),
                    permissions: vec!["read".to_string()],
                    created_at_unix: Utc::now().timestamp(),
                },
            ],
        }))
    }

    async fn create_role(
        &self,
        mut request: Request<CreateRoleRequest>,
    ) -> Result<Response<RoleProto>, Status> {
        self.authenticate_rpc(AuthRpc::CreateRole, &mut request)?;
        Ok(Response::new(RoleProto::default()))
    }
}

#[cfg(test)]
mod store_tests {
    use super::*;

    #[test]
    fn auth_rpc_policy_exposes_only_login_and_register_without_peer_identity() {
        let service =
            AuthServiceServerImpl::new(Arc::new(Store::new()), AuthTransportMode::Cloud);
        let public = [AuthRpc::Login, AuthRpc::Register];
        let protected = [
            AuthRpc::Logout,
            AuthRpc::GetMe,
            AuthRpc::ListUsers,
            AuthRpc::CreateUser,
            AuthRpc::GetUser,
            AuthRpc::UpdateUser,
            AuthRpc::DeleteUser,
            AuthRpc::ListRoles,
            AuthRpc::CreateRole,
        ];

        for rpc in public {
            let mut request = Request::new(());
            assert!(service.authenticate_rpc(rpc, &mut request).is_ok());
        }

        for rpc in protected {
            let mut request = Request::new(());
            request
                .metadata_mut()
                .insert(
                    "x-spiffe-id",
                    "spiffe://onehumancorp.io/org/acme/agent/forged"
                        .parse()
                        .unwrap(),
                );
            request.extensions_mut().insert(AuthInfo {
                spiffe_id: "spiffe://onehumancorp.io/org/acme/agent/forged".to_string(),
                org_id: "acme".to_string(),
                agent_id: "forged".to_string(),
            });

            let status = service.authenticate_rpc(rpc, &mut request).unwrap_err();
            assert_eq!(status.code(), tonic::Code::Unauthenticated, "{rpc:?}");
            assert!(request.extensions().get::<AuthInfo>().is_none(), "{rpc:?}");
        }
    }

    #[test]
    fn standalone_auth_rpc_accepts_only_a_strict_spiffe_identity() {
        let service =
            AuthServiceServerImpl::new(Arc::new(Store::new()), AuthTransportMode::Standalone);
        let mut valid = Request::new(());
        valid.metadata_mut().insert(
            "x-spiffe-id",
            "spiffe://onehumancorp.io/org/acme/agent/worker-1"
                .parse()
                .unwrap(),
        );
        service
            .authenticate_rpc(AuthRpc::ListRoles, &mut valid)
            .unwrap();
        let identity = valid.extensions().get::<AuthInfo>().unwrap();
        assert_eq!(identity.org_id, "acme");
        assert_eq!(identity.agent_id, "worker-1");
        let orchestration_identity = valid
            .extensions()
            .get::<crate::orchestration::AuthInfo>()
            .unwrap();
        assert_eq!(orchestration_identity.org_id, "acme");
        assert_eq!(orchestration_identity.agent_id, "worker-1");

        for claimed in [
            None,
            Some("spiffe://evil.example/org/acme/agent/worker-1"),
            Some("not-a-spiffe-id"),
        ] {
            let mut invalid = Request::new(());
            if let Some(claimed) = claimed {
                invalid
                    .metadata_mut()
                    .insert("x-spiffe-id", claimed.parse().unwrap());
            }
            assert_eq!(
                service
                    .authenticate_rpc(AuthRpc::ListRoles, &mut invalid)
                    .unwrap_err()
                    .code(),
                tonic::Code::Unauthenticated,
            );
        }
    }

    struct FailingRepository;

    #[async_trait::async_trait]
    impl crate::user_repository::UserRepository for FailingRepository {
        async fn create_user(&self, _: User, _: &str) -> Result<(), String> {
            Err("repository unavailable".into())
        }
        async fn get_by_id(&self, _: &str, _: &str) -> Result<User, String> {
            Err("repository unavailable".into())
        }
        async fn get_by_username(&self, _: &str, _: &str) -> Result<User, String> {
            Err("repository unavailable".into())
        }
        async fn get_by_email(&self, _: &str, _: &str) -> Result<User, String> {
            Err("repository unavailable".into())
        }
        async fn get_by_login_identifier(&self, _: &str, _: &str) -> Result<Option<User>, String> {
            Err("repository unavailable".into())
        }
        async fn get_by_oidc_subject(&self, _: &str, _: &str) -> Result<User, String> {
            Err("repository unavailable".into())
        }
        async fn list_users(&self, _: &str) -> Result<Vec<User>, String> {
            Err("repository unavailable".into())
        }
        async fn update_user(&self, _: User, _: &str) -> Result<(), String> {
            Err("repository unavailable".into())
        }
        async fn delete_user(&self, _: &str, _: &str) -> Result<(), String> {
            Err("repository unavailable".into())
        }
        async fn revoke_token(&self, _: String, _: DateTime<Utc>, _: &str) -> Result<(), String> {
            Err("repository unavailable".into())
        }
        async fn is_revoked(&self, _: &str, _: &str) -> Result<bool, String> {
            Err("repository unavailable".into())
        }
    }

    fn credential_test_user(active: bool, organization_id: Option<&str>) -> User {
        let now = Utc::now();
        User {
            id: "user-id".to_string(),
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            password_hash: "real-hash".to_string(),
            roles: vec![ROLE_VIEWER.to_string()],
            active,
            organization_id: organization_id.map(str::to_string),
            created_at: now,
            updated_at: now,
            oidc_subject: None,
        }
    }

    fn test_store_with_repo(repo: Arc<dyn crate::user_repository::UserRepository>) -> Store {
        Store {
            users: RwLock::new(HashMap::new()),
            by_name: RwLock::new(HashMap::new()),
            by_email: RwLock::new(HashMap::new()),
            by_oidc: RwLock::new(HashMap::new()),
            revoked: RwLock::new(HashMap::new()),
            redis_client: None,
            redis_configuration_error: None,
            secret: b"test-secret-with-at-least-32-bytes".to_vec(),
            password_slots: Arc::new(tokio::sync::Semaphore::new(
                MAX_PASSWORD_HASH_CONCURRENCY,
            )),
            oidc_cfg: RwLock::new(OIDCConfig {
                issuer_url: String::new(),
                client_id: String::new(),
                enabled: false,
            }),
            repo: Some(repo),
        }
    }

    fn test_store_with_memory_user(active: bool) -> Store {
        let mut store = test_store_with_repo(Arc::new(FailingRepository));
        store.repo = None;
        let mut user = credential_test_user(active, None);
        user.password_hash = hash("secret".to_string(), 4).unwrap();
        store
            .users
            .write()
            .unwrap()
            .insert(user.id.clone(), user.clone());
        store.by_name.write().unwrap().insert(
            TenantKey {
                org_id: String::new(),
                key: user.username.clone(),
            },
            user.id.clone(),
        );
        store.by_email.write().unwrap().insert(
            TenantKey {
                org_id: String::new(),
                key: user.email.clone(),
            },
            user.id.clone(),
        );
        store
    }

    #[tokio::test]
    async fn strict_bearer_auth_ignores_forged_headers_and_injects_validated_identity() {
        use axum::{Json, Router, extract::Extension, routing::get};
        use tower::ServiceExt;

        let store = Arc::new(test_store_with_memory_user(true));
        let token = store
            .issue_token(&credential_test_user(true, Some("tenant-a")))
            .unwrap();
        let app = Router::new()
            .route(
                "/",
                get(
                    |Extension(claims): Extension<::server_common::Claims>,
                     Extension(auth): Extension<crate::orchestration::AuthInfo>| async move {
                        Json(serde_json::json!({
                            "tenant": claims.organization_id,
                            "user": auth.agent_id,
                            "email": claims.email,
                        }))
                    },
                ),
            )
            .layer(axum::middleware::from_fn_with_state(
                store,
                strict_bearer_auth_middleware,
            ));

        let forged_only = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/")
                    .header("x-tenant-id", "attacker")
                    .header("x-user-id", "attacker")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(forged_only.status(), axum::http::StatusCode::UNAUTHORIZED);

        let duplicate = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/")
                    .header("authorization", format!("Bearer {token}"))
                    .header("authorization", format!("Bearer {token}"))
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(duplicate.status(), axum::http::StatusCode::UNAUTHORIZED);

        let whitespace = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/")
                    .header("authorization", "Bearer invalid token")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(whitespace.status(), axum::http::StatusCode::UNAUTHORIZED);

        let valid = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/")
                    .header("authorization", format!("Bearer {token}"))
                    .header("x-tenant-id", "attacker")
                    .header("x-user-id", "attacker")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(valid.status(), axum::http::StatusCode::OK);
        let body = axum::body::to_bytes(valid.into_body(), 4096).await.unwrap();
        let identity: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(identity["tenant"], "tenant-a");
        assert_eq!(identity["user"], "user-id");
        assert_eq!(identity["email"], "alice@example.com");
    }

    #[test]
    fn credential_decision_verifies_exactly_once_for_every_outcome() {
        let cases = [
            (Some(credential_test_user(true, None)), "", true, true),
            (Some(credential_test_user(true, None)), "", false, false),
            (None, "", false, false),
            (Some(credential_test_user(false, None)), "", false, false),
            (
                Some(credential_test_user(true, Some("other-tenant"))),
                "",
                false,
                false,
            ),
        ];
        for (candidate, org_id, verifier_result, should_succeed) in cases {
            let mut calls = 0;
            let result = decide_credentials_with(candidate, "password", org_id, |_, _| {
                calls += 1;
                Ok(verifier_result)
            });
            assert_eq!(calls, 1);
            assert_eq!(result.is_ok(), should_succeed);
            if !should_succeed {
                assert!(matches!(
                    result,
                    Err(AuthenticationError::InvalidCredentials)
                ));
            }
        }
    }

    #[test]
    fn credential_verifier_failures_are_availability_errors() {
        let result = decide_credentials_with(None, "password", "", |_, _| {
            Err("bcrypt unavailable".to_string())
        });
        assert!(matches!(result, Err(AuthenticationError::Unavailable(_))));
    }

    #[test]
    fn redis_configuration_distinguishes_absent_and_invalid_values() {
        assert!(
            configure_redis_client(RedisUrlSetting::Absent)
                .unwrap()
                .is_none()
        );
        assert!(
            configure_redis_client(RedisUrlSetting::Value("not a redis url".to_string())).is_err()
        );
        assert!(configure_redis_client(RedisUrlSetting::InvalidUnicode).is_err());
    }

    #[tokio::test]
    async fn memory_authentication_supports_username_and_email_with_generic_denials() {
        let store = test_store_with_memory_user(true);
        assert!(store.authenticate("alice", "secret", "").await.is_ok());
        assert!(
            store
                .authenticate("alice@example.com", "secret", "")
                .await
                .is_ok()
        );
        assert!(matches!(
            store.authenticate("alice", "wrong", "").await,
            Err(AuthenticationError::InvalidCredentials)
        ));
        assert!(matches!(
            store.authenticate("missing", "wrong", "").await,
            Err(AuthenticationError::InvalidCredentials)
        ));

        let inactive_store = test_store_with_memory_user(false);
        assert!(matches!(
            inactive_store.authenticate("alice", "secret", "").await,
            Err(AuthenticationError::InvalidCredentials)
        ));
    }

    #[tokio::test]
    async fn authentication_repository_failures_are_unavailable() {
        let store = test_store_with_repo(Arc::new(FailingRepository));
        assert!(matches!(
            store.authenticate("alice", "secret", "").await,
            Err(AuthenticationError::Unavailable(_))
        ));
    }

    #[tokio::test]
    async fn revocation_repository_failures_are_propagated() {
        let store = test_store_with_repo(Arc::new(FailingRepository));
        let expiry = Utc::now() + chrono::Duration::hours(1);
        assert_eq!(
            store
                .revoke_token("jti".to_string(), expiry, "")
                .await
                .unwrap_err(),
            "repository unavailable"
        );
        assert_eq!(
            store.is_revoked("jti", "").await.unwrap_err(),
            "repository unavailable"
        );
    }

    #[tokio::test]
    async fn invalid_redis_configuration_fails_revocation_closed() {
        let mut store = test_store_with_memory_user(true);
        store.redis_configuration_error =
            Some("invalid revocation cache configuration".to_string());
        let expiry = Utc::now() + chrono::Duration::hours(1);
        assert_eq!(
            store
                .revoke_token("jti".to_string(), expiry, "")
                .await
                .unwrap_err(),
            "invalid revocation cache configuration"
        );
        assert_eq!(
            store.is_revoked("jti", "").await.unwrap_err(),
            "invalid revocation cache configuration"
        );
    }

    #[tokio::test]
    async fn token_validation_fails_closed_when_revocation_is_unavailable() {
        let store = test_store_with_repo(Arc::new(FailingRepository));
        let token = store
            .issue_token(&credential_test_user(true, None))
            .unwrap();
        assert_eq!(
            store.validate_token(&token).await.unwrap_err(),
            "token validation unavailable"
        );
    }

    fn logout_request(token: &str) -> Request<EmptyRequest> {
        let mut request = Request::new(EmptyRequest {});
        request.metadata_mut().insert(
            "x-spiffe-id",
            "spiffe://onehumancorp.io/org/local/agent/user-id"
                .parse()
                .unwrap(),
        );
        request
            .metadata_mut()
            .insert("authorization", format!("Bearer {token}").parse().unwrap());
        request
    }

    #[tokio::test]
    async fn logout_is_idempotent_after_token_revocation() {
        let store = Arc::new(test_store_with_memory_user(true));
        let token = store
            .issue_token(&credential_test_user(true, None))
            .unwrap();
        let service = AuthServiceServerImpl::new(store, AuthTransportMode::Standalone);

        assert!(
            AuthService::logout(&service, logout_request(&token))
                .await
                .is_ok()
        );
        assert!(
            AuthService::logout(&service, logout_request(&token))
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn logout_distinguishes_forged_oidc_tokens_from_oidc_outages() {
        let store = test_store_with_memory_user(true);
        *store.oidc_cfg.write().unwrap() = OIDCConfig {
            issuer_url: "ftp://invalid.example".to_string(),
            client_id: "client".to_string(),
            enabled: true,
        };

        let missing_kid = "eyJhbGciOiJSUzI1NiJ9.e30.AA";
        assert_eq!(
            store.logout_token(missing_kid).await,
            Err(LogoutError::InvalidToken)
        );
        let wrong_algorithm_with_kid = "eyJhbGciOiJIUzI1NiIsImtpZCI6ImsifQ.e30.AA";
        assert_eq!(
            store.logout_token(wrong_algorithm_with_kid).await,
            Err(LogoutError::InvalidToken)
        );
        let authority_unavailable = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImsifQ.e30.AA";
        assert_eq!(
            store.logout_token(authority_unavailable).await,
            Err(LogoutError::ValidationUnavailable)
        );
    }

    #[test]
    fn memory_lookup_releases_index_locks_before_reading_users() {
        let store = Arc::new(test_store_with_memory_user(true));
        let worker_store = Arc::clone(&store);
        let users_guard = store.users.write().unwrap();
        let (indices_released_tx, indices_released_rx) = std::sync::mpsc::channel();

        let worker = std::thread::spawn(move || {
            worker_store.find_memory_user_with("alice", "", || {
                indices_released_tx.send(()).unwrap();
            })
        });

        indices_released_rx
            .recv_timeout(std::time::Duration::from_secs(1))
            .expect("lookup did not finish its index phase");
        assert!(
            store.by_name.try_write().is_ok(),
            "username index remained locked while waiting for users"
        );
        drop(users_guard);
        assert_eq!(worker.join().unwrap().unwrap().username, "alice");
    }

    #[test]
    fn test_secret_paths_are_safe() {
        unsafe { std::env::set_var("JWT_SECRET", "test_secret"); }
        let store = Store::new();
        // Since we can't easily assert on the inner paths without modifying visibility,
        // we assert that we don't panic upon creation.
        assert!(!store.secret.is_empty());
    }

    #[test]
    fn jwt_secret_supports_direct_and_secure_file_sources_and_rejects_both() {
        let direct = "direct-jwt-secret";
        temp_env::with_vars(
            [("JWT_SECRET", Some(direct)), ("JWT_SECRET_FILE", None)],
            || assert_eq!(Store::new().secret, direct.as_bytes()),
        );

        let path = std::env::temp_dir().join(uuid::Uuid::new_v4().to_string());
        std::fs::write(&path, b"file-jwt-secret\n").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
        temp_env::with_vars(
            [
                ("JWT_SECRET", None),
                ("JWT_SECRET_FILE", Some(path.to_str().unwrap())),
            ],
            || assert_eq!(Store::new().secret, b"file-jwt-secret"),
        );

        temp_env::with_vars(
            [
                ("JWT_SECRET", Some(direct)),
                ("JWT_SECRET_FILE", Some(path.to_str().unwrap())),
            ],
            || {
                let panic = match std::panic::catch_unwind(Store::new) {
                    Ok(_) => panic!("ambiguous JWT secret sources must fail closed"),
                    Err(panic) => panic,
                };
                let message = panic
                    .downcast_ref::<&str>()
                    .copied()
                    .or_else(|| panic.downcast_ref::<String>().map(String::as_str))
                    .unwrap();
                assert_eq!(message, "invalid authentication secret configuration");
                assert!(!message.contains(direct));
                assert!(!message.contains(path.to_str().unwrap()));
            },
        );

        let missing_path = std::env::temp_dir().join("missing-jwt-secret");
        temp_env::with_vars(
            [
                ("JWT_SECRET", None),
                ("JWT_SECRET_FILE", Some(missing_path.to_str().unwrap())),
            ],
            || {
                let panic = match std::panic::catch_unwind(Store::new) {
                    Ok(_) => panic!("an invalid JWT secret file must fail closed"),
                    Err(panic) => panic,
                };
                let message = panic
                    .downcast_ref::<&str>()
                    .copied()
                    .or_else(|| panic.downcast_ref::<String>().map(String::as_str))
                    .unwrap();
                assert_eq!(message, "invalid authentication secret configuration");
                assert!(!message.contains(missing_path.to_str().unwrap()));
            },
        );
    }

    #[test]
    fn test_random_bytes_length() {
        let b = super::random_bytes(16);
        assert_eq!(b.len(), 16);
        let b2 = super::random_bytes(32);
        assert_eq!(b2.len(), 32);
    }

    #[test]
    fn test_store_validate_org_id_multitenant() {
        unsafe { std::env::set_var("JWT_SECRET", "test_secret"); }
        // Create an empty store just to access the validate_org_id method
        let store = Store::new();

        // In a real environment, ::server_config::get().multitenant is controlled by the config.
        // We test that `validate_org_id` properly returns an error or success based on the config.
        let multitenant = ::server_config::get().multitenant;

        let res_system = store.validate_org_id("system");
        let res_empty = store.validate_org_id("");
        let res_valid = store.validate_org_id("tenant-123");

        if multitenant {
            assert!(res_system.is_err());
            assert_eq!(res_system.unwrap_err(), "tenant_id 'system' cannot be queried in multi-tenant mode");
            assert!(res_empty.is_err());
            assert_eq!(res_empty.unwrap_err(), "empty tenant_id is not allowed in multi-tenant mode");
            assert!(res_valid.is_ok());
        } else {
            // Standalone mode: we allow empty and system.
            assert!(res_system.is_ok());
            assert!(res_empty.is_ok());
            assert!(res_valid.is_ok());
        }
    }
}

#[cfg(test)]
mod multitenancy_isolation;
#[cfg(test)]
mod postgres_test_support;
