pub use ::server_common as common;
pub use ::server_ohc as ohc;
pub use ::server_oidc as oidc;

pub mod orchestration;

use std::collections::HashMap;
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
    /// Pre-shared HMAC-SHA256 token.
    Token { token_hash: Vec<u8> },
    /// SPIFFE/mTLS peer certificate.
    Spiffe { allowed_id: String },
}

/// Build an AuthMode from environment variables.
///
///   OHC_AGENT_AUTH_DISABLED=true   – skip auth (dev only)
///   OHC_AGENT_TOKEN                – enables token mode
///   OHC_AGENT_SPIFFE_ID            – restricts SPIFFE ID (enables SPIFFE mode)
pub fn auth_mode_from_env() -> AuthMode {
    if let Ok(tok) = env::var("OHC_AGENT_TOKEN") {
        if !tok.is_empty() {
            let hash = hmac_token(&tok);
            return AuthMode::Token { token_hash: hash };
        }
    }
    AuthMode::Spiffe {
        allowed_id: env::var("OHC_AGENT_SPIFFE_ID").unwrap_or_default(),
    }
}

/// Compute HMAC-SHA256 of the token using the application key.
fn hmac_token(token: &str) -> Vec<u8> {
    let key = std::env::var("OHC_AGENT_AUTH_KEY")
        .unwrap_or_else(|_| "default_auth_key_change_me".to_string());
    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(token.as_bytes());
    mac.finalize().into_bytes().to_vec()
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
    #[allow(dead_code)]
    roles: RwLock<HashMap<String, Role>>,
    by_name: RwLock<HashMap<TenantKey, String>>,
    by_email: RwLock<HashMap<TenantKey, String>>,
    by_oidc: RwLock<HashMap<TenantKey, String>>,
    revoked: RwLock<HashMap<String, DateTime<Utc>>>,
    #[allow(dead_code)]
    pub secret: Vec<u8>,
    #[allow(dead_code)]
    oidc_cfg: RwLock<OIDCConfig>,
}

impl Store {
    pub fn new() -> Self {
        let secret = std::env::var("JWT_SECRET")
            .map(|s| s.into_bytes())
            .unwrap_or_else(|_| {
                if ::server_config::get().multitenant {
                    panic!("JWT_SECRET must be set in Cloud/Multitenant Mode to ensure secure access token management.");
                }

                let secret_path = std::path::Path::new(".ohc_jwt_secret");
                if secret_path.exists() {
                    if let Ok(bytes) = std::fs::read(secret_path) {
                        if bytes.len() >= 32 {
                            return bytes;
                        }
                    }
                }

                let new_secret = if let Ok(sqlite_key) = std::env::var("OHC_SQLITE_KEY") {
                    tracing::debug!("falling back to generated JWT secret; deriving from OHC_SQLITE_KEY for determinism; writing to .ohc_jwt_secret for persistence");
                    let mut mac = HmacSha256::new_from_slice(b"ohc_jwt_derivation_salt").expect("HMAC can take key of any size");
                    mac.update(sqlite_key.as_bytes());
                    mac.finalize().into_bytes().to_vec()
                } else {
                    tracing::debug!("falling back to generated JWT secret; writing to .ohc_jwt_secret for persistence");
                    panic!("OHC_SQLITE_KEY must be set in Standalone Mode to ensure secure, encrypted SQLite storage.")
                };

                #[cfg(unix)]
                {
                    use std::os::unix::fs::OpenOptionsExt;
                    use std::io::Write;
                    if let Ok(mut file) = std::fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .mode(0o600)
                        .open(secret_path)
                    {
                        let _ = file.write_all(&new_secret);
                    }
                }
                #[cfg(not(unix))]
                {
                    let _ = std::fs::write(secret_path, &new_secret);
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

        let store = Store {
            users: RwLock::new(HashMap::new()),
            roles: RwLock::new(roles),
            by_name: RwLock::new(HashMap::new()),
            by_email: RwLock::new(HashMap::new()),
            by_oidc: RwLock::new(HashMap::new()),
            revoked: RwLock::new(HashMap::new()),
            secret,
            oidc_cfg: RwLock::new(OIDCConfig {
                issuer_url,
                client_id,
                enabled,
            }),
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

        self.users.write().unwrap().insert(id.clone(), admin);
        self.by_name.write().unwrap().insert(TenantKey { org_id: "".to_string(), key: admin_user }, id.clone());
        self.by_email.write().unwrap().insert(TenantKey { org_id: "".to_string(), key: admin_email }, id);
    }

    pub fn create_user(&self, username: String, email: String, password: String, roles: Vec<String>, org_id: String) -> Result<User, String> {
        if username.is_empty() {
            return Err("username is required".to_string());
        }
        if password.len() < 6 {
            return Err("password must be at least 6 characters".to_string());
        }

        let mut users = self.users.write().unwrap();
        let mut by_name = self.by_name.write().unwrap();
        let mut by_email = self.by_email.write().unwrap();

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
        let by_name = self.by_name.read().unwrap();
        let users = self.users.read().unwrap();

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
            if !org_id.is_empty() && user_org != org_id {
                return Err("invalid credentials".to_string());
            }
        }

        if verify(password, &user.password_hash).unwrap_or(false) {
            Ok(user.clone())
        } else {
            Err("invalid credentials".to_string())
        }
    }

    pub fn get_user(&self, id: &str, org_id: &str) -> Option<User> {
        let users = self.users.read().unwrap();
        let u = users.get(id)?;

        if !org_id.is_empty() {
            if let Some(ref user_org) = u.organization_id {
                if user_org != org_id {
                    return None;
                }
            } else {
                return None;
            }
        }
        Some(u.clone())
    }

    pub fn list_users(&self, org_id: &str) -> Vec<User> {
        let users = self.users.read().unwrap();
        users.values()
            .filter(|u| {
                org_id.is_empty() || u.organization_id.as_deref() == Some(org_id)
            })
            .cloned()
            .collect()
    }

    pub fn update_user(&self, id: &str, email_ptr: Option<String>, roles: Option<Vec<String>>, active_ptr: Option<bool>, org_id: &str) -> Result<User, String> {
        let mut users = self.users.write().unwrap();
        let mut by_email = self.by_email.write().unwrap();

        let u = users.get_mut(id).ok_or_else(|| "user not found".to_string())?;

        if !org_id.is_empty() {
             if u.organization_id.as_deref() != Some(org_id) {
                 return Err("user not found".to_string());
             }
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
        let mut users = self.users.write().unwrap();
        let mut by_name = self.by_name.write().unwrap();
        let mut by_email = self.by_email.write().unwrap();
        let mut by_oidc = self.by_oidc.write().unwrap();

        let u = users.get(id).ok_or_else(|| "user not found".to_string())?;

        if !org_id.is_empty() {
             if u.organization_id.as_deref() != Some(org_id) {
                 return Err("user not found".to_string());
             }
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

    pub async fn revoke_token(&self, jti: String, exp: DateTime<Utc>, _org_id: &str) {
        let mut revoked = self.revoked.write().unwrap();
        revoked.insert(jti, exp);

        let now = Utc::now();
        revoked.retain(|_, v| *v > now);
    }

    pub fn is_revoked(&self, jti: &str, _org_id: &str) -> bool {
        let revoked = self.revoked.read().unwrap();
        if let Some(exp) = revoked.get(jti) {
             if exp > &Utc::now() {
                 return true;
             }
        }
        false
    }

    pub fn issue_token(&self, _user: &User) -> Result<String, String> {
        if ::server_config::get().multitenant && _user.organization_id.as_deref().unwrap_or_default().trim().is_empty() {
            return Err("organization_id is required for token issuance in cloud mode".to_string());
        }
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
            jti: hex::encode(random_bytes(8)),
        };

        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
        let token = jsonwebtoken::encode(&header, &claims, &jsonwebtoken::EncodingKey::from_secret(&self.secret))
            .map_err(|e| e.to_string())?;

        Ok(token)
    }

    pub async fn validate_token(&self, _token: &str) -> Result<Claims, String> {
        if let Ok(header) = jsonwebtoken::decode_header(_token) {
            if header.alg == jsonwebtoken::Algorithm::RS256 {
                let oidc_cfg_internal = self.oidc_cfg.read().unwrap().clone();
                let oidc_cfg = crate::oidc::OIDCConfig {
                    issuer_url: oidc_cfg_internal.issuer_url,
                    client_id: oidc_cfg_internal.client_id,
                    enabled: oidc_cfg_internal.enabled,
                };
                if oidc_cfg.enabled {
                    let claims = crate::oidc::validate_oidc_token(_token, &oidc_cfg).await?;
                    if self.is_revoked(&claims.jti, &claims.organization_id.clone().unwrap_or_default()) {
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
                    if self.is_revoked(&data.claims.jti, &data.claims.organization_id.clone().unwrap_or_default()) {
                        return Err("token revoked".to_string());
                    }
                    if data.claims.sub.trim().is_empty() || data.claims.jti.trim().is_empty() {
                        return Err("Invalid token claims".to_string());
                    }
                    Ok(data.claims)
                }
                Err(_) => {
                    let oidc_cfg = {
                        let c = self.oidc_cfg.read().unwrap();
                        crate::oidc::OIDCConfig {
                            issuer_url: c.issuer_url.clone(),
                            client_id: c.client_id.clone(),
                            enabled: c.enabled,
                        }
                    };
                    if let Ok(claims) = crate::oidc::validate_oidc_token(_token, &oidc_cfg).await {
                        if self.is_revoked(&claims.jti, &claims.organization_id.clone().unwrap_or_default()) {
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
    if parts.len() < 7 || parts[2] != "ohc" || parts[3] != "org" || parts[5] != "agent" {
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

        if ::server_config::get().multitenant {
            if req.organization_id.is_empty() {
                return Err(Status::invalid_argument("organization_id is required in cloud mode to maintain tenant isolation"));
            }
            if req.organization_id == "system" {
                return Err(Status::invalid_argument("organization_id cannot be 'system' in cloud mode"));
            }
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
        if ::server_config::get().multitenant {
            if req.organization_id.is_empty() {
                 return Err(Status::invalid_argument("organization_id is required in cloud mode to maintain tenant isolation"));
            }
            if req.organization_id == "system" {
                return Err(Status::invalid_argument("organization_id cannot be 'system' in cloud mode"));
            }
        }

        let user = self.store.create_user(
            req.email.clone(),
            req.email.clone(),
            req.password,
            vec![ROLE_VIEWER.to_string()],
            req.organization_id.clone(),
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
        let req = request.into_inner();
        let user = self.store.create_user(
            req.email.clone(),
            req.email.clone(),
            "temp".to_string(),
            vec![],
            req.organization_id.clone(),
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
        let org_id = request.extensions().get::<AuthInfo>()
            .map(|ai| ai.org_id.clone())
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;
        let req = request.into_inner();

        let user = self.store.update_user(&req.id, req.email, Some(req.roles), req.active, &org_id)
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
        let org_id = request.extensions().get::<AuthInfo>()
            .map(|ai| ai.org_id.clone())
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;

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

#[cfg(test)]
mod sentinel_auth_hardening_tests {
    use super::*;

    #[test]
    fn test_auth_store_secret_derivation() {
        // Verify that in standalone mode without a secret file, we derive it correctly from SQLITE_KEY
        temp_env::with_vars(vec![("OHC_SQLITE_KEY", Some("test-key-123")), ("JWT_SECRET", None), ("OHC_MULTITENANT", Some("false"))], || {
             let store = Store::new();
             assert!(!store.secret.is_empty());
             assert_eq!(store.secret.len(), 32);
        });
    }

    #[test]
    fn test_issue_token_requires_org_id_in_cloud() {
        temp_env::with_vars(vec![("OHC_MULTITENANT", Some("true")), ("JWT_SECRET", Some("secret-32-chars-long-at-least-!!!"))], || {
            let store = Store::new();
            let mut user = User {
                id: "u1".to_string(),
                username: "test".to_string(),
                email: "test@example.com".to_string(),
                password_hash: "hash".to_string(),
                roles: vec![],
                active: true,
                organization_id: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                oidc_subject: None,
            };

            // Should fail if org_id is missing
            assert!(store.issue_token(&user).is_err());

            user.organization_id = Some("org1".to_string());
            assert!(store.issue_token(&user).is_ok());
        });
    }
}

pub mod postgres_store;
