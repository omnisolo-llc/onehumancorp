use async_trait::async_trait;
use sqlx::SqlitePool;
use crate::auth::{User, UserRepository};
use chrono::{DateTime, Utc};
use sqlx::Row;

pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        SqliteUserRepository { pool }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn create_user(&self, user: User, _org_id: &str) -> Result<(), String> {
        let roles_json = serde_json::to_string(&user.roles).unwrap_or_default();

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
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_by_id(&self, id: &str, _org_id: &str) -> Result<User, String> {
        let row = sqlx::query(
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

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

    async fn get_by_username(&self, username: &str, _org_id: &str) -> Result<User, String> {
        let row = sqlx::query(
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE username = $1"
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

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

    async fn get_by_email(&self, email: &str, _org_id: &str) -> Result<User, String> {
        let row = sqlx::query(
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

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

    async fn get_by_oidc_subject(&self, sub: &str, _org_id: &str) -> Result<User, String> {
        let row = sqlx::query(
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE oidc_subject = $1"
        )
        .bind(sub)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

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

    async fn list_users(&self, _org_id: &str) -> Result<Vec<User>, String> {
        let rows = sqlx::query(
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users ORDER BY created_at"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

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

    async fn update_user(&self, user: User, _org_id: &str) -> Result<(), String> {
        let roles_json = serde_json::to_string(&user.roles).unwrap_or_default();

        let res = sqlx::query(
            r#"
            UPDATE users SET username=$2, email=$3, password_hash=$4, roles=$5, active=$6,
            organization_id=$7, oidc_subject=$8, updated_at=$9
            WHERE id=$1 RETURNING id
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
        .bind(user.updated_at)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if res.is_none() {
            return Err("user not found or unauthorized".to_string());
        }

        Ok(())
    }

    async fn delete_user(&self, id: &str, _org_id: &str) -> Result<(), String> {
        let res = sqlx::query("DELETE FROM users WHERE id = $1 RETURNING id")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if res.is_none() {
            return Err("user not found or unauthorized".to_string());
        }

        Ok(())
    }

    async fn revoke_token(&self, jti: String, exp: DateTime<Utc>, _org_id: &str) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO revoked_tokens (jti, expires_at) VALUES ($1, $2)
            ON CONFLICT (jti) DO NOTHING
            "#
        )
        .bind(jti)
        .bind(exp)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        // GC expired entries
        let _ = sqlx::query("DELETE FROM revoked_tokens WHERE expires_at < CURRENT_TIMESTAMP")
            .execute(&self.pool)
            .await;

        Ok(())
    }

    async fn is_revoked(&self, jti: &str, _org_id: &str) -> Result<bool, String> {
        let row = sqlx::query("SELECT COUNT(*) FROM revoked_tokens WHERE jti = $1 AND expires_at >= CURRENT_TIMESTAMP")
            .bind(jti)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let count: i64 = row.get(0);
        Ok(count > 0)
    }
}
