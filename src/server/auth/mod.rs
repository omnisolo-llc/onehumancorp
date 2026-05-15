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
    let mut mac =
        HmacSha256::new_from_slice(key.as_bytes()).expect("HMAC can take key of any size");
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

use ::server_common::auth_utils::set_org_context;
use ::server_common::Claims;
use ::server_ohc::orchestration::auth_service_server::AuthService;
use ::server_ohc::orchestration::*;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use tonic::{Request, Response, Status};

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
    roles: RwLock<HashMap<String, Role>>,
    by_name: RwLock<HashMap<TenantKey, String>>,
    by_email: RwLock<HashMap<TenantKey, String>>,
    by_oidc: RwLock<HashMap<TenantKey, String>>,
    revoked: RwLock<HashMap<String, DateTime<Utc>>>,
    #[allow(dead_code)]
    secret: Vec<u8>,
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
                    tracing::warn!("falling back to generated JWT secret; deriving from OHC_SQLITE_KEY for determinism; writing to .ohc_jwt_secret for persistence");
                    let mut mac = HmacSha256::new_from_slice(b"ohc_jwt_derivation_salt").expect("HMAC can take key of any size");
                    mac.update(sqlite_key.as_bytes());
                    mac.finalize().into_bytes().to_vec()
                } else {
                    tracing::warn!("falling back to generated JWT secret; writing to .ohc_jwt_secret for persistence");
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

        roles.insert(
            ROLE_ADMIN.to_string(),
            Role {
                id: ROLE_ADMIN.to_string(),
                name: ROLE_ADMIN.to_string(),
                permissions: vec!["*".to_string()],
                created_at: now,
            },
        );
        roles.insert(
            ROLE_OPERATOR.to_string(),
            Role {
                id: ROLE_OPERATOR.to_string(),
                name: ROLE_OPERATOR.to_string(),
                permissions: vec!["read".to_string(), "write".to_string()],
                created_at: now,
            },
        );
        roles.insert(
            ROLE_VIEWER.to_string(),
            Role {
                id: ROLE_VIEWER.to_string(),
                name: ROLE_VIEWER.to_string(),
                permissions: vec!["read".to_string()],
                created_at: now,
            },
        );

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
        let admin_email =
            std::env::var("ADMIN_EMAIL").unwrap_or_else(|_| "admin@localhost".to_string());

        let hash = hash(admin_pass, if cfg!(test) { 4 } else { DEFAULT_COST })
            .expect("Failed to hash password");

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
        self.by_name.write().unwrap().insert(
            TenantKey {
                org_id: "".to_string(),
                key: admin_user,
            },
            id.clone(),
        );
        self.by_email.write().unwrap().insert(
            TenantKey {
                org_id: "".to_string(),
                key: admin_email,
            },
            id,
        );
    }

    pub fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
        roles: Vec<String>,
        org_id: String,
    ) -> Result<User, String> {
        if username.is_empty() {
            return Err("username is required".to_string());
        }
        if password.len() < 6 {
            return Err("password must be at least 6 characters".to_string());
        }

        let mut users = self.users.write().unwrap();
        let mut by_name = self.by_name.write().unwrap();
        let mut by_email = self.by_email.write().unwrap();

        let name_key = TenantKey {
            org_id: org_id.clone(),
            key: username.clone(),
        };
        if by_name.contains_key(&name_key) {
            return Err("username already taken".to_string());
        }

        let email_key = TenantKey {
            org_id: org_id.clone(),
            key: email.clone(),
        };
        if by_email.contains_key(&email_key) {
            return Err("email already registered".to_string());
        }

        let hash = hash(password, if cfg!(test) { 4 } else { DEFAULT_COST })
            .expect("Failed to hash password");

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

    pub fn authenticate(
        &self,
        username: &str,
        password: &str,
        org_id: &str,
    ) -> Result<User, String> {
        let by_name = self.by_name.read().unwrap();
        let users = self.users.read().unwrap();

        let name_key = TenantKey {
            org_id: org_id.to_string(),
            key: username.to_string(),
        };
        let mut user_id_opt = by_name.get(&name_key).cloned();

        if user_id_opt.is_none() && org_id.is_empty() {
            user_id_opt = by_name
                .get(&TenantKey {
                    org_id: "".to_string(),
                    key: username.to_string(),
                })
                .cloned();
        }

        let user_id = user_id_opt.ok_or_else(|| "invalid credentials".to_string())?;
        let user = users
            .get(&user_id)
            .ok_or_else(|| "invalid credentials".to_string())?;

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
        users
            .values()
            .filter(|u| org_id.is_empty() || u.organization_id.as_deref() == Some(org_id))
            .cloned()
            .collect()
    }

    pub fn update_user(
        &self,
        id: &str,
        email_ptr: Option<String>,
        roles: Option<Vec<String>>,
        active_ptr: Option<bool>,
        org_id: &str,
    ) -> Result<User, String> {
        let mut users = self.users.write().unwrap();
        let mut by_email = self.by_email.write().unwrap();

        let u = users
            .get_mut(id)
            .ok_or_else(|| "user not found".to_string())?;

        if !org_id.is_empty() {
            if u.organization_id.as_deref() != Some(org_id) {
                return Err("user not found".to_string());
            }
        }

        if let Some(email) = email_ptr {
            if email != u.email {
                let org = u.organization_id.clone().unwrap_or_default();
                let email_key = TenantKey {
                    org_id: org,
                    key: email.clone(),
                };
                if by_email.contains_key(&email_key) {
                    return Err("email already registered".to_string());
                }
                by_email.remove(&TenantKey {
                    org_id: u.organization_id.clone().unwrap_or_default(),
                    key: u.email.clone(),
                });
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
        by_name.remove(&TenantKey {
            org_id: org.clone(),
            key: u.username.clone(),
        });
        by_email.remove(&TenantKey {
            org_id: org.clone(),
            key: u.email.clone(),
        });
        if let Some(ref oidc) = u.oidc_subject {
            by_oidc.remove(&TenantKey {
                org_id: org,
                key: oidc.clone(),
            });
        }

        users.remove(id);

        Ok(())
    }

    pub fn revoke_token(&self, jti: String, exp: DateTime<Utc>, _org_id: &str) {
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
        let token = jsonwebtoken::encode(
            &header,
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(&self.secret),
        )
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
                    return crate::oidc::validate_oidc_token(_token, &oidc_cfg).await;
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
                    return Err("Invalid token: empty claims".to_string());
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
                    return Err(
                        "Invalid token: organization_id is required in cloud mode".to_string()
                    );
                }
                if self.is_revoked(
                    &data.claims.jti,
                    &data.claims.organization_id.clone().unwrap_or_default(),
                ) {
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

pub fn extract_spiffe_id_from_metadata(
    md: &tonic::metadata::MetadataMap,
) -> Result<String, String> {
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
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();

        if ::server_config::get().multitenant && req.organization_id.is_empty() {
            return Err(Status::invalid_argument(
                "organization_id is required in cloud mode to maintain tenant isolation",
            ));
        }

        match self
            .store
            .authenticate(&req.username, &req.password, &req.organization_id)
        {
            Ok(user) => match self.store.issue_token(&user) {
                Ok(token) => {
                    let expires_at = (Utc::now() + chrono::Duration::hours(24)).timestamp();
                    Ok(Response::new(LoginResponse { token, expires_at }))
                }
                Err(e) => Err(Status::internal(e)),
            },
            Err(e) => Err(Status::unauthenticated(e)),
        }
    }

    async fn register(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        if ::server_config::get().multitenant && req.organization_id.is_empty() {
            return Err(Status::invalid_argument(
                "organization_id is required in cloud mode to maintain tenant isolation",
            ));
        }

        let user = self
            .store
            .create_user(
                req.email.clone(),
                req.email.clone(),
                req.password,
                vec![ROLE_VIEWER.to_string()],
                req.organization_id.clone(),
            )
            .map_err(|e| Status::internal(e))?;

        let token = self
            .store
            .issue_token(&user)
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(LoginResponse {
            token,
            expires_at: (Utc::now() + chrono::Duration::hours(24)).timestamp(),
        }))
    }

    async fn logout(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        Ok(Response::new(EmptyResponse {}))
    }

    async fn get_me(&self, request: Request<EmptyRequest>) -> Result<Response<UserProto>, Status> {
        let auth_info = request
            .extensions()
            .get::<AuthInfo>()
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;

        let user = self
            .store
            .get_user(&auth_info.spiffe_id, &auth_info.org_id)
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
        request: Request<ListUsersRequest>,
    ) -> Result<Response<ListUsersResponse>, Status> {
        let auth_info = request
            .extensions()
            .get::<AuthInfo>()
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;

        let users = self.store.list_users(&auth_info.org_id);
        let proto_users = users
            .into_iter()
            .map(|u| UserProto {
                id: u.id,
                username: u.username,
                email: u.email,
                roles: u.roles,
                active: u.active,
                organization_id: u.organization_id.unwrap_or_default(),
                created_at_unix: u.created_at.timestamp(),
                updated_at_unix: u.updated_at.timestamp(),
                oidc_subject: u.oidc_subject.unwrap_or_default(),
            })
            .collect();
        Ok(Response::new(ListUsersResponse { users: proto_users }))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<UserProto>, Status> {
        let req = request.into_inner();
        let user = self
            .store
            .create_user(
                req.email.clone(),
                req.email.clone(),
                "temp".to_string(),
                vec![],
                req.organization_id.clone(),
            )
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

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserProto>, Status> {
        let auth_info = request
            .extensions()
            .get::<AuthInfo>()
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;

        let user = self
            .store
            .get_user(&request.get_ref().id, &auth_info.org_id)
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
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UserProto>, Status> {
        let org_id = request
            .extensions()
            .get::<AuthInfo>()
            .map(|ai| ai.org_id.clone())
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;
        let req = request.into_inner();

        let user = self
            .store
            .update_user(&req.id, req.email, Some(req.roles), req.active, &org_id)
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
        request: Request<DeleteUserRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let org_id = request
            .extensions()
            .get::<AuthInfo>()
            .map(|ai| ai.org_id.clone())
            .ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;

        self.store
            .delete_user(&request.get_ref().id, &org_id)
            .map_err(|e| Status::internal(e))?;

        Ok(Response::new(EmptyResponse {}))
    }

    async fn list_roles(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<ListRolesResponse>, Status> {
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
        request: Request<CreateRoleRequest>,
    ) -> Result<Response<RoleProto>, Status> {
        Ok(Response::new(RoleProto::default()))
    }
}
pub mod granular_permissions {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum PermissionNode {
        CreateUser,
        ReadUser,
        UpdateUser,
        DeleteUser,
        ListUser,
        ApproveUser,
        RejectUser,
        ArchiveUser,
        RestoreUser,
        TransferUser,
        CreateRole,
        ReadRole,
        UpdateRole,
        DeleteRole,
        ListRole,
        ApproveRole,
        RejectRole,
        ArchiveRole,
        RestoreRole,
        TransferRole,
        CreateTenant,
        ReadTenant,
        UpdateTenant,
        DeleteTenant,
        ListTenant,
        ApproveTenant,
        RejectTenant,
        ArchiveTenant,
        RestoreTenant,
        TransferTenant,
        CreateBillingProfile,
        ReadBillingProfile,
        UpdateBillingProfile,
        DeleteBillingProfile,
        ListBillingProfile,
        ApproveBillingProfile,
        RejectBillingProfile,
        ArchiveBillingProfile,
        RestoreBillingProfile,
        TransferBillingProfile,
        CreateAgent,
        ReadAgent,
        UpdateAgent,
        DeleteAgent,
        ListAgent,
        ApproveAgent,
        RejectAgent,
        ArchiveAgent,
        RestoreAgent,
        TransferAgent,
        CreateTask,
        ReadTask,
        UpdateTask,
        DeleteTask,
        ListTask,
        ApproveTask,
        RejectTask,
        ArchiveTask,
        RestoreTask,
        TransferTask,
        CreateMeeting,
        ReadMeeting,
        UpdateMeeting,
        DeleteMeeting,
        ListMeeting,
        ApproveMeeting,
        RejectMeeting,
        ArchiveMeeting,
        RestoreMeeting,
        TransferMeeting,
        CreateIntegration,
        ReadIntegration,
        UpdateIntegration,
        DeleteIntegration,
        ListIntegration,
        ApproveIntegration,
        RejectIntegration,
        ArchiveIntegration,
        RestoreIntegration,
        TransferIntegration,
        CreateApiKey,
        ReadApiKey,
        UpdateApiKey,
        DeleteApiKey,
        ListApiKey,
        ApproveApiKey,
        RejectApiKey,
        ArchiveApiKey,
        RestoreApiKey,
        TransferApiKey,
        CreateWebhook,
        ReadWebhook,
        UpdateWebhook,
        DeleteWebhook,
        ListWebhook,
        ApproveWebhook,
        RejectWebhook,
        ArchiveWebhook,
        RestoreWebhook,
        TransferWebhook,
        CreateInvoice,
        ReadInvoice,
        UpdateInvoice,
        DeleteInvoice,
        ListInvoice,
        ApproveInvoice,
        RejectInvoice,
        ArchiveInvoice,
        RestoreInvoice,
        TransferInvoice,
        CreateSubscription,
        ReadSubscription,
        UpdateSubscription,
        DeleteSubscription,
        ListSubscription,
        ApproveSubscription,
        RejectSubscription,
        ArchiveSubscription,
        RestoreSubscription,
        TransferSubscription,
        CreateAuditLog,
        ReadAuditLog,
        UpdateAuditLog,
        DeleteAuditLog,
        ListAuditLog,
        ApproveAuditLog,
        RejectAuditLog,
        ArchiveAuditLog,
        RestoreAuditLog,
        TransferAuditLog,
        CreateSecurityPolicy,
        ReadSecurityPolicy,
        UpdateSecurityPolicy,
        DeleteSecurityPolicy,
        ListSecurityPolicy,
        ApproveSecurityPolicy,
        RejectSecurityPolicy,
        ArchiveSecurityPolicy,
        RestoreSecurityPolicy,
        TransferSecurityPolicy,
        CreateNetworkRoute,
        ReadNetworkRoute,
        UpdateNetworkRoute,
        DeleteNetworkRoute,
        ListNetworkRoute,
        ApproveNetworkRoute,
        RejectNetworkRoute,
        ArchiveNetworkRoute,
        RestoreNetworkRoute,
        TransferNetworkRoute,
        Unknown(String),
    }

    impl std::str::FromStr for PermissionNode {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "createuser" => Ok(PermissionNode::CreateUser),
                "readuser" => Ok(PermissionNode::ReadUser),
                "updateuser" => Ok(PermissionNode::UpdateUser),
                "deleteuser" => Ok(PermissionNode::DeleteUser),
                "listuser" => Ok(PermissionNode::ListUser),
                "approveuser" => Ok(PermissionNode::ApproveUser),
                "rejectuser" => Ok(PermissionNode::RejectUser),
                "archiveuser" => Ok(PermissionNode::ArchiveUser),
                "restoreuser" => Ok(PermissionNode::RestoreUser),
                "transferuser" => Ok(PermissionNode::TransferUser),
                "createrole" => Ok(PermissionNode::CreateRole),
                "readrole" => Ok(PermissionNode::ReadRole),
                "updaterole" => Ok(PermissionNode::UpdateRole),
                "deleterole" => Ok(PermissionNode::DeleteRole),
                "listrole" => Ok(PermissionNode::ListRole),
                "approverole" => Ok(PermissionNode::ApproveRole),
                "rejectrole" => Ok(PermissionNode::RejectRole),
                "archiverole" => Ok(PermissionNode::ArchiveRole),
                "restorerole" => Ok(PermissionNode::RestoreRole),
                "transferrole" => Ok(PermissionNode::TransferRole),
                "createtenant" => Ok(PermissionNode::CreateTenant),
                "readtenant" => Ok(PermissionNode::ReadTenant),
                "updatetenant" => Ok(PermissionNode::UpdateTenant),
                "deletetenant" => Ok(PermissionNode::DeleteTenant),
                "listtenant" => Ok(PermissionNode::ListTenant),
                "approvetenant" => Ok(PermissionNode::ApproveTenant),
                "rejecttenant" => Ok(PermissionNode::RejectTenant),
                "archivetenant" => Ok(PermissionNode::ArchiveTenant),
                "restoretenant" => Ok(PermissionNode::RestoreTenant),
                "transfertenant" => Ok(PermissionNode::TransferTenant),
                "createbillingprofile" => Ok(PermissionNode::CreateBillingProfile),
                "readbillingprofile" => Ok(PermissionNode::ReadBillingProfile),
                "updatebillingprofile" => Ok(PermissionNode::UpdateBillingProfile),
                "deletebillingprofile" => Ok(PermissionNode::DeleteBillingProfile),
                "listbillingprofile" => Ok(PermissionNode::ListBillingProfile),
                "approvebillingprofile" => Ok(PermissionNode::ApproveBillingProfile),
                "rejectbillingprofile" => Ok(PermissionNode::RejectBillingProfile),
                "archivebillingprofile" => Ok(PermissionNode::ArchiveBillingProfile),
                "restorebillingprofile" => Ok(PermissionNode::RestoreBillingProfile),
                "transferbillingprofile" => Ok(PermissionNode::TransferBillingProfile),
                "createagent" => Ok(PermissionNode::CreateAgent),
                "readagent" => Ok(PermissionNode::ReadAgent),
                "updateagent" => Ok(PermissionNode::UpdateAgent),
                "deleteagent" => Ok(PermissionNode::DeleteAgent),
                "listagent" => Ok(PermissionNode::ListAgent),
                "approveagent" => Ok(PermissionNode::ApproveAgent),
                "rejectagent" => Ok(PermissionNode::RejectAgent),
                "archiveagent" => Ok(PermissionNode::ArchiveAgent),
                "restoreagent" => Ok(PermissionNode::RestoreAgent),
                "transferagent" => Ok(PermissionNode::TransferAgent),
                "createtask" => Ok(PermissionNode::CreateTask),
                "readtask" => Ok(PermissionNode::ReadTask),
                "updatetask" => Ok(PermissionNode::UpdateTask),
                "deletetask" => Ok(PermissionNode::DeleteTask),
                "listtask" => Ok(PermissionNode::ListTask),
                "approvetask" => Ok(PermissionNode::ApproveTask),
                "rejecttask" => Ok(PermissionNode::RejectTask),
                "archivetask" => Ok(PermissionNode::ArchiveTask),
                "restoretask" => Ok(PermissionNode::RestoreTask),
                "transfertask" => Ok(PermissionNode::TransferTask),
                "createmeeting" => Ok(PermissionNode::CreateMeeting),
                "readmeeting" => Ok(PermissionNode::ReadMeeting),
                "updatemeeting" => Ok(PermissionNode::UpdateMeeting),
                "deletemeeting" => Ok(PermissionNode::DeleteMeeting),
                "listmeeting" => Ok(PermissionNode::ListMeeting),
                "approvemeeting" => Ok(PermissionNode::ApproveMeeting),
                "rejectmeeting" => Ok(PermissionNode::RejectMeeting),
                "archivemeeting" => Ok(PermissionNode::ArchiveMeeting),
                "restoremeeting" => Ok(PermissionNode::RestoreMeeting),
                "transfermeeting" => Ok(PermissionNode::TransferMeeting),
                "createintegration" => Ok(PermissionNode::CreateIntegration),
                "readintegration" => Ok(PermissionNode::ReadIntegration),
                "updateintegration" => Ok(PermissionNode::UpdateIntegration),
                "deleteintegration" => Ok(PermissionNode::DeleteIntegration),
                "listintegration" => Ok(PermissionNode::ListIntegration),
                "approveintegration" => Ok(PermissionNode::ApproveIntegration),
                "rejectintegration" => Ok(PermissionNode::RejectIntegration),
                "archiveintegration" => Ok(PermissionNode::ArchiveIntegration),
                "restoreintegration" => Ok(PermissionNode::RestoreIntegration),
                "transferintegration" => Ok(PermissionNode::TransferIntegration),
                "createapikey" => Ok(PermissionNode::CreateApiKey),
                "readapikey" => Ok(PermissionNode::ReadApiKey),
                "updateapikey" => Ok(PermissionNode::UpdateApiKey),
                "deleteapikey" => Ok(PermissionNode::DeleteApiKey),
                "listapikey" => Ok(PermissionNode::ListApiKey),
                "approveapikey" => Ok(PermissionNode::ApproveApiKey),
                "rejectapikey" => Ok(PermissionNode::RejectApiKey),
                "archiveapikey" => Ok(PermissionNode::ArchiveApiKey),
                "restoreapikey" => Ok(PermissionNode::RestoreApiKey),
                "transferapikey" => Ok(PermissionNode::TransferApiKey),
                "createwebhook" => Ok(PermissionNode::CreateWebhook),
                "readwebhook" => Ok(PermissionNode::ReadWebhook),
                "updatewebhook" => Ok(PermissionNode::UpdateWebhook),
                "deletewebhook" => Ok(PermissionNode::DeleteWebhook),
                "listwebhook" => Ok(PermissionNode::ListWebhook),
                "approvewebhook" => Ok(PermissionNode::ApproveWebhook),
                "rejectwebhook" => Ok(PermissionNode::RejectWebhook),
                "archivewebhook" => Ok(PermissionNode::ArchiveWebhook),
                "restorewebhook" => Ok(PermissionNode::RestoreWebhook),
                "transferwebhook" => Ok(PermissionNode::TransferWebhook),
                "createinvoice" => Ok(PermissionNode::CreateInvoice),
                "readinvoice" => Ok(PermissionNode::ReadInvoice),
                "updateinvoice" => Ok(PermissionNode::UpdateInvoice),
                "deleteinvoice" => Ok(PermissionNode::DeleteInvoice),
                "listinvoice" => Ok(PermissionNode::ListInvoice),
                "approveinvoice" => Ok(PermissionNode::ApproveInvoice),
                "rejectinvoice" => Ok(PermissionNode::RejectInvoice),
                "archiveinvoice" => Ok(PermissionNode::ArchiveInvoice),
                "restoreinvoice" => Ok(PermissionNode::RestoreInvoice),
                "transferinvoice" => Ok(PermissionNode::TransferInvoice),
                "createsubscription" => Ok(PermissionNode::CreateSubscription),
                "readsubscription" => Ok(PermissionNode::ReadSubscription),
                "updatesubscription" => Ok(PermissionNode::UpdateSubscription),
                "deletesubscription" => Ok(PermissionNode::DeleteSubscription),
                "listsubscription" => Ok(PermissionNode::ListSubscription),
                "approvesubscription" => Ok(PermissionNode::ApproveSubscription),
                "rejectsubscription" => Ok(PermissionNode::RejectSubscription),
                "archivesubscription" => Ok(PermissionNode::ArchiveSubscription),
                "restoresubscription" => Ok(PermissionNode::RestoreSubscription),
                "transfersubscription" => Ok(PermissionNode::TransferSubscription),
                "createauditlog" => Ok(PermissionNode::CreateAuditLog),
                "readauditlog" => Ok(PermissionNode::ReadAuditLog),
                "updateauditlog" => Ok(PermissionNode::UpdateAuditLog),
                "deleteauditlog" => Ok(PermissionNode::DeleteAuditLog),
                "listauditlog" => Ok(PermissionNode::ListAuditLog),
                "approveauditlog" => Ok(PermissionNode::ApproveAuditLog),
                "rejectauditlog" => Ok(PermissionNode::RejectAuditLog),
                "archiveauditlog" => Ok(PermissionNode::ArchiveAuditLog),
                "restoreauditlog" => Ok(PermissionNode::RestoreAuditLog),
                "transferauditlog" => Ok(PermissionNode::TransferAuditLog),
                "createsecuritypolicy" => Ok(PermissionNode::CreateSecurityPolicy),
                "readsecuritypolicy" => Ok(PermissionNode::ReadSecurityPolicy),
                "updatesecuritypolicy" => Ok(PermissionNode::UpdateSecurityPolicy),
                "deletesecuritypolicy" => Ok(PermissionNode::DeleteSecurityPolicy),
                "listsecuritypolicy" => Ok(PermissionNode::ListSecurityPolicy),
                "approvesecuritypolicy" => Ok(PermissionNode::ApproveSecurityPolicy),
                "rejectsecuritypolicy" => Ok(PermissionNode::RejectSecurityPolicy),
                "archivesecuritypolicy" => Ok(PermissionNode::ArchiveSecurityPolicy),
                "restoresecuritypolicy" => Ok(PermissionNode::RestoreSecurityPolicy),
                "transfersecuritypolicy" => Ok(PermissionNode::TransferSecurityPolicy),
                "createnetworkroute" => Ok(PermissionNode::CreateNetworkRoute),
                "readnetworkroute" => Ok(PermissionNode::ReadNetworkRoute),
                "updatenetworkroute" => Ok(PermissionNode::UpdateNetworkRoute),
                "deletenetworkroute" => Ok(PermissionNode::DeleteNetworkRoute),
                "listnetworkroute" => Ok(PermissionNode::ListNetworkRoute),
                "approvenetworkroute" => Ok(PermissionNode::ApproveNetworkRoute),
                "rejectnetworkroute" => Ok(PermissionNode::RejectNetworkRoute),
                "archivenetworkroute" => Ok(PermissionNode::ArchiveNetworkRoute),
                "restorenetworkroute" => Ok(PermissionNode::RestoreNetworkRoute),
                "transfernetworkroute" => Ok(PermissionNode::TransferNetworkRoute),
                _ => Ok(PermissionNode::Unknown(s.to_string())),
            }
        }
    }

    impl PermissionNode {
        pub fn requires_mfa(&self) -> bool {
            match self {
                PermissionNode::CreateUser => false,
                PermissionNode::ReadUser => false,
                PermissionNode::UpdateUser => false,
                PermissionNode::DeleteUser => true,
                PermissionNode::ListUser => false,
                PermissionNode::ApproveUser => true,
                PermissionNode::RejectUser => false,
                PermissionNode::ArchiveUser => false,
                PermissionNode::RestoreUser => false,
                PermissionNode::TransferUser => false,
                PermissionNode::CreateRole => false,
                PermissionNode::ReadRole => false,
                PermissionNode::UpdateRole => false,
                PermissionNode::DeleteRole => true,
                PermissionNode::ListRole => false,
                PermissionNode::ApproveRole => true,
                PermissionNode::RejectRole => false,
                PermissionNode::ArchiveRole => false,
                PermissionNode::RestoreRole => false,
                PermissionNode::TransferRole => false,
                PermissionNode::CreateTenant => false,
                PermissionNode::ReadTenant => false,
                PermissionNode::UpdateTenant => false,
                PermissionNode::DeleteTenant => true,
                PermissionNode::ListTenant => false,
                PermissionNode::ApproveTenant => true,
                PermissionNode::RejectTenant => false,
                PermissionNode::ArchiveTenant => false,
                PermissionNode::RestoreTenant => false,
                PermissionNode::TransferTenant => false,
                PermissionNode::CreateBillingProfile => true,
                PermissionNode::ReadBillingProfile => true,
                PermissionNode::UpdateBillingProfile => true,
                PermissionNode::DeleteBillingProfile => true,
                PermissionNode::ListBillingProfile => true,
                PermissionNode::ApproveBillingProfile => true,
                PermissionNode::RejectBillingProfile => true,
                PermissionNode::ArchiveBillingProfile => true,
                PermissionNode::RestoreBillingProfile => true,
                PermissionNode::TransferBillingProfile => true,
                PermissionNode::CreateAgent => false,
                PermissionNode::ReadAgent => false,
                PermissionNode::UpdateAgent => false,
                PermissionNode::DeleteAgent => true,
                PermissionNode::ListAgent => false,
                PermissionNode::ApproveAgent => true,
                PermissionNode::RejectAgent => false,
                PermissionNode::ArchiveAgent => false,
                PermissionNode::RestoreAgent => false,
                PermissionNode::TransferAgent => false,
                PermissionNode::CreateTask => false,
                PermissionNode::ReadTask => false,
                PermissionNode::UpdateTask => false,
                PermissionNode::DeleteTask => true,
                PermissionNode::ListTask => false,
                PermissionNode::ApproveTask => true,
                PermissionNode::RejectTask => false,
                PermissionNode::ArchiveTask => false,
                PermissionNode::RestoreTask => false,
                PermissionNode::TransferTask => false,
                PermissionNode::CreateMeeting => false,
                PermissionNode::ReadMeeting => false,
                PermissionNode::UpdateMeeting => false,
                PermissionNode::DeleteMeeting => true,
                PermissionNode::ListMeeting => false,
                PermissionNode::ApproveMeeting => true,
                PermissionNode::RejectMeeting => false,
                PermissionNode::ArchiveMeeting => false,
                PermissionNode::RestoreMeeting => false,
                PermissionNode::TransferMeeting => false,
                PermissionNode::CreateIntegration => false,
                PermissionNode::ReadIntegration => false,
                PermissionNode::UpdateIntegration => false,
                PermissionNode::DeleteIntegration => true,
                PermissionNode::ListIntegration => false,
                PermissionNode::ApproveIntegration => true,
                PermissionNode::RejectIntegration => false,
                PermissionNode::ArchiveIntegration => false,
                PermissionNode::RestoreIntegration => false,
                PermissionNode::TransferIntegration => false,
                PermissionNode::CreateApiKey => false,
                PermissionNode::ReadApiKey => false,
                PermissionNode::UpdateApiKey => false,
                PermissionNode::DeleteApiKey => true,
                PermissionNode::ListApiKey => false,
                PermissionNode::ApproveApiKey => true,
                PermissionNode::RejectApiKey => false,
                PermissionNode::ArchiveApiKey => false,
                PermissionNode::RestoreApiKey => false,
                PermissionNode::TransferApiKey => false,
                PermissionNode::CreateWebhook => false,
                PermissionNode::ReadWebhook => false,
                PermissionNode::UpdateWebhook => false,
                PermissionNode::DeleteWebhook => true,
                PermissionNode::ListWebhook => false,
                PermissionNode::ApproveWebhook => true,
                PermissionNode::RejectWebhook => false,
                PermissionNode::ArchiveWebhook => false,
                PermissionNode::RestoreWebhook => false,
                PermissionNode::TransferWebhook => false,
                PermissionNode::CreateInvoice => false,
                PermissionNode::ReadInvoice => false,
                PermissionNode::UpdateInvoice => false,
                PermissionNode::DeleteInvoice => true,
                PermissionNode::ListInvoice => false,
                PermissionNode::ApproveInvoice => true,
                PermissionNode::RejectInvoice => false,
                PermissionNode::ArchiveInvoice => false,
                PermissionNode::RestoreInvoice => false,
                PermissionNode::TransferInvoice => false,
                PermissionNode::CreateSubscription => false,
                PermissionNode::ReadSubscription => false,
                PermissionNode::UpdateSubscription => false,
                PermissionNode::DeleteSubscription => true,
                PermissionNode::ListSubscription => false,
                PermissionNode::ApproveSubscription => true,
                PermissionNode::RejectSubscription => false,
                PermissionNode::ArchiveSubscription => false,
                PermissionNode::RestoreSubscription => false,
                PermissionNode::TransferSubscription => false,
                PermissionNode::CreateAuditLog => false,
                PermissionNode::ReadAuditLog => false,
                PermissionNode::UpdateAuditLog => false,
                PermissionNode::DeleteAuditLog => true,
                PermissionNode::ListAuditLog => false,
                PermissionNode::ApproveAuditLog => true,
                PermissionNode::RejectAuditLog => false,
                PermissionNode::ArchiveAuditLog => false,
                PermissionNode::RestoreAuditLog => false,
                PermissionNode::TransferAuditLog => false,
                PermissionNode::CreateSecurityPolicy => true,
                PermissionNode::ReadSecurityPolicy => true,
                PermissionNode::UpdateSecurityPolicy => true,
                PermissionNode::DeleteSecurityPolicy => true,
                PermissionNode::ListSecurityPolicy => true,
                PermissionNode::ApproveSecurityPolicy => true,
                PermissionNode::RejectSecurityPolicy => true,
                PermissionNode::ArchiveSecurityPolicy => true,
                PermissionNode::RestoreSecurityPolicy => true,
                PermissionNode::TransferSecurityPolicy => true,
                PermissionNode::CreateNetworkRoute => false,
                PermissionNode::ReadNetworkRoute => false,
                PermissionNode::UpdateNetworkRoute => false,
                PermissionNode::DeleteNetworkRoute => true,
                PermissionNode::ListNetworkRoute => false,
                PermissionNode::ApproveNetworkRoute => true,
                PermissionNode::RejectNetworkRoute => false,
                PermissionNode::ArchiveNetworkRoute => false,
                PermissionNode::RestoreNetworkRoute => false,
                PermissionNode::TransferNetworkRoute => false,
                PermissionNode::Unknown(_) => true,
            }
        }

        pub fn as_str(&self) -> &str {
            match self {
                PermissionNode::CreateUser => "createuser",
                PermissionNode::ReadUser => "readuser",
                PermissionNode::UpdateUser => "updateuser",
                PermissionNode::DeleteUser => "deleteuser",
                PermissionNode::ListUser => "listuser",
                PermissionNode::ApproveUser => "approveuser",
                PermissionNode::RejectUser => "rejectuser",
                PermissionNode::ArchiveUser => "archiveuser",
                PermissionNode::RestoreUser => "restoreuser",
                PermissionNode::TransferUser => "transferuser",
                PermissionNode::CreateRole => "createrole",
                PermissionNode::ReadRole => "readrole",
                PermissionNode::UpdateRole => "updaterole",
                PermissionNode::DeleteRole => "deleterole",
                PermissionNode::ListRole => "listrole",
                PermissionNode::ApproveRole => "approverole",
                PermissionNode::RejectRole => "rejectrole",
                PermissionNode::ArchiveRole => "archiverole",
                PermissionNode::RestoreRole => "restorerole",
                PermissionNode::TransferRole => "transferrole",
                PermissionNode::CreateTenant => "createtenant",
                PermissionNode::ReadTenant => "readtenant",
                PermissionNode::UpdateTenant => "updatetenant",
                PermissionNode::DeleteTenant => "deletetenant",
                PermissionNode::ListTenant => "listtenant",
                PermissionNode::ApproveTenant => "approvetenant",
                PermissionNode::RejectTenant => "rejecttenant",
                PermissionNode::ArchiveTenant => "archivetenant",
                PermissionNode::RestoreTenant => "restoretenant",
                PermissionNode::TransferTenant => "transfertenant",
                PermissionNode::CreateBillingProfile => "createbillingprofile",
                PermissionNode::ReadBillingProfile => "readbillingprofile",
                PermissionNode::UpdateBillingProfile => "updatebillingprofile",
                PermissionNode::DeleteBillingProfile => "deletebillingprofile",
                PermissionNode::ListBillingProfile => "listbillingprofile",
                PermissionNode::ApproveBillingProfile => "approvebillingprofile",
                PermissionNode::RejectBillingProfile => "rejectbillingprofile",
                PermissionNode::ArchiveBillingProfile => "archivebillingprofile",
                PermissionNode::RestoreBillingProfile => "restorebillingprofile",
                PermissionNode::TransferBillingProfile => "transferbillingprofile",
                PermissionNode::CreateAgent => "createagent",
                PermissionNode::ReadAgent => "readagent",
                PermissionNode::UpdateAgent => "updateagent",
                PermissionNode::DeleteAgent => "deleteagent",
                PermissionNode::ListAgent => "listagent",
                PermissionNode::ApproveAgent => "approveagent",
                PermissionNode::RejectAgent => "rejectagent",
                PermissionNode::ArchiveAgent => "archiveagent",
                PermissionNode::RestoreAgent => "restoreagent",
                PermissionNode::TransferAgent => "transferagent",
                PermissionNode::CreateTask => "createtask",
                PermissionNode::ReadTask => "readtask",
                PermissionNode::UpdateTask => "updatetask",
                PermissionNode::DeleteTask => "deletetask",
                PermissionNode::ListTask => "listtask",
                PermissionNode::ApproveTask => "approvetask",
                PermissionNode::RejectTask => "rejecttask",
                PermissionNode::ArchiveTask => "archivetask",
                PermissionNode::RestoreTask => "restoretask",
                PermissionNode::TransferTask => "transfertask",
                PermissionNode::CreateMeeting => "createmeeting",
                PermissionNode::ReadMeeting => "readmeeting",
                PermissionNode::UpdateMeeting => "updatemeeting",
                PermissionNode::DeleteMeeting => "deletemeeting",
                PermissionNode::ListMeeting => "listmeeting",
                PermissionNode::ApproveMeeting => "approvemeeting",
                PermissionNode::RejectMeeting => "rejectmeeting",
                PermissionNode::ArchiveMeeting => "archivemeeting",
                PermissionNode::RestoreMeeting => "restoremeeting",
                PermissionNode::TransferMeeting => "transfermeeting",
                PermissionNode::CreateIntegration => "createintegration",
                PermissionNode::ReadIntegration => "readintegration",
                PermissionNode::UpdateIntegration => "updateintegration",
                PermissionNode::DeleteIntegration => "deleteintegration",
                PermissionNode::ListIntegration => "listintegration",
                PermissionNode::ApproveIntegration => "approveintegration",
                PermissionNode::RejectIntegration => "rejectintegration",
                PermissionNode::ArchiveIntegration => "archiveintegration",
                PermissionNode::RestoreIntegration => "restoreintegration",
                PermissionNode::TransferIntegration => "transferintegration",
                PermissionNode::CreateApiKey => "createapikey",
                PermissionNode::ReadApiKey => "readapikey",
                PermissionNode::UpdateApiKey => "updateapikey",
                PermissionNode::DeleteApiKey => "deleteapikey",
                PermissionNode::ListApiKey => "listapikey",
                PermissionNode::ApproveApiKey => "approveapikey",
                PermissionNode::RejectApiKey => "rejectapikey",
                PermissionNode::ArchiveApiKey => "archiveapikey",
                PermissionNode::RestoreApiKey => "restoreapikey",
                PermissionNode::TransferApiKey => "transferapikey",
                PermissionNode::CreateWebhook => "createwebhook",
                PermissionNode::ReadWebhook => "readwebhook",
                PermissionNode::UpdateWebhook => "updatewebhook",
                PermissionNode::DeleteWebhook => "deletewebhook",
                PermissionNode::ListWebhook => "listwebhook",
                PermissionNode::ApproveWebhook => "approvewebhook",
                PermissionNode::RejectWebhook => "rejectwebhook",
                PermissionNode::ArchiveWebhook => "archivewebhook",
                PermissionNode::RestoreWebhook => "restorewebhook",
                PermissionNode::TransferWebhook => "transferwebhook",
                PermissionNode::CreateInvoice => "createinvoice",
                PermissionNode::ReadInvoice => "readinvoice",
                PermissionNode::UpdateInvoice => "updateinvoice",
                PermissionNode::DeleteInvoice => "deleteinvoice",
                PermissionNode::ListInvoice => "listinvoice",
                PermissionNode::ApproveInvoice => "approveinvoice",
                PermissionNode::RejectInvoice => "rejectinvoice",
                PermissionNode::ArchiveInvoice => "archiveinvoice",
                PermissionNode::RestoreInvoice => "restoreinvoice",
                PermissionNode::TransferInvoice => "transferinvoice",
                PermissionNode::CreateSubscription => "createsubscription",
                PermissionNode::ReadSubscription => "readsubscription",
                PermissionNode::UpdateSubscription => "updatesubscription",
                PermissionNode::DeleteSubscription => "deletesubscription",
                PermissionNode::ListSubscription => "listsubscription",
                PermissionNode::ApproveSubscription => "approvesubscription",
                PermissionNode::RejectSubscription => "rejectsubscription",
                PermissionNode::ArchiveSubscription => "archivesubscription",
                PermissionNode::RestoreSubscription => "restoresubscription",
                PermissionNode::TransferSubscription => "transfersubscription",
                PermissionNode::CreateAuditLog => "createauditlog",
                PermissionNode::ReadAuditLog => "readauditlog",
                PermissionNode::UpdateAuditLog => "updateauditlog",
                PermissionNode::DeleteAuditLog => "deleteauditlog",
                PermissionNode::ListAuditLog => "listauditlog",
                PermissionNode::ApproveAuditLog => "approveauditlog",
                PermissionNode::RejectAuditLog => "rejectauditlog",
                PermissionNode::ArchiveAuditLog => "archiveauditlog",
                PermissionNode::RestoreAuditLog => "restoreauditlog",
                PermissionNode::TransferAuditLog => "transferauditlog",
                PermissionNode::CreateSecurityPolicy => "createsecuritypolicy",
                PermissionNode::ReadSecurityPolicy => "readsecuritypolicy",
                PermissionNode::UpdateSecurityPolicy => "updatesecuritypolicy",
                PermissionNode::DeleteSecurityPolicy => "deletesecuritypolicy",
                PermissionNode::ListSecurityPolicy => "listsecuritypolicy",
                PermissionNode::ApproveSecurityPolicy => "approvesecuritypolicy",
                PermissionNode::RejectSecurityPolicy => "rejectsecuritypolicy",
                PermissionNode::ArchiveSecurityPolicy => "archivesecuritypolicy",
                PermissionNode::RestoreSecurityPolicy => "restoresecuritypolicy",
                PermissionNode::TransferSecurityPolicy => "transfersecuritypolicy",
                PermissionNode::CreateNetworkRoute => "createnetworkroute",
                PermissionNode::ReadNetworkRoute => "readnetworkroute",
                PermissionNode::UpdateNetworkRoute => "updatenetworkroute",
                PermissionNode::DeleteNetworkRoute => "deletenetworkroute",
                PermissionNode::ListNetworkRoute => "listnetworkroute",
                PermissionNode::ApproveNetworkRoute => "approvenetworkroute",
                PermissionNode::RejectNetworkRoute => "rejectnetworkroute",
                PermissionNode::ArchiveNetworkRoute => "archivenetworkroute",
                PermissionNode::RestoreNetworkRoute => "restorenetworkroute",
                PermissionNode::TransferNetworkRoute => "transfernetworkroute",
                PermissionNode::Unknown(s) => s.as_str(),
            }
        }
    }
}
pub mod granular_permissions_v2 {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum PermissionNode {
        CreateUser,
        ReadUser,
        UpdateUser,
        DeleteUser,
        ListUser,
        ApproveUser,
        RejectUser,
        ArchiveUser,
        RestoreUser,
        TransferUser,
        CreateRole,
        ReadRole,
        UpdateRole,
        DeleteRole,
        ListRole,
        ApproveRole,
        RejectRole,
        ArchiveRole,
        RestoreRole,
        TransferRole,
        CreateTenant,
        ReadTenant,
        UpdateTenant,
        DeleteTenant,
        ListTenant,
        ApproveTenant,
        RejectTenant,
        ArchiveTenant,
        RestoreTenant,
        TransferTenant,
        CreateBillingProfile,
        ReadBillingProfile,
        UpdateBillingProfile,
        DeleteBillingProfile,
        ListBillingProfile,
        ApproveBillingProfile,
        RejectBillingProfile,
        ArchiveBillingProfile,
        RestoreBillingProfile,
        TransferBillingProfile,
        CreateAgent,
        ReadAgent,
        UpdateAgent,
        DeleteAgent,
        ListAgent,
        ApproveAgent,
        RejectAgent,
        ArchiveAgent,
        RestoreAgent,
        TransferAgent,
        CreateTask,
        ReadTask,
        UpdateTask,
        DeleteTask,
        ListTask,
        ApproveTask,
        RejectTask,
        ArchiveTask,
        RestoreTask,
        TransferTask,
        CreateMeeting,
        ReadMeeting,
        UpdateMeeting,
        DeleteMeeting,
        ListMeeting,
        ApproveMeeting,
        RejectMeeting,
        ArchiveMeeting,
        RestoreMeeting,
        TransferMeeting,
        CreateIntegration,
        ReadIntegration,
        UpdateIntegration,
        DeleteIntegration,
        ListIntegration,
        ApproveIntegration,
        RejectIntegration,
        ArchiveIntegration,
        RestoreIntegration,
        TransferIntegration,
        CreateApiKey,
        ReadApiKey,
        UpdateApiKey,
        DeleteApiKey,
        ListApiKey,
        ApproveApiKey,
        RejectApiKey,
        ArchiveApiKey,
        RestoreApiKey,
        TransferApiKey,
        CreateWebhook,
        ReadWebhook,
        UpdateWebhook,
        DeleteWebhook,
        ListWebhook,
        ApproveWebhook,
        RejectWebhook,
        ArchiveWebhook,
        RestoreWebhook,
        TransferWebhook,
        CreateInvoice,
        ReadInvoice,
        UpdateInvoice,
        DeleteInvoice,
        ListInvoice,
        ApproveInvoice,
        RejectInvoice,
        ArchiveInvoice,
        RestoreInvoice,
        TransferInvoice,
        CreateSubscription,
        ReadSubscription,
        UpdateSubscription,
        DeleteSubscription,
        ListSubscription,
        ApproveSubscription,
        RejectSubscription,
        ArchiveSubscription,
        RestoreSubscription,
        TransferSubscription,
        CreateAuditLog,
        ReadAuditLog,
        UpdateAuditLog,
        DeleteAuditLog,
        ListAuditLog,
        ApproveAuditLog,
        RejectAuditLog,
        ArchiveAuditLog,
        RestoreAuditLog,
        TransferAuditLog,
        CreateSecurityPolicy,
        ReadSecurityPolicy,
        UpdateSecurityPolicy,
        DeleteSecurityPolicy,
        ListSecurityPolicy,
        ApproveSecurityPolicy,
        RejectSecurityPolicy,
        ArchiveSecurityPolicy,
        RestoreSecurityPolicy,
        TransferSecurityPolicy,
        CreateNetworkRoute,
        ReadNetworkRoute,
        UpdateNetworkRoute,
        DeleteNetworkRoute,
        ListNetworkRoute,
        ApproveNetworkRoute,
        RejectNetworkRoute,
        ArchiveNetworkRoute,
        RestoreNetworkRoute,
        TransferNetworkRoute,
        Unknown(String),
    }

    impl std::str::FromStr for PermissionNode {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "createuser" => Ok(PermissionNode::CreateUser),
                "readuser" => Ok(PermissionNode::ReadUser),
                "updateuser" => Ok(PermissionNode::UpdateUser),
                "deleteuser" => Ok(PermissionNode::DeleteUser),
                "listuser" => Ok(PermissionNode::ListUser),
                "approveuser" => Ok(PermissionNode::ApproveUser),
                "rejectuser" => Ok(PermissionNode::RejectUser),
                "archiveuser" => Ok(PermissionNode::ArchiveUser),
                "restoreuser" => Ok(PermissionNode::RestoreUser),
                "transferuser" => Ok(PermissionNode::TransferUser),
                "createrole" => Ok(PermissionNode::CreateRole),
                "readrole" => Ok(PermissionNode::ReadRole),
                "updaterole" => Ok(PermissionNode::UpdateRole),
                "deleterole" => Ok(PermissionNode::DeleteRole),
                "listrole" => Ok(PermissionNode::ListRole),
                "approverole" => Ok(PermissionNode::ApproveRole),
                "rejectrole" => Ok(PermissionNode::RejectRole),
                "archiverole" => Ok(PermissionNode::ArchiveRole),
                "restorerole" => Ok(PermissionNode::RestoreRole),
                "transferrole" => Ok(PermissionNode::TransferRole),
                "createtenant" => Ok(PermissionNode::CreateTenant),
                "readtenant" => Ok(PermissionNode::ReadTenant),
                "updatetenant" => Ok(PermissionNode::UpdateTenant),
                "deletetenant" => Ok(PermissionNode::DeleteTenant),
                "listtenant" => Ok(PermissionNode::ListTenant),
                "approvetenant" => Ok(PermissionNode::ApproveTenant),
                "rejecttenant" => Ok(PermissionNode::RejectTenant),
                "archivetenant" => Ok(PermissionNode::ArchiveTenant),
                "restoretenant" => Ok(PermissionNode::RestoreTenant),
                "transfertenant" => Ok(PermissionNode::TransferTenant),
                "createbillingprofile" => Ok(PermissionNode::CreateBillingProfile),
                "readbillingprofile" => Ok(PermissionNode::ReadBillingProfile),
                "updatebillingprofile" => Ok(PermissionNode::UpdateBillingProfile),
                "deletebillingprofile" => Ok(PermissionNode::DeleteBillingProfile),
                "listbillingprofile" => Ok(PermissionNode::ListBillingProfile),
                "approvebillingprofile" => Ok(PermissionNode::ApproveBillingProfile),
                "rejectbillingprofile" => Ok(PermissionNode::RejectBillingProfile),
                "archivebillingprofile" => Ok(PermissionNode::ArchiveBillingProfile),
                "restorebillingprofile" => Ok(PermissionNode::RestoreBillingProfile),
                "transferbillingprofile" => Ok(PermissionNode::TransferBillingProfile),
                "createagent" => Ok(PermissionNode::CreateAgent),
                "readagent" => Ok(PermissionNode::ReadAgent),
                "updateagent" => Ok(PermissionNode::UpdateAgent),
                "deleteagent" => Ok(PermissionNode::DeleteAgent),
                "listagent" => Ok(PermissionNode::ListAgent),
                "approveagent" => Ok(PermissionNode::ApproveAgent),
                "rejectagent" => Ok(PermissionNode::RejectAgent),
                "archiveagent" => Ok(PermissionNode::ArchiveAgent),
                "restoreagent" => Ok(PermissionNode::RestoreAgent),
                "transferagent" => Ok(PermissionNode::TransferAgent),
                "createtask" => Ok(PermissionNode::CreateTask),
                "readtask" => Ok(PermissionNode::ReadTask),
                "updatetask" => Ok(PermissionNode::UpdateTask),
                "deletetask" => Ok(PermissionNode::DeleteTask),
                "listtask" => Ok(PermissionNode::ListTask),
                "approvetask" => Ok(PermissionNode::ApproveTask),
                "rejecttask" => Ok(PermissionNode::RejectTask),
                "archivetask" => Ok(PermissionNode::ArchiveTask),
                "restoretask" => Ok(PermissionNode::RestoreTask),
                "transfertask" => Ok(PermissionNode::TransferTask),
                "createmeeting" => Ok(PermissionNode::CreateMeeting),
                "readmeeting" => Ok(PermissionNode::ReadMeeting),
                "updatemeeting" => Ok(PermissionNode::UpdateMeeting),
                "deletemeeting" => Ok(PermissionNode::DeleteMeeting),
                "listmeeting" => Ok(PermissionNode::ListMeeting),
                "approvemeeting" => Ok(PermissionNode::ApproveMeeting),
                "rejectmeeting" => Ok(PermissionNode::RejectMeeting),
                "archivemeeting" => Ok(PermissionNode::ArchiveMeeting),
                "restoremeeting" => Ok(PermissionNode::RestoreMeeting),
                "transfermeeting" => Ok(PermissionNode::TransferMeeting),
                "createintegration" => Ok(PermissionNode::CreateIntegration),
                "readintegration" => Ok(PermissionNode::ReadIntegration),
                "updateintegration" => Ok(PermissionNode::UpdateIntegration),
                "deleteintegration" => Ok(PermissionNode::DeleteIntegration),
                "listintegration" => Ok(PermissionNode::ListIntegration),
                "approveintegration" => Ok(PermissionNode::ApproveIntegration),
                "rejectintegration" => Ok(PermissionNode::RejectIntegration),
                "archiveintegration" => Ok(PermissionNode::ArchiveIntegration),
                "restoreintegration" => Ok(PermissionNode::RestoreIntegration),
                "transferintegration" => Ok(PermissionNode::TransferIntegration),
                "createapikey" => Ok(PermissionNode::CreateApiKey),
                "readapikey" => Ok(PermissionNode::ReadApiKey),
                "updateapikey" => Ok(PermissionNode::UpdateApiKey),
                "deleteapikey" => Ok(PermissionNode::DeleteApiKey),
                "listapikey" => Ok(PermissionNode::ListApiKey),
                "approveapikey" => Ok(PermissionNode::ApproveApiKey),
                "rejectapikey" => Ok(PermissionNode::RejectApiKey),
                "archiveapikey" => Ok(PermissionNode::ArchiveApiKey),
                "restoreapikey" => Ok(PermissionNode::RestoreApiKey),
                "transferapikey" => Ok(PermissionNode::TransferApiKey),
                "createwebhook" => Ok(PermissionNode::CreateWebhook),
                "readwebhook" => Ok(PermissionNode::ReadWebhook),
                "updatewebhook" => Ok(PermissionNode::UpdateWebhook),
                "deletewebhook" => Ok(PermissionNode::DeleteWebhook),
                "listwebhook" => Ok(PermissionNode::ListWebhook),
                "approvewebhook" => Ok(PermissionNode::ApproveWebhook),
                "rejectwebhook" => Ok(PermissionNode::RejectWebhook),
                "archivewebhook" => Ok(PermissionNode::ArchiveWebhook),
                "restorewebhook" => Ok(PermissionNode::RestoreWebhook),
                "transferwebhook" => Ok(PermissionNode::TransferWebhook),
                "createinvoice" => Ok(PermissionNode::CreateInvoice),
                "readinvoice" => Ok(PermissionNode::ReadInvoice),
                "updateinvoice" => Ok(PermissionNode::UpdateInvoice),
                "deleteinvoice" => Ok(PermissionNode::DeleteInvoice),
                "listinvoice" => Ok(PermissionNode::ListInvoice),
                "approveinvoice" => Ok(PermissionNode::ApproveInvoice),
                "rejectinvoice" => Ok(PermissionNode::RejectInvoice),
                "archiveinvoice" => Ok(PermissionNode::ArchiveInvoice),
                "restoreinvoice" => Ok(PermissionNode::RestoreInvoice),
                "transferinvoice" => Ok(PermissionNode::TransferInvoice),
                "createsubscription" => Ok(PermissionNode::CreateSubscription),
                "readsubscription" => Ok(PermissionNode::ReadSubscription),
                "updatesubscription" => Ok(PermissionNode::UpdateSubscription),
                "deletesubscription" => Ok(PermissionNode::DeleteSubscription),
                "listsubscription" => Ok(PermissionNode::ListSubscription),
                "approvesubscription" => Ok(PermissionNode::ApproveSubscription),
                "rejectsubscription" => Ok(PermissionNode::RejectSubscription),
                "archivesubscription" => Ok(PermissionNode::ArchiveSubscription),
                "restoresubscription" => Ok(PermissionNode::RestoreSubscription),
                "transfersubscription" => Ok(PermissionNode::TransferSubscription),
                "createauditlog" => Ok(PermissionNode::CreateAuditLog),
                "readauditlog" => Ok(PermissionNode::ReadAuditLog),
                "updateauditlog" => Ok(PermissionNode::UpdateAuditLog),
                "deleteauditlog" => Ok(PermissionNode::DeleteAuditLog),
                "listauditlog" => Ok(PermissionNode::ListAuditLog),
                "approveauditlog" => Ok(PermissionNode::ApproveAuditLog),
                "rejectauditlog" => Ok(PermissionNode::RejectAuditLog),
                "archiveauditlog" => Ok(PermissionNode::ArchiveAuditLog),
                "restoreauditlog" => Ok(PermissionNode::RestoreAuditLog),
                "transferauditlog" => Ok(PermissionNode::TransferAuditLog),
                "createsecuritypolicy" => Ok(PermissionNode::CreateSecurityPolicy),
                "readsecuritypolicy" => Ok(PermissionNode::ReadSecurityPolicy),
                "updatesecuritypolicy" => Ok(PermissionNode::UpdateSecurityPolicy),
                "deletesecuritypolicy" => Ok(PermissionNode::DeleteSecurityPolicy),
                "listsecuritypolicy" => Ok(PermissionNode::ListSecurityPolicy),
                "approvesecuritypolicy" => Ok(PermissionNode::ApproveSecurityPolicy),
                "rejectsecuritypolicy" => Ok(PermissionNode::RejectSecurityPolicy),
                "archivesecuritypolicy" => Ok(PermissionNode::ArchiveSecurityPolicy),
                "restoresecuritypolicy" => Ok(PermissionNode::RestoreSecurityPolicy),
                "transfersecuritypolicy" => Ok(PermissionNode::TransferSecurityPolicy),
                "createnetworkroute" => Ok(PermissionNode::CreateNetworkRoute),
                "readnetworkroute" => Ok(PermissionNode::ReadNetworkRoute),
                "updatenetworkroute" => Ok(PermissionNode::UpdateNetworkRoute),
                "deletenetworkroute" => Ok(PermissionNode::DeleteNetworkRoute),
                "listnetworkroute" => Ok(PermissionNode::ListNetworkRoute),
                "approvenetworkroute" => Ok(PermissionNode::ApproveNetworkRoute),
                "rejectnetworkroute" => Ok(PermissionNode::RejectNetworkRoute),
                "archivenetworkroute" => Ok(PermissionNode::ArchiveNetworkRoute),
                "restorenetworkroute" => Ok(PermissionNode::RestoreNetworkRoute),
                "transfernetworkroute" => Ok(PermissionNode::TransferNetworkRoute),
                _ => Ok(PermissionNode::Unknown(s.to_string())),
            }
        }
    }

    impl PermissionNode {
        pub fn requires_mfa(&self) -> bool {
            match self {
                PermissionNode::CreateUser => false,
                PermissionNode::ReadUser => false,
                PermissionNode::UpdateUser => false,
                PermissionNode::DeleteUser => true,
                PermissionNode::ListUser => false,
                PermissionNode::ApproveUser => true,
                PermissionNode::RejectUser => false,
                PermissionNode::ArchiveUser => false,
                PermissionNode::RestoreUser => false,
                PermissionNode::TransferUser => false,
                PermissionNode::CreateRole => false,
                PermissionNode::ReadRole => false,
                PermissionNode::UpdateRole => false,
                PermissionNode::DeleteRole => true,
                PermissionNode::ListRole => false,
                PermissionNode::ApproveRole => true,
                PermissionNode::RejectRole => false,
                PermissionNode::ArchiveRole => false,
                PermissionNode::RestoreRole => false,
                PermissionNode::TransferRole => false,
                PermissionNode::CreateTenant => false,
                PermissionNode::ReadTenant => false,
                PermissionNode::UpdateTenant => false,
                PermissionNode::DeleteTenant => true,
                PermissionNode::ListTenant => false,
                PermissionNode::ApproveTenant => true,
                PermissionNode::RejectTenant => false,
                PermissionNode::ArchiveTenant => false,
                PermissionNode::RestoreTenant => false,
                PermissionNode::TransferTenant => false,
                PermissionNode::CreateBillingProfile => true,
                PermissionNode::ReadBillingProfile => true,
                PermissionNode::UpdateBillingProfile => true,
                PermissionNode::DeleteBillingProfile => true,
                PermissionNode::ListBillingProfile => true,
                PermissionNode::ApproveBillingProfile => true,
                PermissionNode::RejectBillingProfile => true,
                PermissionNode::ArchiveBillingProfile => true,
                PermissionNode::RestoreBillingProfile => true,
                PermissionNode::TransferBillingProfile => true,
                PermissionNode::CreateAgent => false,
                PermissionNode::ReadAgent => false,
                PermissionNode::UpdateAgent => false,
                PermissionNode::DeleteAgent => true,
                PermissionNode::ListAgent => false,
                PermissionNode::ApproveAgent => true,
                PermissionNode::RejectAgent => false,
                PermissionNode::ArchiveAgent => false,
                PermissionNode::RestoreAgent => false,
                PermissionNode::TransferAgent => false,
                PermissionNode::CreateTask => false,
                PermissionNode::ReadTask => false,
                PermissionNode::UpdateTask => false,
                PermissionNode::DeleteTask => true,
                PermissionNode::ListTask => false,
                PermissionNode::ApproveTask => true,
                PermissionNode::RejectTask => false,
                PermissionNode::ArchiveTask => false,
                PermissionNode::RestoreTask => false,
                PermissionNode::TransferTask => false,
                PermissionNode::CreateMeeting => false,
                PermissionNode::ReadMeeting => false,
                PermissionNode::UpdateMeeting => false,
                PermissionNode::DeleteMeeting => true,
                PermissionNode::ListMeeting => false,
                PermissionNode::ApproveMeeting => true,
                PermissionNode::RejectMeeting => false,
                PermissionNode::ArchiveMeeting => false,
                PermissionNode::RestoreMeeting => false,
                PermissionNode::TransferMeeting => false,
                PermissionNode::CreateIntegration => false,
                PermissionNode::ReadIntegration => false,
                PermissionNode::UpdateIntegration => false,
                PermissionNode::DeleteIntegration => true,
                PermissionNode::ListIntegration => false,
                PermissionNode::ApproveIntegration => true,
                PermissionNode::RejectIntegration => false,
                PermissionNode::ArchiveIntegration => false,
                PermissionNode::RestoreIntegration => false,
                PermissionNode::TransferIntegration => false,
                PermissionNode::CreateApiKey => false,
                PermissionNode::ReadApiKey => false,
                PermissionNode::UpdateApiKey => false,
                PermissionNode::DeleteApiKey => true,
                PermissionNode::ListApiKey => false,
                PermissionNode::ApproveApiKey => true,
                PermissionNode::RejectApiKey => false,
                PermissionNode::ArchiveApiKey => false,
                PermissionNode::RestoreApiKey => false,
                PermissionNode::TransferApiKey => false,
                PermissionNode::CreateWebhook => false,
                PermissionNode::ReadWebhook => false,
                PermissionNode::UpdateWebhook => false,
                PermissionNode::DeleteWebhook => true,
                PermissionNode::ListWebhook => false,
                PermissionNode::ApproveWebhook => true,
                PermissionNode::RejectWebhook => false,
                PermissionNode::ArchiveWebhook => false,
                PermissionNode::RestoreWebhook => false,
                PermissionNode::TransferWebhook => false,
                PermissionNode::CreateInvoice => false,
                PermissionNode::ReadInvoice => false,
                PermissionNode::UpdateInvoice => false,
                PermissionNode::DeleteInvoice => true,
                PermissionNode::ListInvoice => false,
                PermissionNode::ApproveInvoice => true,
                PermissionNode::RejectInvoice => false,
                PermissionNode::ArchiveInvoice => false,
                PermissionNode::RestoreInvoice => false,
                PermissionNode::TransferInvoice => false,
                PermissionNode::CreateSubscription => false,
                PermissionNode::ReadSubscription => false,
                PermissionNode::UpdateSubscription => false,
                PermissionNode::DeleteSubscription => true,
                PermissionNode::ListSubscription => false,
                PermissionNode::ApproveSubscription => true,
                PermissionNode::RejectSubscription => false,
                PermissionNode::ArchiveSubscription => false,
                PermissionNode::RestoreSubscription => false,
                PermissionNode::TransferSubscription => false,
                PermissionNode::CreateAuditLog => false,
                PermissionNode::ReadAuditLog => false,
                PermissionNode::UpdateAuditLog => false,
                PermissionNode::DeleteAuditLog => true,
                PermissionNode::ListAuditLog => false,
                PermissionNode::ApproveAuditLog => true,
                PermissionNode::RejectAuditLog => false,
                PermissionNode::ArchiveAuditLog => false,
                PermissionNode::RestoreAuditLog => false,
                PermissionNode::TransferAuditLog => false,
                PermissionNode::CreateSecurityPolicy => true,
                PermissionNode::ReadSecurityPolicy => true,
                PermissionNode::UpdateSecurityPolicy => true,
                PermissionNode::DeleteSecurityPolicy => true,
                PermissionNode::ListSecurityPolicy => true,
                PermissionNode::ApproveSecurityPolicy => true,
                PermissionNode::RejectSecurityPolicy => true,
                PermissionNode::ArchiveSecurityPolicy => true,
                PermissionNode::RestoreSecurityPolicy => true,
                PermissionNode::TransferSecurityPolicy => true,
                PermissionNode::CreateNetworkRoute => false,
                PermissionNode::ReadNetworkRoute => false,
                PermissionNode::UpdateNetworkRoute => false,
                PermissionNode::DeleteNetworkRoute => true,
                PermissionNode::ListNetworkRoute => false,
                PermissionNode::ApproveNetworkRoute => true,
                PermissionNode::RejectNetworkRoute => false,
                PermissionNode::ArchiveNetworkRoute => false,
                PermissionNode::RestoreNetworkRoute => false,
                PermissionNode::TransferNetworkRoute => false,
                PermissionNode::Unknown(_) => true,
            }
        }

        pub fn as_str(&self) -> &str {
            match self {
                PermissionNode::CreateUser => "createuser",
                PermissionNode::ReadUser => "readuser",
                PermissionNode::UpdateUser => "updateuser",
                PermissionNode::DeleteUser => "deleteuser",
                PermissionNode::ListUser => "listuser",
                PermissionNode::ApproveUser => "approveuser",
                PermissionNode::RejectUser => "rejectuser",
                PermissionNode::ArchiveUser => "archiveuser",
                PermissionNode::RestoreUser => "restoreuser",
                PermissionNode::TransferUser => "transferuser",
                PermissionNode::CreateRole => "createrole",
                PermissionNode::ReadRole => "readrole",
                PermissionNode::UpdateRole => "updaterole",
                PermissionNode::DeleteRole => "deleterole",
                PermissionNode::ListRole => "listrole",
                PermissionNode::ApproveRole => "approverole",
                PermissionNode::RejectRole => "rejectrole",
                PermissionNode::ArchiveRole => "archiverole",
                PermissionNode::RestoreRole => "restorerole",
                PermissionNode::TransferRole => "transferrole",
                PermissionNode::CreateTenant => "createtenant",
                PermissionNode::ReadTenant => "readtenant",
                PermissionNode::UpdateTenant => "updatetenant",
                PermissionNode::DeleteTenant => "deletetenant",
                PermissionNode::ListTenant => "listtenant",
                PermissionNode::ApproveTenant => "approvetenant",
                PermissionNode::RejectTenant => "rejecttenant",
                PermissionNode::ArchiveTenant => "archivetenant",
                PermissionNode::RestoreTenant => "restoretenant",
                PermissionNode::TransferTenant => "transfertenant",
                PermissionNode::CreateBillingProfile => "createbillingprofile",
                PermissionNode::ReadBillingProfile => "readbillingprofile",
                PermissionNode::UpdateBillingProfile => "updatebillingprofile",
                PermissionNode::DeleteBillingProfile => "deletebillingprofile",
                PermissionNode::ListBillingProfile => "listbillingprofile",
                PermissionNode::ApproveBillingProfile => "approvebillingprofile",
                PermissionNode::RejectBillingProfile => "rejectbillingprofile",
                PermissionNode::ArchiveBillingProfile => "archivebillingprofile",
                PermissionNode::RestoreBillingProfile => "restorebillingprofile",
                PermissionNode::TransferBillingProfile => "transferbillingprofile",
                PermissionNode::CreateAgent => "createagent",
                PermissionNode::ReadAgent => "readagent",
                PermissionNode::UpdateAgent => "updateagent",
                PermissionNode::DeleteAgent => "deleteagent",
                PermissionNode::ListAgent => "listagent",
                PermissionNode::ApproveAgent => "approveagent",
                PermissionNode::RejectAgent => "rejectagent",
                PermissionNode::ArchiveAgent => "archiveagent",
                PermissionNode::RestoreAgent => "restoreagent",
                PermissionNode::TransferAgent => "transferagent",
                PermissionNode::CreateTask => "createtask",
                PermissionNode::ReadTask => "readtask",
                PermissionNode::UpdateTask => "updatetask",
                PermissionNode::DeleteTask => "deletetask",
                PermissionNode::ListTask => "listtask",
                PermissionNode::ApproveTask => "approvetask",
                PermissionNode::RejectTask => "rejecttask",
                PermissionNode::ArchiveTask => "archivetask",
                PermissionNode::RestoreTask => "restoretask",
                PermissionNode::TransferTask => "transfertask",
                PermissionNode::CreateMeeting => "createmeeting",
                PermissionNode::ReadMeeting => "readmeeting",
                PermissionNode::UpdateMeeting => "updatemeeting",
                PermissionNode::DeleteMeeting => "deletemeeting",
                PermissionNode::ListMeeting => "listmeeting",
                PermissionNode::ApproveMeeting => "approvemeeting",
                PermissionNode::RejectMeeting => "rejectmeeting",
                PermissionNode::ArchiveMeeting => "archivemeeting",
                PermissionNode::RestoreMeeting => "restoremeeting",
                PermissionNode::TransferMeeting => "transfermeeting",
                PermissionNode::CreateIntegration => "createintegration",
                PermissionNode::ReadIntegration => "readintegration",
                PermissionNode::UpdateIntegration => "updateintegration",
                PermissionNode::DeleteIntegration => "deleteintegration",
                PermissionNode::ListIntegration => "listintegration",
                PermissionNode::ApproveIntegration => "approveintegration",
                PermissionNode::RejectIntegration => "rejectintegration",
                PermissionNode::ArchiveIntegration => "archiveintegration",
                PermissionNode::RestoreIntegration => "restoreintegration",
                PermissionNode::TransferIntegration => "transferintegration",
                PermissionNode::CreateApiKey => "createapikey",
                PermissionNode::ReadApiKey => "readapikey",
                PermissionNode::UpdateApiKey => "updateapikey",
                PermissionNode::DeleteApiKey => "deleteapikey",
                PermissionNode::ListApiKey => "listapikey",
                PermissionNode::ApproveApiKey => "approveapikey",
                PermissionNode::RejectApiKey => "rejectapikey",
                PermissionNode::ArchiveApiKey => "archiveapikey",
                PermissionNode::RestoreApiKey => "restoreapikey",
                PermissionNode::TransferApiKey => "transferapikey",
                PermissionNode::CreateWebhook => "createwebhook",
                PermissionNode::ReadWebhook => "readwebhook",
                PermissionNode::UpdateWebhook => "updatewebhook",
                PermissionNode::DeleteWebhook => "deletewebhook",
                PermissionNode::ListWebhook => "listwebhook",
                PermissionNode::ApproveWebhook => "approvewebhook",
                PermissionNode::RejectWebhook => "rejectwebhook",
                PermissionNode::ArchiveWebhook => "archivewebhook",
                PermissionNode::RestoreWebhook => "restorewebhook",
                PermissionNode::TransferWebhook => "transferwebhook",
                PermissionNode::CreateInvoice => "createinvoice",
                PermissionNode::ReadInvoice => "readinvoice",
                PermissionNode::UpdateInvoice => "updateinvoice",
                PermissionNode::DeleteInvoice => "deleteinvoice",
                PermissionNode::ListInvoice => "listinvoice",
                PermissionNode::ApproveInvoice => "approveinvoice",
                PermissionNode::RejectInvoice => "rejectinvoice",
                PermissionNode::ArchiveInvoice => "archiveinvoice",
                PermissionNode::RestoreInvoice => "restoreinvoice",
                PermissionNode::TransferInvoice => "transferinvoice",
                PermissionNode::CreateSubscription => "createsubscription",
                PermissionNode::ReadSubscription => "readsubscription",
                PermissionNode::UpdateSubscription => "updatesubscription",
                PermissionNode::DeleteSubscription => "deletesubscription",
                PermissionNode::ListSubscription => "listsubscription",
                PermissionNode::ApproveSubscription => "approvesubscription",
                PermissionNode::RejectSubscription => "rejectsubscription",
                PermissionNode::ArchiveSubscription => "archivesubscription",
                PermissionNode::RestoreSubscription => "restoresubscription",
                PermissionNode::TransferSubscription => "transfersubscription",
                PermissionNode::CreateAuditLog => "createauditlog",
                PermissionNode::ReadAuditLog => "readauditlog",
                PermissionNode::UpdateAuditLog => "updateauditlog",
                PermissionNode::DeleteAuditLog => "deleteauditlog",
                PermissionNode::ListAuditLog => "listauditlog",
                PermissionNode::ApproveAuditLog => "approveauditlog",
                PermissionNode::RejectAuditLog => "rejectauditlog",
                PermissionNode::ArchiveAuditLog => "archiveauditlog",
                PermissionNode::RestoreAuditLog => "restoreauditlog",
                PermissionNode::TransferAuditLog => "transferauditlog",
                PermissionNode::CreateSecurityPolicy => "createsecuritypolicy",
                PermissionNode::ReadSecurityPolicy => "readsecuritypolicy",
                PermissionNode::UpdateSecurityPolicy => "updatesecuritypolicy",
                PermissionNode::DeleteSecurityPolicy => "deletesecuritypolicy",
                PermissionNode::ListSecurityPolicy => "listsecuritypolicy",
                PermissionNode::ApproveSecurityPolicy => "approvesecuritypolicy",
                PermissionNode::RejectSecurityPolicy => "rejectsecuritypolicy",
                PermissionNode::ArchiveSecurityPolicy => "archivesecuritypolicy",
                PermissionNode::RestoreSecurityPolicy => "restoresecuritypolicy",
                PermissionNode::TransferSecurityPolicy => "transfersecuritypolicy",
                PermissionNode::CreateNetworkRoute => "createnetworkroute",
                PermissionNode::ReadNetworkRoute => "readnetworkroute",
                PermissionNode::UpdateNetworkRoute => "updatenetworkroute",
                PermissionNode::DeleteNetworkRoute => "deletenetworkroute",
                PermissionNode::ListNetworkRoute => "listnetworkroute",
                PermissionNode::ApproveNetworkRoute => "approvenetworkroute",
                PermissionNode::RejectNetworkRoute => "rejectnetworkroute",
                PermissionNode::ArchiveNetworkRoute => "archivenetworkroute",
                PermissionNode::RestoreNetworkRoute => "restorenetworkroute",
                PermissionNode::TransferNetworkRoute => "transfernetworkroute",
                PermissionNode::Unknown(s) => s.as_str(),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum AuditEventCategory {
        LoginInfo,
        LoginWarning,
        LoginCritical,
        LogoutInfo,
        LogoutWarning,
        LogoutCritical,
        TokenIssueInfo,
        TokenIssueWarning,
        TokenIssueCritical,
        TokenRevokeInfo,
        TokenRevokeWarning,
        TokenRevokeCritical,
        PasswordChangeInfo,
        PasswordChangeWarning,
        PasswordChangeCritical,
        RoleAssignInfo,
        RoleAssignWarning,
        RoleAssignCritical,
        RoleRevokeInfo,
        RoleRevokeWarning,
        RoleRevokeCritical,
        MfaEnableInfo,
        MfaEnableWarning,
        MfaEnableCritical,
        MfaDisableInfo,
        MfaDisableWarning,
        MfaDisableCritical,
        TenantJoinInfo,
        TenantJoinWarning,
        TenantJoinCritical,
        TenantLeaveInfo,
        TenantLeaveWarning,
        TenantLeaveCritical,
        AccountLockInfo,
        AccountLockWarning,
        AccountLockCritical,
        AccountUnlockInfo,
        AccountUnlockWarning,
        AccountUnlockCritical,
        ApiKeysGenerateInfo,
        ApiKeysGenerateWarning,
        ApiKeysGenerateCritical,
        ApiKeysRevokeInfo,
        ApiKeysRevokeWarning,
        ApiKeysRevokeCritical,
        Other(String),
    }

    impl std::fmt::Display for AuditEventCategory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                AuditEventCategory::LoginInfo => write!(f, "LoginInfo"),
                AuditEventCategory::LoginWarning => write!(f, "LoginWarning"),
                AuditEventCategory::LoginCritical => write!(f, "LoginCritical"),
                AuditEventCategory::LogoutInfo => write!(f, "LogoutInfo"),
                AuditEventCategory::LogoutWarning => write!(f, "LogoutWarning"),
                AuditEventCategory::LogoutCritical => write!(f, "LogoutCritical"),
                AuditEventCategory::TokenIssueInfo => write!(f, "TokenIssueInfo"),
                AuditEventCategory::TokenIssueWarning => write!(f, "TokenIssueWarning"),
                AuditEventCategory::TokenIssueCritical => write!(f, "TokenIssueCritical"),
                AuditEventCategory::TokenRevokeInfo => write!(f, "TokenRevokeInfo"),
                AuditEventCategory::TokenRevokeWarning => write!(f, "TokenRevokeWarning"),
                AuditEventCategory::TokenRevokeCritical => write!(f, "TokenRevokeCritical"),
                AuditEventCategory::PasswordChangeInfo => write!(f, "PasswordChangeInfo"),
                AuditEventCategory::PasswordChangeWarning => write!(f, "PasswordChangeWarning"),
                AuditEventCategory::PasswordChangeCritical => write!(f, "PasswordChangeCritical"),
                AuditEventCategory::RoleAssignInfo => write!(f, "RoleAssignInfo"),
                AuditEventCategory::RoleAssignWarning => write!(f, "RoleAssignWarning"),
                AuditEventCategory::RoleAssignCritical => write!(f, "RoleAssignCritical"),
                AuditEventCategory::RoleRevokeInfo => write!(f, "RoleRevokeInfo"),
                AuditEventCategory::RoleRevokeWarning => write!(f, "RoleRevokeWarning"),
                AuditEventCategory::RoleRevokeCritical => write!(f, "RoleRevokeCritical"),
                AuditEventCategory::MfaEnableInfo => write!(f, "MfaEnableInfo"),
                AuditEventCategory::MfaEnableWarning => write!(f, "MfaEnableWarning"),
                AuditEventCategory::MfaEnableCritical => write!(f, "MfaEnableCritical"),
                AuditEventCategory::MfaDisableInfo => write!(f, "MfaDisableInfo"),
                AuditEventCategory::MfaDisableWarning => write!(f, "MfaDisableWarning"),
                AuditEventCategory::MfaDisableCritical => write!(f, "MfaDisableCritical"),
                AuditEventCategory::TenantJoinInfo => write!(f, "TenantJoinInfo"),
                AuditEventCategory::TenantJoinWarning => write!(f, "TenantJoinWarning"),
                AuditEventCategory::TenantJoinCritical => write!(f, "TenantJoinCritical"),
                AuditEventCategory::TenantLeaveInfo => write!(f, "TenantLeaveInfo"),
                AuditEventCategory::TenantLeaveWarning => write!(f, "TenantLeaveWarning"),
                AuditEventCategory::TenantLeaveCritical => write!(f, "TenantLeaveCritical"),
                AuditEventCategory::AccountLockInfo => write!(f, "AccountLockInfo"),
                AuditEventCategory::AccountLockWarning => write!(f, "AccountLockWarning"),
                AuditEventCategory::AccountLockCritical => write!(f, "AccountLockCritical"),
                AuditEventCategory::AccountUnlockInfo => write!(f, "AccountUnlockInfo"),
                AuditEventCategory::AccountUnlockWarning => write!(f, "AccountUnlockWarning"),
                AuditEventCategory::AccountUnlockCritical => write!(f, "AccountUnlockCritical"),
                AuditEventCategory::ApiKeysGenerateInfo => write!(f, "ApiKeysGenerateInfo"),
                AuditEventCategory::ApiKeysGenerateWarning => write!(f, "ApiKeysGenerateWarning"),
                AuditEventCategory::ApiKeysGenerateCritical => write!(f, "ApiKeysGenerateCritical"),
                AuditEventCategory::ApiKeysRevokeInfo => write!(f, "ApiKeysRevokeInfo"),
                AuditEventCategory::ApiKeysRevokeWarning => write!(f, "ApiKeysRevokeWarning"),
                AuditEventCategory::ApiKeysRevokeCritical => write!(f, "ApiKeysRevokeCritical"),
                AuditEventCategory::Other(s) => write!(f, "{}", s),
            }
        }
    }

    impl AuditEventCategory {
        pub fn is_alertable(&self) -> bool {
            match self {
                AuditEventCategory::LoginInfo => false,
                AuditEventCategory::LoginWarning => false,
                AuditEventCategory::LoginCritical => true,
                AuditEventCategory::LogoutInfo => false,
                AuditEventCategory::LogoutWarning => false,
                AuditEventCategory::LogoutCritical => true,
                AuditEventCategory::TokenIssueInfo => false,
                AuditEventCategory::TokenIssueWarning => false,
                AuditEventCategory::TokenIssueCritical => true,
                AuditEventCategory::TokenRevokeInfo => false,
                AuditEventCategory::TokenRevokeWarning => false,
                AuditEventCategory::TokenRevokeCritical => true,
                AuditEventCategory::PasswordChangeInfo => false,
                AuditEventCategory::PasswordChangeWarning => false,
                AuditEventCategory::PasswordChangeCritical => true,
                AuditEventCategory::RoleAssignInfo => true,
                AuditEventCategory::RoleAssignWarning => true,
                AuditEventCategory::RoleAssignCritical => true,
                AuditEventCategory::RoleRevokeInfo => false,
                AuditEventCategory::RoleRevokeWarning => false,
                AuditEventCategory::RoleRevokeCritical => true,
                AuditEventCategory::MfaEnableInfo => false,
                AuditEventCategory::MfaEnableWarning => false,
                AuditEventCategory::MfaEnableCritical => true,
                AuditEventCategory::MfaDisableInfo => false,
                AuditEventCategory::MfaDisableWarning => false,
                AuditEventCategory::MfaDisableCritical => true,
                AuditEventCategory::TenantJoinInfo => false,
                AuditEventCategory::TenantJoinWarning => false,
                AuditEventCategory::TenantJoinCritical => true,
                AuditEventCategory::TenantLeaveInfo => false,
                AuditEventCategory::TenantLeaveWarning => false,
                AuditEventCategory::TenantLeaveCritical => true,
                AuditEventCategory::AccountLockInfo => true,
                AuditEventCategory::AccountLockWarning => true,
                AuditEventCategory::AccountLockCritical => true,
                AuditEventCategory::AccountUnlockInfo => false,
                AuditEventCategory::AccountUnlockWarning => false,
                AuditEventCategory::AccountUnlockCritical => true,
                AuditEventCategory::ApiKeysGenerateInfo => false,
                AuditEventCategory::ApiKeysGenerateWarning => false,
                AuditEventCategory::ApiKeysGenerateCritical => true,
                AuditEventCategory::ApiKeysRevokeInfo => false,
                AuditEventCategory::ApiKeysRevokeWarning => false,
                AuditEventCategory::ApiKeysRevokeCritical => true,
                AuditEventCategory::Other(_) => false,
            }
        }
    }
}
