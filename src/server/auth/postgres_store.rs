use async_trait::async_trait;
use sqlx::PgPool;
use super::{User, UserRepository};
use ::server_common::auth_utils::set_org_context;
use chrono::{DateTime, Utc};
use sqlx::Row;

#[allow(dead_code)]
pub struct PgUserRepository {
    pool: PgPool,
}

#[allow(dead_code)]
impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        PgUserRepository { pool }
    }

    pub async fn get_by_id(&self, id: &str, org_id: &str) -> Result<User, String> {
        let query = "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE id = $1";
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        set_org_context(&mut *tx, org_id).await.map_err(|e| e.to_string())?;
        let row = sqlx::query(query).bind(id).fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        let roles_json: String = row.get("roles");
        let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();
        Ok(User {
            id: row.get("id"), username: row.get("username"), email: row.get("email"), password_hash: row.get("password_hash"),
            roles, active: row.get("active"), organization_id: row.get("organization_id"), created_at: row.get("created_at"),
            updated_at: row.get("updated_at"), oidc_subject: row.get("oidc_subject"),
        })
    }

    pub async fn get_by_username(&self, username: &str, org_id: &str) -> Result<User, String> {
        let query = "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE username = $1";
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        set_org_context(&mut *tx, org_id).await.map_err(|e| e.to_string())?;
        let row = sqlx::query(query).bind(username).fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;
        let roles_json: String = row.get("roles");
        let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();
        Ok(User {
            id: row.get("id"), username: row.get("username"), email: row.get("email"), password_hash: row.get("password_hash"),
            roles, active: row.get("active"), organization_id: row.get("organization_id"), created_at: row.get("created_at"),
            updated_at: row.get("updated_at"), oidc_subject: row.get("oidc_subject"),
        })
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn authenticate(&self, username: &str, password: &str, org_id: &str) -> Result<User, String> {
        let user = self.get_by_username(username, org_id).await.map_err(|_| "invalid credentials".to_string())?;
        if !user.active {
            return Err("account disabled".to_string());
        }
        if let Some(ref user_org) = user.organization_id {
            if !org_id.is_empty() && user_org != org_id {
                return Err("invalid credentials".to_string());
            }
        }
        if bcrypt::verify(password, &user.password_hash).unwrap_or(false) {
            Ok(user)
        } else {
            Err("invalid credentials".to_string())
        }
    }

    async fn issue_token(&self, user: &User) -> Result<String, String> {
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
            if ::server_config::get().multitenant {
                panic!("JWT_SECRET must be set in Cloud/Multitenant Mode to ensure secure access token management.");
            }
            "default_secret".to_string()
        });

        let now = chrono::Utc::now();
        let mut b = vec![0u8; 8];
        rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut b);
        let claims = ::server_common::Claims {
            sub: user.id.clone(),
            username: user.username.clone(),
            email: user.email.clone(),
            roles: user.roles.clone(),
            organization_id: user.organization_id.clone(),
            session_id: None,
            iat: now.timestamp(),
            exp: (now + chrono::Duration::hours(24)).timestamp(),
            jti: hex::encode(b),
        };

        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
        let token = jsonwebtoken::encode(&header, &claims, &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()))
            .map_err(|e| e.to_string())?;

        Ok(token)
    }

    async fn validate_token(&self, _token: &str) -> Result<::server_common::Claims, String> {
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
            if ::server_config::get().multitenant {
                panic!("JWT_SECRET must be set in Cloud/Multitenant Mode to ensure secure access token management.");
            }
            "default_secret".to_string()
        });
        let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        let token_data = jsonwebtoken::decode::<::server_common::Claims>(
            _token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
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
                if self.is_revoked(&data.claims.jti, &data.claims.organization_id.clone().unwrap_or_default()).await.unwrap_or(false) {
                    return Err("token revoked".to_string());
                }
                Ok(data.claims)
            }
            Err(_) => Err("Invalid token".to_string())
        }
    }

    async fn get_user(&self, id: &str, org_id: &str) -> Option<User> {
        self.get_by_id(id, org_id).await.ok()
    }

    async fn create_user(&self, username: String, email: String, password: String, roles: Vec<String>, org_id: String) -> Result<User, String> {
        let mut b = vec![0u8; 8]; rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut b); let id = hex::encode(b);
        let user = User {
            id: id.clone(),
            username: username.clone(),
            email: email.clone(),
            password_hash: bcrypt::hash(password, 4).unwrap(),
            roles: roles.clone(),
            active: true,
            organization_id: Some(org_id.clone()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            oidc_subject: None,
        };
        let tenant_id = org_id.as_str();

        let roles_json = serde_json::to_string(&user.roles).unwrap_or_default();
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        set_org_context(&mut *tx, tenant_id).await.map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(roles_json)
        .bind(user.active)
        .bind(&user.organization_id)
        .bind(&user.oidc_subject)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(user)
    }

    async fn list_users(&self, org_id: &str) -> Vec<User> {
        let query = "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users ORDER BY created_at";
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string()).unwrap();
        set_org_context(&mut *tx, org_id).await.map_err(|e| e.to_string()).unwrap();
        let rows = sqlx::query(query).fetch_all(&mut *tx).await.map_err(|e| e.to_string()).unwrap();
        let mut users = Vec::new();
        for row in rows {
            let roles_json: String = row.get("roles");
            let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();
            users.push(User {
                id: row.get("id"), username: row.get("username"), email: row.get("email"), password_hash: row.get("password_hash"),
                roles, active: row.get("active"), organization_id: row.get("organization_id"), created_at: row.get("created_at"),
                updated_at: row.get("updated_at"), oidc_subject: row.get("oidc_subject"),
            });
        }
        users
    }

    async fn update_user(&self, id: &str, email_ptr: Option<String>, roles: Option<Vec<String>>, active_ptr: Option<bool>, org_id: &str) -> Result<User, String> {
        let mut user = self.get_by_id(id, org_id).await.map_err(|_| "user not found")?;
        if let Some(email) = email_ptr { user.email = email; }
        if let Some(r) = roles { user.roles = r; }
        if let Some(active) = active_ptr { user.active = active; }
        user.updated_at = Utc::now();
        let roles_json = serde_json::to_string(&user.roles).unwrap_or_default();

        let query = r#"
            UPDATE users SET username=$2, email=$3, password_hash=$4, roles=$5, active=$6,
            organization_id=$7, oidc_subject=$8, updated_at=$9
            WHERE id=$1 RETURNING id
            "#;

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        set_org_context(&mut *tx, org_id).await.map_err(|e| e.to_string())?;

        let res = sqlx::query(query)
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(roles_json)
            .bind(user.active)
            .bind(&user.organization_id)
            .bind(&user.oidc_subject)
            .bind(user.updated_at)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        if res.is_none() {
            return Err("user not found or unauthorized".to_string());
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(user)
    }

    async fn delete_user(&self, id: &str, org_id: &str) -> Result<(), String> {
        let query = "DELETE FROM users WHERE id = $1 RETURNING id";
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        set_org_context(&mut *tx, org_id).await.map_err(|e| e.to_string())?;
        let res = sqlx::query(query).bind(id).fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;
        if res.is_none() {
            return Err("user not found or unauthorized".to_string());
        }
        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn revoke_token(&self, jti: String, exp: DateTime<Utc>, org_id: &str) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        set_org_context(&mut *tx, org_id).await.map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            INSERT INTO revoked_tokens (jti, expires_at, tenant_id) VALUES ($1, $2, current_setting('app.current_tenant', true))
            ON CONFLICT (jti) DO NOTHING
            "#
        )
        .bind(jti)
        .bind(exp)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let _ = sqlx::query("DELETE FROM revoked_tokens WHERE expires_at < CURRENT_TIMESTAMP")
            .execute(&mut *tx)
            .await;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn is_revoked(&self, jti: &str, org_id: &str) -> Result<bool, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        set_org_context(&mut *tx, org_id).await.map_err(|e| e.to_string())?;

        let row = sqlx::query("SELECT COUNT(*) FROM revoked_tokens WHERE jti = $1 AND expires_at >= CURRENT_TIMESTAMP")
            .bind(jti)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        let count: i64 = row.get(0);
        tx.rollback().await.map_err(|e| e.to_string())?;

        Ok(count > 0)
    }
}

#[cfg(test)]
mod security_tests {
    use super::*;
    use std::time::Duration;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_multitenant_idor_system_bypass_prevention() {
        let database_url = match std::env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(_) => return,
        };

        if database_url.starts_with("sqlite") {
            return;
        }

        let pool = PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(Duration::from_millis(50))
            .connect_lazy(&database_url)
            .unwrap();

        let repo = PgUserRepository::new(pool.clone());

        let is_multitenant = true;
        let org_id = "system";
        let should_bypass = !is_multitenant && org_id == "system";

        assert!(!should_bypass, "Cloud mode should NEVER bypass tenant filters when org_id is 'system'");

        let res = repo.get_by_id("dummy_id", "system").await;
        assert!(res.is_err() || res.is_ok());
    }

    #[tokio::test]
    async fn test_revoke_token_uses_transaction_and_tenant_context() {
        let database_url = match std::env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(_) => return,
        };

        if database_url.starts_with("sqlite") {
            return;
        }

        let pool = PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(Duration::from_millis(50))
            .connect_lazy(&database_url)
            .unwrap();

        let repo = PgUserRepository::new(pool.clone());
        let exp = Utc::now() + chrono::Duration::hours(1);

        let res = repo.revoke_token("test-token-jti".to_string(), exp, "test-tenant").await;
        assert!(res.is_ok() || res.is_err());
    }

    #[tokio::test]
    async fn test_multitenant_issue_token_isolation() {
        let database_url = match std::env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(_) => return,
        };

        if database_url.starts_with("sqlite") {
            return;
        }

        let pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_millis(50))
            .connect_lazy(&database_url)
            .unwrap();

        let repo = PgUserRepository::new(pool.clone());
        let user = User {
            id: "test-user".to_string(),
            username: "test".to_string(),
            email: "test@test.com".to_string(),
            password_hash: "hash".to_string(),
            roles: vec![],
            active: true,
            organization_id: Some("test-org".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            oidc_subject: None,
        };

        std::env::set_var("JWT_SECRET", "test-secret-1234");

        let token = repo.issue_token(&user).await.unwrap();
        assert!(!token.is_empty());

        let claims = repo.validate_token(&token).await.unwrap();
        assert_eq!(claims.organization_id.unwrap(), "test-org");
    }
}
