use async_trait::async_trait;
use sqlx::SqlitePool;
use crate::auth::{User, UserRepository};
use chrono::{DateTime, Utc};
use sqlx::Row;

#[allow(dead_code)]
pub struct SqliteUserRepository {
    pool: SqlitePool,
    secret: Vec<u8>,
}

#[allow(dead_code)]
impl SqliteUserRepository {
    pub fn new(pool: SqlitePool, secret: Vec<u8>) -> Self {
        SqliteUserRepository { pool, secret }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn create_user(&self, user: User, org_id: &str) -> Result<(), String> {
        let roles_json = serde_json::to_string(&user.roles).unwrap_or_default();

        let query = "INSERT INTO users (id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        sqlx::query(query)
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
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_by_id(&self, id: &str, org_id: &str) -> Result<User, String> {
        let query = if org_id.is_empty() {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE id = ?"
        } else {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE id = ? AND (organization_id = ? OR organization_id IS NULL OR organization_id = '')"
        };

        let row = if org_id.is_empty() {
            sqlx::query(query).bind(id).fetch_one(&self.pool).await
        } else {
            sqlx::query(query).bind(id).bind(org_id).fetch_one(&self.pool).await
        }.map_err(|e| e.to_string())?;

        let roles_json: String = row.get("roles");
        let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            roles,
            active: row.get("active"),
            organization_id: row.get("organization_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            oidc_subject: row.get("oidc_subject"),
        })
    }

    async fn get_by_username(&self, username: &str, org_id: &str) -> Result<User, String> {
        let query = if org_id.is_empty() {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE username = ?"
        } else {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE username = ? AND (organization_id = ? OR organization_id IS NULL OR organization_id = '')"
        };

        let row = if org_id.is_empty() {
            sqlx::query(query).bind(username).fetch_one(&self.pool).await
        } else {
            sqlx::query(query).bind(username).bind(org_id).fetch_one(&self.pool).await
        }.map_err(|e| e.to_string())?;

        let roles_json: String = row.get("roles");
        let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            roles,
            active: row.get("active"),
            organization_id: row.get("organization_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            oidc_subject: row.get("oidc_subject"),
        })
    }

    async fn get_by_email(&self, email: &str, org_id: &str) -> Result<User, String> {
        let query = if org_id.is_empty() {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE email = ?"
        } else {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE email = ? AND (organization_id = ? OR organization_id IS NULL OR organization_id = '')"
        };

        let row = if org_id.is_empty() {
            sqlx::query(query).bind(email).fetch_one(&self.pool).await
        } else {
            sqlx::query(query).bind(email).bind(org_id).fetch_one(&self.pool).await
        }.map_err(|e| e.to_string())?;

        let roles_json: String = row.get("roles");
        let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            roles,
            active: row.get("active"),
            organization_id: row.get("organization_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            oidc_subject: row.get("oidc_subject"),
        })
    }

    async fn get_by_oidc_subject(&self, sub: &str, org_id: &str) -> Result<User, String> {
        let query = if org_id.is_empty() {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE oidc_subject = ?"
        } else {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE oidc_subject = ? AND (organization_id = ? OR organization_id IS NULL OR organization_id = '')"
        };

        let row = if org_id.is_empty() {
            sqlx::query(query).bind(sub).fetch_one(&self.pool).await
        } else {
            sqlx::query(query).bind(sub).bind(org_id).fetch_one(&self.pool).await
        }.map_err(|e| e.to_string())?;

        let roles_json: String = row.get("roles");
        let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            roles,
            active: row.get("active"),
            organization_id: row.get("organization_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            oidc_subject: row.get("oidc_subject"),
        })
    }

    async fn list_users(&self, org_id: &str) -> Result<Vec<User>, String> {
        let query = if org_id.is_empty() {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users ORDER BY created_at"
        } else {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE (organization_id = ? OR organization_id IS NULL OR organization_id = '') ORDER BY created_at"
        };

        let rows = if org_id.is_empty() {
            sqlx::query(query).fetch_all(&self.pool).await
        } else {
            sqlx::query(query).bind(org_id).fetch_all(&self.pool).await
        }.map_err(|e| e.to_string())?;

        let mut users = Vec::new();
        for row in rows {
            let roles_json: String = row.get("roles");
            let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

            users.push(User {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                roles,
                active: row.get("active"),
                organization_id: row.get("organization_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                oidc_subject: row.get("oidc_subject"),
            });
        }
        Ok(users)
    }

    async fn update_user(&self, user: User, org_id: &str) -> Result<(), String> {
        let roles_json = serde_json::to_string(&user.roles).unwrap_or_default();

        let query = if org_id.is_empty() {
            r#"
            UPDATE users SET username=?, email=?, password_hash=?, roles=?, active=?,
            organization_id=?, oidc_subject=?, updated_at=?
            WHERE id=?
            "#
        } else {
            r#"
            UPDATE users SET username=?, email=?, password_hash=?, roles=?, active=?,
            organization_id=?, oidc_subject=?, updated_at=?
            WHERE id=? AND (organization_id=? OR organization_id IS NULL OR organization_id = '')
            "#
        };

        let res = if org_id.is_empty() {
            sqlx::query(query)
                .bind(&user.username)
                .bind(&user.email)
                .bind(&user.password_hash)
                .bind(roles_json)
                .bind(user.active)
                .bind(&user.organization_id)
                .bind(&user.oidc_subject)
                .bind(user.updated_at)
                .bind(&user.id)
                .execute(&self.pool)
                .await
        } else {
            sqlx::query(query)
                .bind(&user.username)
                .bind(&user.email)
                .bind(&user.password_hash)
                .bind(roles_json)
                .bind(user.active)
                .bind(&user.organization_id)
                .bind(&user.oidc_subject)
                .bind(user.updated_at)
                .bind(&user.id)
                .bind(org_id)
                .execute(&self.pool)
                .await
        }.map_err(|e| e.to_string())?;

        if res.rows_affected() == 0 {
            return Err("user not found or unauthorized".to_string());
        }

        Ok(())
    }

    async fn delete_user(&self, id: &str, org_id: &str) -> Result<(), String> {
        let query = if org_id.is_empty() {
            "DELETE FROM users WHERE id = ?"
        } else {
            "DELETE FROM users WHERE id = ? AND (organization_id = ? OR organization_id IS NULL OR organization_id = '')"
        };

        let res = if org_id.is_empty() {
            sqlx::query(query).bind(id).execute(&self.pool).await
        } else {
            sqlx::query(query).bind(id).bind(org_id).execute(&self.pool).await
        }.map_err(|e| e.to_string())?;

        if res.rows_affected() == 0 {
            return Err("user not found or unauthorized".to_string());
        }

        Ok(())
    }

    async fn revoke_token(&self, jti: String, exp: DateTime<Utc>) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO revoked_tokens (jti, expires_at) VALUES (?, ?)
            ON CONFLICT (jti) DO NOTHING
            "#
        )
        .bind(jti)
        .bind(exp)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let _ = sqlx::query("DELETE FROM revoked_tokens WHERE expires_at < CURRENT_TIMESTAMP")
            .execute(&self.pool)
            .await;

        Ok(())
    }

    async fn is_revoked(&self, jti: &str) -> Result<bool, String> {
        let row = sqlx::query("SELECT COUNT(*) FROM revoked_tokens WHERE jti = ? AND expires_at >= CURRENT_TIMESTAMP")
            .bind(jti)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let count: i64 = row.get(0);
        Ok(count > 0)
    }

    async fn authenticate(&self, username: &str, password: &str, org_id: &str) -> Result<User, String> {
        let mut user_res = self.get_by_username(username, org_id).await;
        if user_res.is_err() && org_id.is_empty() {
             user_res = self.get_by_username(username, "").await;
        }
        let user = user_res?;
        if !user.active {
            return Err("account disabled".to_string());
        }
        if bcrypt::verify(password, &user.password_hash).unwrap_or(false) {
            Ok(user)
        } else {
            Err("invalid credentials".to_string())
        }
    }

    fn get_secret(&self) -> Vec<u8> {
        self.secret.clone()
    }

    async fn get_roles(&self) -> Result<Vec<crate::auth::Role>, String> {
        let now = chrono::Utc::now();
        Ok(vec![
            crate::auth::Role {
                id: crate::auth::ROLE_ADMIN.to_string(),
                name: crate::auth::ROLE_ADMIN.to_string(),
                permissions: vec!["*".to_string()],
                created_at: now,
            },
            crate::auth::Role {
                id: crate::auth::ROLE_OPERATOR.to_string(),
                name: crate::auth::ROLE_OPERATOR.to_string(),
                permissions: vec!["read".to_string(), "write".to_string()],
                created_at: now,
            },
            crate::auth::Role {
                id: crate::auth::ROLE_VIEWER.to_string(),
                name: crate::auth::ROLE_VIEWER.to_string(),
                permissions: vec!["read".to_string()],
                created_at: now,
            },
        ])
    }

    async fn create_role(&self, _role: crate::auth::Role) -> Result<(), String> {
        Err("Custom roles are not supported yet in the persistent store.".to_string())
    }
}
