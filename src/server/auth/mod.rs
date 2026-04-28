use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::RwLock;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use bcrypt::{hash, verify, DEFAULT_COST};
use rand::RngCore;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use tonic::{Request, Response, Status};

pub mod postgres_store;
pub mod grpc;
pub mod orchestration;
use crate::ohc::orchestration::auth_service_server::AuthService;
use crate::ohc::orchestration::*;

pub const ROLE_ADMIN: &str = "admin";
pub const ROLE_OPERATOR: &str = "operator";
pub const ROLE_VIEWER: &str = "viewer";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub roles: Vec<String>,
    pub active: bool,
    pub tenant_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub oidc_subject: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserPublic {
    pub id: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub active: bool,
    pub tenant_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub oidc_subject: Option<String>,
}

impl User {
    pub fn public_view(&self) -> UserPublic {
        UserPublic {
            id: self.id.clone(),
            username: self.username.clone(),
            email: self.email.clone(),
            roles: self.roles.clone(),
            active: self.active,
            tenant_id: self.tenant_id.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            oidc_subject: self.oidc_subject.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
    pub tenant_id: Option<String>,
    pub session_id: Option<String>,
    pub iat: i64,
    pub exp: i64,
    pub jti: String,
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
    pub tenant_id: String,
    pub key: String,
}

#[derive(Debug, Clone)]
pub struct OIDCConfig {
    pub issuer_url: String,
    pub client_id: String,
    pub enabled: bool,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: User, tenant_id: &str) -> Result<(), String>;
    async fn get_by_id(&self, id: &str, tenant_id: &str) -> Result<User, String>;
    async fn get_by_username(&self, username: &str, tenant_id: &str) -> Result<User, String>;
    async fn get_by_email(&self, email: &str, tenant_id: &str) -> Result<User, String>;
    async fn get_by_oidc_subject(&self, sub: &str, tenant_id: &str) -> Result<User, String>;
    async fn list_users(&self, tenant_id: &str) -> Result<Vec<User>, String>;
    async fn update_user(&self, user: User, tenant_id: &str) -> Result<(), String>;
    async fn delete_user(&self, id: &str, tenant_id: &str) -> Result<(), String>;
    async fn revoke_token(&self, jti: String, exp: DateTime<Utc>) -> Result<(), String>;
    async fn is_revoked(&self, jti: &str) -> Result<bool, String>;
}

pub struct Store {
    users: RwLock<HashMap<String, User>>,
    roles: RwLock<HashMap<String, Role>>,
    by_name: RwLock<HashMap<TenantKey, String>>, // key -> user_id
    by_email: RwLock<HashMap<TenantKey, String>>, // key -> user_id
    by_oidc: RwLock<HashMap<TenantKey, String>>,  // key -> user_id
    revoked: RwLock<HashMap<String, DateTime<Utc>>>, // jti -> expiry
    secret: Vec<u8>,
    oidc_cfg: RwLock<OIDCConfig>,
}

impl Store {
    pub fn new() -> Self {
        let secret = std::env::var("JWT_SECRET")
            .map(|s| s.into_bytes())
            .unwrap_or_else(|_| {
                println!("warning: falling back to random JWT secret; this is only suitable for single-node or standalone deployments");
                random_bytes(32)
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

        let hash = hash(admin_pass, DEFAULT_COST).expect("Failed to hash password");

        let id = hex::encode(random_bytes(8));
        
        let admin = User {
            id: id.clone(),
            username: admin_user.clone(),
            email: admin_email.clone(),
            password_hash: hash,
            roles: vec![ROLE_ADMIN.to_string()],
            active: true,
            tenant_id: None,
            created_at: now,
            updated_at: now,
            oidc_subject: None,
        };

        self.users.write().unwrap().insert(id.clone(), admin);
        self.by_name.write().unwrap().insert(TenantKey { tenant_id: "".to_string(), key: admin_user }, id.clone());
        self.by_email.write().unwrap().insert(TenantKey { tenant_id: "".to_string(), key: admin_email }, id);
    }

    pub fn create_user(&self, username: String, email: String, password: String, roles: Vec<String>, tenant_id: String) -> Result<User, String> {
        if username.is_empty() {
            return Err("username is required".to_string());
        }
        if password.len() < 6 {
            return Err("password must be at least 6 characters".to_string());
        }

        let mut users = self.users.write().unwrap();
        let mut by_name = self.by_name.write().unwrap();
        let mut by_email = self.by_email.write().unwrap();

        let name_key = TenantKey { tenant_id: tenant_id.clone(), key: username.clone() };
        if by_name.contains_key(&name_key) {
            return Err("username already taken".to_string());
        }

        let email_key = TenantKey { tenant_id: tenant_id.clone(), key: email.clone() };
        if by_email.contains_key(&email_key) {
            return Err("email already registered".to_string());
        }

        let hash = hash(password, DEFAULT_COST).expect("Failed to hash password");

        let id = hex::encode(random_bytes(8));
        let now = Utc::now();

        let user = User {
            id: id.clone(),
            username,
            email,
            password_hash: hash,
            roles,
            active: true,
            tenant_id: Some(tenant_id),
            created_at: now,
            updated_at: now,
            oidc_subject: None,
        };

        users.insert(id.clone(), user.clone());
        by_name.insert(name_key, id.clone());
        by_email.insert(email_key, id);

        Ok(user)
    }

    pub fn authenticate(&self, username: &str, password: &str, tenant_id: &str) -> Result<User, String> {
        let by_name = self.by_name.read().unwrap();
        let users = self.users.read().unwrap();

        let name_key = TenantKey { tenant_id: tenant_id.to_string(), key: username.to_string() };
        let mut user_id_opt = by_name.get(&name_key).cloned();

        if user_id_opt.is_none() && (tenant_id == "sys" || tenant_id.is_empty()) {
            user_id_opt = by_name.get(&TenantKey { tenant_id: "".to_string(), key: username.to_string() }).cloned();
        }

        let user_id = user_id_opt.ok_or_else(|| "invalid credentials".to_string())?;
        let user = users.get(&user_id).ok_or_else(|| "invalid credentials".to_string())?;

        if !user.active {
            return Err("account disabled".to_string());
        }

        if let Some(ref user_org) = user.tenant_id {
            if !tenant_id.is_empty() && tenant_id != "sys" && user_org != tenant_id {
                return Err("invalid credentials".to_string());
            }
        }

        if verify(password, &user.password_hash).unwrap_or(false) {
            Ok(user.clone())
        } else {
            Err("invalid credentials".to_string())
        }
    }

    pub fn get_user(&self, id: &str, tenant_id: &str) -> Option<User> {
        let users = self.users.read().unwrap();
        let u = users.get(id)?;
        
        if !tenant_id.is_empty() && tenant_id != "sys" {
            if let Some(ref user_org) = u.tenant_id {
                if user_org != tenant_id {
                    return None;
                }
            } else {
                return None;
            }
        }
        Some(u.clone())
    }

    pub fn list_users(&self, tenant_id: &str) -> Vec<User> {
        let users = self.users.read().unwrap();
        users.values()
            .filter(|u| {
                tenant_id.is_empty() || tenant_id == "sys" || u.tenant_id.as_deref() == Some(tenant_id)
            })
            .cloned()
            .collect()
    }

    pub fn update_user(&self, id: &str, email_ptr: Option<String>, roles: Option<Vec<String>>, active_ptr: Option<bool>, tenant_id: &str) -> Result<User, String> {
        let mut users = self.users.write().unwrap();
        let mut by_email = self.by_email.write().unwrap();

        let u = users.get_mut(id).ok_or_else(|| "user not found".to_string())?;

        if !tenant_id.is_empty() && tenant_id != "sys" {
             if u.tenant_id.as_deref() != Some(tenant_id) {
                 return Err("user not found".to_string());
             }
        }

        if let Some(email) = email_ptr {
            if email != u.email {
                let org = u.tenant_id.clone().unwrap_or_default();
                let email_key = TenantKey { tenant_id: org, key: email.clone() };
                if by_email.contains_key(&email_key) {
                    return Err("email already registered".to_string());
                }
                by_email.remove(&TenantKey { tenant_id: u.tenant_id.clone().unwrap_or_default(), key: u.email.clone() });
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

    pub fn delete_user(&self, id: &str, tenant_id: &str) -> Result<(), String> {
        let mut users = self.users.write().unwrap();
        let mut by_name = self.by_name.write().unwrap();
        let mut by_email = self.by_email.write().unwrap();
        let mut by_oidc = self.by_oidc.write().unwrap();

        let u = users.get(id).ok_or_else(|| "user not found".to_string())?;

        if !tenant_id.is_empty() && tenant_id != "sys" {
             if u.tenant_id.as_deref() != Some(tenant_id) {
                 return Err("user not found".to_string());
             }
        }

        let org = u.tenant_id.clone().unwrap_or_default();
        by_name.remove(&TenantKey { tenant_id: org.clone(), key: u.username.clone() });
        by_email.remove(&TenantKey { tenant_id: org.clone(), key: u.email.clone() });
        if let Some(ref oidc) = u.oidc_subject {
            by_oidc.remove(&TenantKey { tenant_id: org, key: oidc.clone() });
        }

        users.remove(id);

        Ok(())
    }

    pub fn revoke_token(&self, jti: String, exp: DateTime<Utc>) {
        let mut revoked = self.revoked.write().unwrap();
        revoked.insert(jti, exp);
        
        let now = Utc::now();
        revoked.retain(|_, v| *v > now);
    }

    pub fn is_revoked(&self, jti: &str) -> bool {
        let revoked = self.revoked.read().unwrap();
        if let Some(exp) = revoked.get(jti) {
             if exp > &Utc::now() {
                 return true;
             }
        }
        false
    }

    pub fn issue_token(&self, user: &User) -> Result<String, String> {
        let now = Utc::now();
        let claims = Claims {
            sub: user.id.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
            roles: user.roles.clone(),
            tenant_id: user.tenant_id.clone(),
            session_id: None,
            iat: now.timestamp(),
            exp: (now + chrono::Duration::hours(24)).timestamp(),
            jti: hex::encode(random_bytes(8)),
        };

        let header = Header::new(Algorithm::HS256);
        let token = encode(&header, &claims, &EncodingKey::from_secret(&self.secret))
            .map_err(|e| e.to_string())?;
            
        Ok(token)
    }

    pub async fn validate_token(&self, token: &str) -> Result<Claims, String> {
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret),
            &validation
        );

        match token_data {
            Ok(data) => {
                if self.is_revoked(&data.claims.jti) {
                    return Err("token revoked".to_string());
                }
                Ok(data.claims)
            }
            Err(e) => {
                let oidc_cfg = self.oidc_cfg.read().unwrap();
                if oidc_cfg.enabled {
                     let cfg = crate::oidc::OIDCConfig {
                         issuer_url: oidc_cfg.issuer_url.clone(),
                         client_id: oidc_cfg.client_id.clone(),
                         enabled: oidc_cfg.enabled,
                     };
                     return crate::oidc::validate_oidc_token(token, &cfg).await;
                }
                Err(e.to_string())
            }
        }
    }
    pub fn get_or_create_oidc_user(&self, sub: &str, email: &str, preferred_username: &str, tenant_id: &str) -> User {
        let mut users = self.users.write().unwrap();
        let mut by_oidc = self.by_oidc.write().unwrap();
        let mut by_email = self.by_email.write().unwrap();
        let mut by_name = self.by_name.write().unwrap();

        let oidc_key = TenantKey { tenant_id: tenant_id.to_string(), key: sub.to_string() };
        if let Some(user_id) = by_oidc.get(&oidc_key) {
            if let Some(user) = users.get(user_id) {
                return user.clone();
            }
        }

        if !email.is_empty() {
            let email_key = TenantKey { tenant_id: tenant_id.to_string(), key: email.to_string() };
            if let Some(user_id) = by_email.get(&email_key) {
                if let Some(user) = users.get_mut(user_id) {
                    user.oidc_subject = Some(sub.to_string());
                    by_oidc.insert(oidc_key, user_id.clone());
                    return user.clone();
                }
            }
        }

        let mut uname = preferred_username.to_string();
        if uname.is_empty() {
            uname = email.to_string();
        }
        // de-duplicate username
        let name_key = TenantKey { tenant_id: tenant_id.to_string(), key: uname.clone() };
        if by_name.contains_key(&name_key) {
             uname = format!("{}_{}", uname, hex::encode(random_bytes(3)));
        }

        let now = Utc::now();
        let id = hex::encode(random_bytes(8));
        let user = User {
            id: id.clone(),
            username: uname.clone(),
            email: email.to_string(),
            password_hash: String::new(), // No password for OIDC users
            roles: vec![ROLE_VIEWER.to_string()],
            active: true,
            tenant_id: Some(tenant_id.to_string()),
            oidc_subject: Some(sub.to_string()),
            created_at: now,
            updated_at: now,
        };

        users.insert(id.clone(), user.clone());
        if !uname.is_empty() {
            by_name.insert(TenantKey { tenant_id: tenant_id.to_string(), key: uname }, id.clone());
        }
        if !email.is_empty() {
            by_email.insert(TenantKey { tenant_id: tenant_id.to_string(), key: email.to_string() }, id.clone());
        }
        by_oidc.insert(oidc_key, id);

        user
    }
}

fn random_bytes(n: usize) -> Vec<u8> {
    let mut b = vec![0u8; n];
    rand::thread_rng().fill_bytes(&mut b);
    b
}

use std::sync::Arc;

#[tonic::async_trait]
impl AuthService for Arc<Store> {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        
        match self.as_ref().authenticate(&req.username, &req.password, &req.tenant_id) {
            Ok(user) => {
                match self.as_ref().issue_token(&user) {
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
        let roles = if req.roles.is_empty() {
            vec![ROLE_ADMIN.to_string()]
        } else {
            req.roles
        };
        
        match self.as_ref().create_user(req.username, req.email, req.password, roles, req.tenant_id) {
            Ok(user) => {
                match self.as_ref().issue_token(&user) {
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
            Err(e) => Err(Status::already_exists(e)),
        }
    }

    async fn logout(&self, _request: Request<EmptyRequest>) -> Result<Response<EmptyResponse>, Status> {
        // TODO: Implement token revocation in gRPC
        Ok(Response::new(EmptyResponse {}))
    }

    async fn get_me(&self, _request: Request<EmptyRequest>) -> Result<Response<UserProto>, Status> {
        // TODO: Implement after adding JWT interceptor to extract user claims
        Err(Status::unimplemented("get_me requires JWT authentication interceptor"))
    }

    async fn list_users(&self, request: Request<ListUsersRequest>) -> Result<Response<ListUsersResponse>, Status> {
        let req = request.into_inner();
        let users = self.as_ref().list_users(&req.tenant_id);
        let proto_users = users.into_iter().map(|u| UserProto {
            id: u.id,
            username: u.username,
            email: u.email,
            roles: u.roles,
            active: u.active,
            tenant_id: u.tenant_id.unwrap_or_default(),
            created_at_unix: u.created_at.timestamp(),
            updated_at_unix: u.updated_at.timestamp(),
            oidc_subject: u.oidc_subject.unwrap_or_default(),
        }).collect();
        
        Ok(Response::new(ListUsersResponse { users: proto_users }))
    }

    async fn create_user(&self, request: Request<CreateUserRequest>) -> Result<Response<UserProto>, Status> {
        let req = request.into_inner();
        match self.as_ref().create_user(req.username, req.email, req.password, req.roles, req.tenant_id) {
            Ok(u) => Ok(Response::new(UserProto {
                id: u.id,
                username: u.username,
                email: u.email,
                roles: u.roles,
                active: u.active,
                tenant_id: u.tenant_id.unwrap_or_default(),
                created_at_unix: u.created_at.timestamp(),
                updated_at_unix: u.updated_at.timestamp(),
                oidc_subject: u.oidc_subject.unwrap_or_default(),
            })),
            Err(e) => Err(Status::already_exists(e)),
        }
    }

    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<UserProto>, Status> {
        let req = request.into_inner();
        match self.as_ref().get_user(&req.id, &req.tenant_id) {
            Some(u) => Ok(Response::new(UserProto {
                id: u.id,
                username: u.username,
                email: u.email,
                roles: u.roles,
                active: u.active,
                tenant_id: u.tenant_id.unwrap_or_default(),
                created_at_unix: u.created_at.timestamp(),
                updated_at_unix: u.updated_at.timestamp(),
                oidc_subject: u.oidc_subject.unwrap_or_default(),
            })),
            None => Err(Status::not_found("user not found")),
        }
    }

    async fn update_user(&self, request: Request<UpdateUserRequest>) -> Result<Response<UserProto>, Status> {
        let req = request.into_inner();
        match self.as_ref().update_user(&req.id, req.email, Some(req.roles), req.active, &req.tenant_id) {
            Ok(u) => Ok(Response::new(UserProto {
                id: u.id,
                username: u.username,
                email: u.email,
                roles: u.roles,
                active: u.active,
                tenant_id: u.tenant_id.unwrap_or_default(),
                created_at_unix: u.created_at.timestamp(),
                updated_at_unix: u.updated_at.timestamp(),
                oidc_subject: u.oidc_subject.unwrap_or_default(),
            })),
            Err(e) => Err(Status::internal(e)),
        }
    }

    async fn delete_user(&self, request: Request<DeleteUserRequest>) -> Result<Response<EmptyResponse>, Status> {
        let req = request.into_inner();
        match self.as_ref().delete_user(&req.id, &req.tenant_id) {
            Ok(_) => Ok(Response::new(EmptyResponse {})),
            Err(e) => Err(Status::not_found(e)),
        }
    }

    async fn list_roles(&self, _request: Request<EmptyRequest>) -> Result<Response<ListRolesResponse>, Status> {
        let roles = self.roles.read().unwrap();
        let proto_roles = roles.values().map(|r| RoleProto {
            id: r.id.clone(),
            name: r.name.clone(),
            permissions: r.permissions.clone(),
            created_at_unix: r.created_at.timestamp(),
        }).collect();
        
        Ok(Response::new(ListRolesResponse { roles: proto_roles }))
    }

    async fn create_role(&self, request: Request<CreateRoleRequest>) -> Result<Response<RoleProto>, Status> {
        let req = request.into_inner();
        if req.name.is_empty() {
            return Err(Status::invalid_argument("role name is required"));
        }
        
        let mut roles = self.roles.write().unwrap();
        if roles.contains_key(&req.name) {
            return Err(Status::already_exists(format!("role {} already exists", req.name)));
        }
        
        let r = Role {
            id: req.name.clone(),
            name: req.name.clone(),
            permissions: req.permissions.clone(),
            created_at: Utc::now(),
        };
        
        roles.insert(req.name.clone(), r.clone());
        
        Ok(Response::new(RoleProto {
            id: r.id,
            name: r.name,
            permissions: r.permissions,
            created_at_unix: r.created_at.timestamp(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_store_admin_user_created() {
        std::env::set_var("ADMIN_USERNAME", "testadmin");
        std::env::set_var("ADMIN_PASSWORD", "secret99");
        std::env::set_var("ADMIN_EMAIL", "testadmin@test.com");
        
        let s = Store::new();
        let users = s.list_users("");
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].username, "testadmin");
    }

    #[test]
    fn test_store_create_and_authenticate() {
        let s = Store::new();
        let u = s.create_user("alice".to_string(), "alice@test.com".to_string(), "hunter2!".to_string(), vec![ROLE_VIEWER.to_string()], "".to_string()).unwrap();
        
        let got = s.authenticate("alice", "hunter2!", "").unwrap();
        assert_eq!(got.id, u.id);
        
        assert!(s.authenticate("alice", "wrongpass", "").is_err());
        assert!(s.authenticate("nobody", "x", "").is_err());
    }

    #[test]
    fn test_store_duplicate_username() {
        let s = Store::new();
        s.create_user("bob".to_string(), "bob@test.com".to_string(), "pass123".to_string(), vec![], "".to_string()).unwrap();
        assert!(s.create_user("bob".to_string(), "bob2@test.com".to_string(), "pass123".to_string(), vec![], "".to_string()).is_err());
    }

    #[test]
    fn test_store_short_password_rejected() {
        let s = Store::new();
        assert!(s.create_user("short".to_string(), "short@test.com".to_string(), "abc".to_string(), vec![], "".to_string()).is_err());
    }

    #[test]
    fn test_store_update_and_delete_user() {
        let s = Store::new();
        let u = s.create_user("charlie".to_string(), "c@test.com".to_string(), "p@ssw0rd".to_string(), vec![ROLE_VIEWER.to_string()], "".to_string()).unwrap();
        
        let new_email = "charlie2@test.com".to_string();
        let active = false;
        let updated = s.update_user(&u.id, Some(new_email.clone()), Some(vec![ROLE_OPERATOR.to_string()]), Some(active), "").unwrap();
        
        assert_eq!(updated.email, new_email);
        assert_eq!(updated.active, active);
        
        s.delete_user(&u.id, "").unwrap();
        assert!(s.get_user(&u.id, "").is_none());
    }

    #[tokio::test]
    async fn test_jwt_round_trip() {
        let s = Store::new();
        let u = s.create_user("jwt-user".to_string(), "jwt@test.com".to_string(), "jwtpass1".to_string(), vec![ROLE_OPERATOR.to_string()], "".to_string()).unwrap();
        
        let token = s.issue_token(&u).unwrap();
        assert!(!token.is_empty());
        
        let claims = s.validate_token(&token).await.unwrap();
        assert_eq!(claims.sub, u.id);
        assert_eq!(claims.username, "jwt-user");
    }

    #[tokio::test]
    async fn test_jwt_revoked_token() {
        let s = Store::new();
        let u = s.create_user("revoke-me".to_string(), "revoke@test.com".to_string(), "revpass1".to_string(), vec![], "".to_string()).unwrap();
        let token = s.issue_token(&u).unwrap();
        
        let claims = s.validate_token(&token).await.unwrap();
        s.revoke_token(claims.jti.clone(), Utc::now() + chrono::Duration::hours(24));
        
        assert!(s.validate_token(&token).await.is_err());
    }

    #[test]
    fn test_parse_spiffe_id() {
        let (org, agent) = parse_spiffe_id("spiffe://onehumancorp.io/org-1/agent-1").unwrap();
        assert_eq!(org, "org-1");
        assert_eq!(agent, "agent-1");

        let (org, agent) = parse_spiffe_id("spiffe://ohc.local/org/org-2/agent/agent-2").unwrap();
        assert_eq!(org, "org-2");
        assert_eq!(agent, "agent-2");

        let (org, agent) = parse_spiffe_id("spiffe://ohc.os/agent/agent-3").unwrap();
        assert_eq!(org, "");
        assert_eq!(agent, "agent-3");

        let (org, agent) = parse_spiffe_id("spiffe://us-east.ohc.global/org/org-4/agent/agent-4").unwrap();
        assert_eq!(org, "org-4");
        assert_eq!(agent, "agent-4");

        assert!(parse_spiffe_id("invalid").is_err());
        assert!(parse_spiffe_id("spiffe://invalid.com/x").is_err());
    }
    #[tokio::test]
    async fn test_auth_service_login_valid() {
        let s = Arc::new(Store::new());
        let req = Request::new(LoginRequest {
            username: "admin".to_string(),
            password: "admin".to_string(),
            tenant_id: "".to_string(),
        });
        
        let resp = s.login(req).await.unwrap();
        let resp = resp.into_inner();
        assert!(!resp.token.is_empty());
    }

    #[tokio::test]
    async fn test_auth_service_register_valid() {
        let s = Arc::new(Store::new());
        let req = Request::new(CreateUserRequest {
            username: "newuser".to_string(),
            email: "new@test.com".to_string(),
            password: "password123".to_string(),
            roles: vec![],
            tenant_id: "".to_string(),
        });
        
        let resp = s.register(req).await.unwrap();
        let resp = resp.into_inner();
        assert!(!resp.token.is_empty());
    }

    #[tokio::test]
    async fn test_auth_service_list_users() {
        let s = Arc::new(Store::new());
        let req = Request::new(ListUsersRequest {
            tenant_id: "".to_string(),
        });
        
        let resp = s.list_users(req).await.unwrap();
        let resp = resp.into_inner();
        assert_eq!(resp.users.len(), 1);
        assert_eq!(resp.users[0].username, "admin");
    }

    #[tokio::test]
    async fn test_auth_service_create_role() {
        let s = Arc::new(Store::new());
        let req = Request::new(CreateRoleRequest {
            name: "new_role".to_string(),
            permissions: vec!["read".to_string()],
        });
        
        let resp = s.create_role(req).await.unwrap();
        let resp = resp.into_inner();
        assert_eq!(resp.name, "new_role");
        assert_eq!(resp.permissions, vec!["read".to_string()]);
    }
}

pub fn parse_spiffe_id(spiffe_id: &str) -> Result<(String, String), String> {
    if !spiffe_id.starts_with("spiffe://") {
        return Err(format!("invalid SPIFFE ID format: {}", spiffe_id));
    }
    
    let trimmed = &spiffe_id["spiffe://".len()..];
    if trimmed.contains("..") || trimmed.contains("//") {
        return Err(format!("invalid SPIFFE ID format: {}", spiffe_id));
    }
    
    let parts: Vec<&str> = trimmed.split('/').collect();
    if parts.len() < 2 {
        return Err(format!("SPIFFE ID lacks required path segments for agent identity: {}", spiffe_id));
    }
    
    let domain = parts[0];
    let agent_id: String;
    let tenant_id: String;
    
    if domain == "onehumancorp.io" {
        if parts.len() != 3 {
            return Err(format!("invalid SPIFFE ID path structure for domain onehumancorp.io: {}", spiffe_id));
        }
        tenant_id = parts[1].to_string();
        agent_id = parts[2].to_string();
    } else if domain == "ohc.local" {
        if parts.len() != 5 || parts[1] != "org" || parts[3] != "agent" {
            return Err(format!("invalid SPIFFE ID path structure for domain ohc.local: {}", spiffe_id));
        }
        tenant_id = parts[2].to_string();
        agent_id = parts[4].to_string();
    } else if domain == "ohc.os" {
        if parts.len() != 3 || parts[1] != "agent" {
            return Err(format!("invalid SPIFFE ID path structure for domain ohc.os: {}", spiffe_id));
        }
        tenant_id = String::new();
        agent_id = parts[2].to_string();
    } else if domain == "ohc.global" || domain.ends_with(".ohc.global") {
        if parts.len() != 5 || parts[1] != "org" || parts[3] != "agent" {
            return Err(format!("invalid SPIFFE ID path structure for domain {}: {}", domain, spiffe_id));
        }
        tenant_id = parts[2].to_string();
        agent_id = parts[4].to_string();
    } else {
        return Err(format!("unsupported SPIFFE trust domain in ID: {}", spiffe_id));
    }
    
    Ok((tenant_id, agent_id))
}
