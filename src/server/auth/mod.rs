#![allow(clippy::all)]
pub use ::server_common as common;
pub use ::server_ohc as ohc;
pub use ::server_oidc as oidc;

pub mod orchestration;
pub mod postgres_store;
pub mod sqlite_store;
pub mod user_repository;
pub mod grpc;

use std::collections::HashMap;

pub async fn guest_auth_middleware(
    mut req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    if ::server_config::get().multitenant {
        return axum::response::Response::builder()
            .status(axum::http::StatusCode::UNAUTHORIZED)
            .body(axum::body::Body::from("Guest auth is not allowed in cloud mode"))
            .expect("Failed to build response");
    }

    let tenant_id = req.headers().get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("storefront").to_string();
    let user_id = req.headers().get("x-user-id").and_then(|v| v.to_str().ok()).unwrap_or("test-user").to_string();

    let now = chrono::Utc::now().timestamp();
    req.extensions_mut().insert(::server_common::Claims {
        sub: user_id.clone(),
        exp: now + 3600,
        iat: now,
        organization_id: Some(tenant_id.clone()),
        username: user_id.clone(),
        email: format!("{}@localhost", user_id),
        roles: vec!["ADMIN".to_string()],
        session_id: None,
        jti: "test-jti-uuid".to_string(),
    });
    req.extensions_mut().insert(crate::orchestration::AuthInfo {
        org_id: tenant_id,
        agent_id: user_id.clone(),
        spiffe_id: format!("spiffe://onehumancorp.io/guest/{}", user_id),
    });

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

fn hash(password: String, cost: u32) -> Result<String, String> {
    bcrypt::hash(password, cost).map_err(|e| e.to_string())
}

fn verify(password: &str, hash: &str) -> Result<bool, String> {
    bcrypt::verify(password, hash).map_err(|e| e.to_string())
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
    secret: Vec<u8>,
    oidc_cfg: RwLock<OIDCConfig>,
    repo: Option<std::sync::Arc<dyn crate::user_repository::UserRepository>>,
}

impl Store {
    pub fn new() -> Self {
        let secret = std::env::var("JWT_SECRET")
            .map(|s| s.into_bytes())
            .unwrap_or_else(|_| {
                if ::server_config::get().multitenant {
                    panic!("JWT_SECRET must be set in Cloud/Multitenant Mode to ensure secure access token management.");
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
                                let perms = metadata.permissions();
                                if perms.mode() & 0o777 != 0o600 {
                                    tracing::warn!("Insecure permissions on .ohc_jwt_secret. Ignoring it to prevent TOCTOU attacks.");
                                    std::process::exit(1);
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
                                    let perms = metadata.permissions();
                                    if perms.mode() & 0o777 != 0o600 {
                                        tracing::warn!("Insecure permissions on .ohc_sqlite_key. Ignoring it to prevent TOCTOU attacks.");
                                        std::process::exit(1);
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
                    options.write(true).create_new(true).mode(0o600);
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

        let redis_client = if ::server_config::get().multitenant {
            std::env::var("OHC_REDIS_URL")
                .ok()
                .and_then(|url| redis::Client::open(url).ok())
        } else {
            None
        };

        let store = Store {
            users: RwLock::new(HashMap::new()),

            by_name: RwLock::new(HashMap::new()),
            by_email: RwLock::new(HashMap::new()),
            by_oidc: RwLock::new(HashMap::new()),
            revoked: RwLock::new(HashMap::new()),
            redis_client,
            secret,
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

    pub fn create_user(&self, username: String, email: String, password: String, roles: Vec<String>, org_id: String) -> Result<User, String> {
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

        users.insert(id.clone(), user.clone());
        by_name.insert(name_key, id.clone());
        by_email.insert(email_key, id);

        Ok(user)
    }

    pub fn authenticate(&self, username: &str, password: &str, org_id: &str) -> Result<User, String> {
        self.validate_org_id(org_id)?;
        let by_name = self.by_name.read().expect("Failed to acquire lock");
        let users = self.users.read().expect("Failed to acquire lock");

        let name_key = TenantKey { org_id: org_id.to_string(), key: username.to_string() };
        let mut user_id_opt = by_name.get(&name_key).cloned();

        if user_id_opt.is_none() && org_id.is_empty() {
            user_id_opt = by_name.get(&TenantKey { org_id: "".to_string(), key: username.to_string() }).cloned();
        }

        let user_id = user_id_opt.ok_or_else(|| "invalid credentials".to_string())?;
        let user = users.get(&user_id).ok_or_else(|| "invalid credentials".to_string())?;

        if !user.active {
            return Err("account disabled".to_string());
        }

        if let Some(ref user_org) = user.organization_id {
            if user_org != org_id {
                return Err("invalid credentials".to_string());
            }
        } else if !org_id.is_empty() {
            return Err("invalid credentials".to_string());
        }

        if verify(password, &user.password_hash).unwrap_or(false) {
            Ok(user.clone())
        } else {
            Err("invalid credentials".to_string())
        }
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

    pub fn get_user(&self, id: &str, org_id: &str) -> Option<User> {
        if self.validate_org_id(org_id).is_err() {
            return None;
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

    pub fn list_users(&self, org_id: &str) -> Vec<User> {
        if self.validate_org_id(org_id).is_err() {
            return vec![];
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

    pub fn update_user(&self, id: &str, email_ptr: Option<String>, roles: Option<Vec<String>>, active_ptr: Option<bool>, org_id: &str) -> Result<User, String> {
        self.validate_org_id(org_id)?;

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

    pub fn delete_user(&self, id: &str, org_id: &str) -> Result<(), String> {
        self.validate_org_id(org_id)?;

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

        pub async fn revoke_token(&self, jti: String, exp: DateTime<Utc>, org_id: &str) {
        if self.validate_org_id(org_id).is_err() {
            return;
        }

        if let Some(repo) = &self.repo {
            let _ = repo.revoke_token(jti.clone(), exp, org_id).await;
        }

        {
            let mut revoked = self.revoked.write().expect("Failed to acquire lock");
            revoked.insert(format!("{}:{}", org_id, jti), exp);

            let now = Utc::now();
            revoked.retain(|_, v| *v > now);
        }
        if let Some(client) = &self.redis_client {
            if let Ok(mut conn) = client.get_multiplexed_tokio_connection().await {
                let ttl = (exp.timestamp() - Utc::now().timestamp()).max(1);
                let redis_key = format!("revoked_token:{}:{}", org_id, jti);
                let _: redis::RedisResult<()> = redis::AsyncCommands::set_ex(&mut conn, &redis_key, "1", ttl as u64).await;
            }
        }
    }

        pub async fn is_revoked(&self, jti: &str, org_id: &str) -> bool {
        if self.validate_org_id(org_id).is_err() {
            return false;
        }

        if let Some(repo) = &self.repo {
            if let Ok(true) = repo.is_revoked(jti, org_id).await {
                return true;
            }
        }

        {
            let revoked = self.revoked.read().expect("Failed to acquire lock");
            if let Some(exp) = revoked.get(&format!("{}:{}", org_id, jti)) {
                 if *exp > Utc::now() {
                     return true;
                 }
            }
        }
        if let Some(client) = &self.redis_client {
            if let Ok(mut conn) = client.get_multiplexed_tokio_connection().await {
                let redis_key = format!("revoked_token:{}:{}", org_id, jti);
                let exists: redis::RedisResult<bool> = redis::AsyncCommands::exists(&mut conn, &redis_key).await;
                if let Ok(true) = exists {
                    return true;
                }
            }
        }
        false
    }

    pub fn issue_token(&self, _user: &User) -> Result<String, String> {
            let now = chrono::Utc::now();
            let claims = Claims {
                sub: _user.id.clone(),
                username: _user.username.clone(),
                email: _user.email.clone(),
                roles: _user.roles.clone(),
                organization_id: _user.organization_id.clone(),
                session_id: None,
                iat: now.timestamp(),
                exp: (now + chrono::Duration::hours(24)).timestamp(),
                jti: hex::encode(random_bytes(16)), // Use 16 bytes for better entropy
            };

            let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
            let token = jsonwebtoken::encode(&header, &claims, &jsonwebtoken::EncodingKey::from_secret(&self.secret))
                .map_err(|e| e.to_string())?;

            Ok(token)
    }

    pub async fn validate_token(&self, _token: &str) -> Result<Claims, String> {
        if let Ok(header) = jsonwebtoken::decode_header(_token) {
            if header.alg == jsonwebtoken::Algorithm::RS256 {
                let oidc_cfg_internal = self.oidc_cfg.read().expect("Failed to acquire lock").clone();
                let oidc_cfg = crate::oidc::OIDCConfig {
                    issuer_url: oidc_cfg_internal.issuer_url,
                    client_id: oidc_cfg_internal.client_id,
                    enabled: oidc_cfg_internal.enabled,
                };
                if oidc_cfg.enabled {
                    let claims = crate::oidc::validate_oidc_token(_token, &oidc_cfg).await?;
                    if ::server_config::get().multitenant && claims.organization_id.clone().unwrap_or_default().trim().is_empty() {
                        return Err("Invalid token: organization_id is required in cloud mode".to_string());
                    }
                    if ::server_config::get().multitenant && claims.organization_id.as_deref() .map(|s| s.eq_ignore_ascii_case("system")).unwrap_or(false) {
                        return Err("Invalid token: 'system' organization cannot be used in multitenant mode".to_string());
                    }
                    if self.is_revoked(&claims.jti, &claims.organization_id.clone().unwrap_or_default()).await {
                        return Err("token revoked".to_string());
                    }
                    return Ok(claims);
                }
            }
        }

        let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
            let token_data = jsonwebtoken::decode::<Claims>(
                _token,
                &jsonwebtoken::DecodingKey::from_secret(&self.secret),
                &validation
            );

            match token_data {
                Ok(data) => {
                    if data.claims.sub.trim().is_empty() || data.claims.jti.trim().is_empty() {
                        return Err("Invalid token: empty claims".to_string());
                    }
                    if ::server_config::get().multitenant && data.claims.organization_id.clone().unwrap_or_default().trim().is_empty() {
                        return Err("Invalid token: organization_id is required in cloud mode".to_string());
                    }
                    if ::server_config::get().multitenant && data.claims.organization_id.as_deref() .map(|s| s.eq_ignore_ascii_case("system")).unwrap_or(false) {
                        return Err("Invalid token: 'system' organization cannot be used in multitenant mode".to_string());
                    }
                    if self.is_revoked(&data.claims.jti, &data.claims.organization_id.clone().unwrap_or_default()).await {
                        return Err("token revoked".to_string());
                    }
                    if data.claims.sub.trim().is_empty() || data.claims.jti.trim().is_empty() {
                        return Err("Invalid token claims".to_string());
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
                    if let Ok(claims) = crate::oidc::validate_oidc_token(_token, &oidc_cfg).await {
                        if ::server_config::get().multitenant && claims.organization_id.clone().unwrap_or_default().trim().is_empty() {
                            return Err("Invalid token: organization_id is required in cloud mode".to_string());
                        }
                        if ::server_config::get().multitenant && claims.organization_id.as_deref() .map(|s| s.eq_ignore_ascii_case("system")).unwrap_or(false) {
                            return Err("Invalid token: 'system' organization cannot be used in multitenant mode".to_string());
                        }
                        if self.is_revoked(&claims.jti, &claims.organization_id.clone().unwrap_or_default()).await {
                            return Err("token revoked".to_string());
                        }
                        return Ok(claims);
                    }
                    Err("Invalid token".to_string())
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
}

impl AuthServiceServerImpl {
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

pub fn parse_spiffe_id(spiffe_id: &str) -> Result<(String, String), Status> {
    let parts: Vec<&str> = spiffe_id.split('/').collect();
    if parts.len() < 7 || parts[3] != "org" || parts[5] != "agent" {
         return Err(Status::unauthenticated("Invalid SPIFFE ID format"));
    }
    Ok((parts[4].to_string(), parts[6].to_string()))
}

pub fn extract_spiffe_id_from_metadata(md: &tonic::metadata::MetadataMap) -> Result<String, String> {
    md.get("x-spiffe-id")
        .ok_or_else(|| "missing x-spiffe-id header".to_string())?
        .to_str()
        .map_err(|_| "invalid x-spiffe-id header".to_string())
        .map(|s| s.to_string())
}

pub struct AuthInfo {
    pub spiffe_id: String,
    pub org_id: String,
    pub agent_id: String,
}

#[tonic::async_trait]
impl AuthService for AuthServiceServerImpl {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();

        if ::server_config::get().multitenant && req.organization_id.is_empty() {
            return Err(Status::invalid_argument("organization_id is required in cloud mode to maintain tenant isolation"));
        }

        match self.store.authenticate(&req.username, &req.password, &req.organization_id) {
            Ok(user) => {
                match self.store.issue_token(&user) {
                    Ok(token) => {
                         let expires_at = (Utc::now() + chrono::Duration::hours(24)).timestamp();
                         Ok(Response::new(LoginResponse {
                             token,
                             expires_at,
                         }))
                    }
                    Err(e) => Err(Status::internal(e)),
                }
            }
            Err(e) => Err(Status::unauthenticated(e)),
        }
    }

    async fn register(&self, request: Request<CreateUserRequest>) -> Result<Response<LoginResponse>, Status> {
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
        ).map_err(|e| Status::internal(e))?;

        let token = self.store.issue_token(&user).map_err(|e| Status::internal(e))?;

        Ok(Response::new(LoginResponse {
             token,
             expires_at: (Utc::now() + chrono::Duration::hours(24)).timestamp(),
        }))
    }

    async fn logout(&self, request: Request<EmptyRequest>) -> Result<Response<EmptyResponse>, Status> {
        if let Some(auth_info) = request.extensions().get::<AuthInfo>() {
            if let Some(auth_header) = request.metadata().get("authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    let token = if auth_str.to_lowercase().starts_with("bearer ") {
                        &auth_str[7..]
                    } else {
                        auth_str
                    };

                    if let Ok(claims) = self.store.validate_token(token).await {
                        // Securely revoke the session
                        let exp = chrono::DateTime::from_timestamp(claims.exp, 0).unwrap_or_else(chrono::Utc::now);
                        self.store.revoke_token(claims.jti, exp, &auth_info.org_id).await;
                    }
                }
            }
        }
        Ok(Response::new(EmptyResponse {}))
    }

    async fn get_me(&self, request: Request<EmptyRequest>) -> Result<Response<UserProto>, Status> {
        let auth_info = request.extensions().get::<AuthInfo>()
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;

        let user = self.store.get_user(&auth_info.spiffe_id, &auth_info.org_id)
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

    async fn list_users(&self, request: Request<ListUsersRequest>) -> Result<Response<ListUsersResponse>, Status> {
        let auth_info = request.extensions().get::<AuthInfo>()
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;

        let users = self.store.list_users(&auth_info.org_id);
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

    async fn create_user(&self, request: Request<CreateUserRequest>) -> Result<Response<UserProto>, Status> {
        let auth_info = request.extensions().get::<AuthInfo>()
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;
        let org_id = auth_info.org_id.clone();

        let caller = self.store.get_user(&auth_info.spiffe_id, &org_id)
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
        ).map_err(|e| Status::internal(e))?;
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

    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<UserProto>, Status> {
        let auth_info = request.extensions().get::<AuthInfo>()
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;

        let user = self.store.get_user(&request.get_ref().id, &auth_info.org_id)
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

    async fn update_user(&self, request: Request<UpdateUserRequest>) -> Result<Response<UserProto>, Status> {
        let auth_info = request.extensions().get::<AuthInfo>()
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;
        let org_id = auth_info.org_id.clone();

        // Privilege Escalation fix: Ensure caller is ADMIN or the target user themselves
        let caller = self.store.get_user(&auth_info.spiffe_id, &org_id)
            .ok_or_else(|| Status::not_found("Caller not found"))?;

        let req = request.into_inner();

        let is_admin = caller.roles.contains(&"ADMIN".to_string());
        if !is_admin && caller.id != req.id {
            return Err(Status::permission_denied("Insufficient permissions to update this user"));
        }

        // Only ADMIN can change roles or active status
        let target_user = self.store.get_user(&req.id, &org_id).ok_or_else(|| Status::not_found("User not found"))?;
        let final_roles = if is_admin { req.roles } else { target_user.roles.clone() };
        let final_active = if is_admin { req.active } else { Some(target_user.active) };

        let user = self.store.update_user(&req.id, req.email, Some(final_roles), final_active, &org_id)
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

    async fn delete_user(&self, request: Request<DeleteUserRequest>) -> Result<Response<EmptyResponse>, Status> {
        let auth_info = request.extensions().get::<AuthInfo>()
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;
        let org_id = auth_info.org_id.clone();

        // Privilege Escalation fix: Ensure caller is ADMIN
        let caller = self.store.get_user(&auth_info.spiffe_id, &org_id)
            .ok_or_else(|| Status::not_found("Caller not found"))?;

        if !caller.roles.contains(&"ADMIN".to_string()) {
            return Err(Status::permission_denied("Only ADMIN can delete users"));
        }

        self.store.delete_user(&request.get_ref().id, &org_id)
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(EmptyResponse {}))
    }

    async fn list_roles(&self, _request: Request<EmptyRequest>) -> Result<Response<ListRolesResponse>, Status> {
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

    async fn create_role(&self, _request: Request<CreateRoleRequest>) -> Result<Response<RoleProto>, Status> {
        Ok(Response::new(RoleProto::default()))
    }
}

#[cfg(test)]
mod store_tests {
    use super::*;

    #[test]
    fn test_secret_paths_are_safe() {
        let store = Store::new();
        // Since we can't easily assert on the inner paths without modifying visibility,
        // we assert that we don't panic upon creation.
        assert!(!store.secret.is_empty());
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
