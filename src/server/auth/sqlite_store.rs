use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use chrono::DateTime;
use chrono::Utc;
use crate::auth::{UserRepository, User};

pub struct SqliteUserRepository {
    pool: SqlitePool,
}

impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn create_user(&self, user: User, org_id: &str) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let roles_json = serde_json::to_string(&user.roles).unwrap_or_default();

        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&roles_json)
        .bind(user.active)
        .bind(org_id)
        .bind(&user.oidc_subject)
        .bind(user.created_at.to_rfc3339())
        .bind(user.updated_at.to_rfc3339())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_by_id(&self, id: &str, org_id: &str) -> Result<User, String> {
        let query = if org_id.is_empty() || (org_id == "system" && !crate::config::get().multitenant) {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE id = ?"
        } else {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE id = ? AND organization_id = ?"
        };

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let row = if org_id.is_empty() || (org_id == "system" && !crate::config::get().multitenant) {
            sqlx::query(query).bind(id).fetch_one(&mut *tx).await
        } else {
            sqlx::query(query).bind(id).bind(org_id).fetch_one(&mut *tx).await
        }.map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        let roles_json: String = row.get("roles");
        let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

        let created_at_str: String = row.get("created_at");
        let updated_at_str: String = row.get("updated_at");

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            roles,
            active: row.get("active"),
            organization_id: row.get("organization_id"),
            created_at: DateTime::parse_from_rfc3339(&created_at_str).unwrap_or_default().into(),
            updated_at: DateTime::parse_from_rfc3339(&updated_at_str).unwrap_or_default().into(),
            oidc_subject: row.get("oidc_subject"),
        })
    }

    async fn get_by_username(&self, username: &str, org_id: &str) -> Result<User, String> {
        let query = if org_id.is_empty() || (org_id == "system" && !crate::config::get().multitenant) {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE username = ?"
        } else {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE username = ? AND organization_id = ?"
        };

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let row = if org_id.is_empty() || (org_id == "system" && !crate::config::get().multitenant) {
            sqlx::query(query).bind(username).fetch_one(&mut *tx).await
        } else {
            sqlx::query(query).bind(username).bind(org_id).fetch_one(&mut *tx).await
        }.map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        let roles_json: String = row.get("roles");
        let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

        let created_at_str: String = row.get("created_at");
        let updated_at_str: String = row.get("updated_at");

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            roles,
            active: row.get("active"),
            organization_id: row.get("organization_id"),
            created_at: DateTime::parse_from_rfc3339(&created_at_str).unwrap_or_default().into(),
            updated_at: DateTime::parse_from_rfc3339(&updated_at_str).unwrap_or_default().into(),
            oidc_subject: row.get("oidc_subject"),
        })
    }

    async fn get_by_email(&self, email: &str, org_id: &str) -> Result<User, String> {
        let query = if org_id.is_empty() || (org_id == "system" && !crate::config::get().multitenant) {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE email = ?"
        } else {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE email = ? AND organization_id = ?"
        };

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let row = if org_id.is_empty() || (org_id == "system" && !crate::config::get().multitenant) {
            sqlx::query(query).bind(email).fetch_one(&mut *tx).await
        } else {
            sqlx::query(query).bind(email).bind(org_id).fetch_one(&mut *tx).await
        }.map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        let roles_json: String = row.get("roles");
        let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

        let created_at_str: String = row.get("created_at");
        let updated_at_str: String = row.get("updated_at");

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            roles,
            active: row.get("active"),
            organization_id: row.get("organization_id"),
            created_at: DateTime::parse_from_rfc3339(&created_at_str).unwrap_or_default().into(),
            updated_at: DateTime::parse_from_rfc3339(&updated_at_str).unwrap_or_default().into(),
            oidc_subject: row.get("oidc_subject"),
        })
    }

    async fn get_by_oidc_subject(&self, _sub: &str, _org_id: &str) -> Result<User, String> {
        Err("Not implemented".to_string())
    }

    async fn list_users(&self, org_id: &str) -> Result<Vec<User>, String> {
        let query = if org_id == "system" && !crate::config::get().multitenant {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users ORDER BY created_at"
        } else {
            "SELECT id, username, email, password_hash, roles, active, organization_id, oidc_subject, created_at, updated_at FROM users WHERE organization_id = ? ORDER BY created_at"
        };

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let rows = if org_id == "system" && !crate::config::get().multitenant {
            sqlx::query(query).fetch_all(&mut *tx).await
        } else {
            sqlx::query(query).bind(org_id).fetch_all(&mut *tx).await
        }.map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        let mut users = Vec::new();
        for row in rows {
            let roles_json: String = row.get("roles");
            let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

            let created_at_str: String = row.get("created_at");
            let updated_at_str: String = row.get("updated_at");

            users.push(User {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                roles,
                active: row.get("active"),
                organization_id: row.get("organization_id"),
                created_at: DateTime::parse_from_rfc3339(&created_at_str).unwrap_or_default().into(),
                updated_at: DateTime::parse_from_rfc3339(&updated_at_str).unwrap_or_default().into(),
                oidc_subject: row.get("oidc_subject"),
            });
        }
        Ok(users)
    }

    async fn update_user(&self, user: User, org_id: &str) -> Result<(), String> {
        let roles_json = serde_json::to_string(&user.roles).unwrap_or_default();

        let query = if org_id == "system" && !crate::config::get().multitenant {
            r#"
            UPDATE users SET username=?, email=?, password_hash=?, roles=?, active=?,
            organization_id=?, oidc_subject=?, updated_at=?
            WHERE id=? RETURNING id
            "#
        } else {
            r#"
            UPDATE users SET username=?, email=?, password_hash=?, roles=?, active=?,
            organization_id=?, oidc_subject=?, updated_at=?
            WHERE id=? AND organization_id=? RETURNING id
            "#
        };

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let res = if org_id == "system" && !crate::config::get().multitenant {
            sqlx::query(query)
                .bind(&user.username).bind(&user.email).bind(&user.password_hash)
                .bind(roles_json).bind(user.active).bind(&user.organization_id)
                .bind(&user.oidc_subject).bind(user.updated_at.to_rfc3339()).bind(&user.id)
                .fetch_optional(&mut *tx).await
        } else {
            sqlx::query(query)
                .bind(&user.username).bind(&user.email).bind(&user.password_hash)
                .bind(roles_json).bind(user.active).bind(&user.organization_id)
                .bind(&user.oidc_subject).bind(user.updated_at.to_rfc3339()).bind(&user.id).bind(org_id)
                .fetch_optional(&mut *tx).await
        }.map_err(|e| e.to_string())?;

        if res.is_none() {
            return Err("user not found or unauthorized".to_string());
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn delete_user(&self, id: &str, org_id: &str) -> Result<(), String> {
        let query = if org_id == "system" && !crate::config::get().multitenant {
            "DELETE FROM users WHERE id = ? RETURNING id"
        } else {
            "DELETE FROM users WHERE id = ? AND organization_id = ? RETURNING id"
        };

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let res = if org_id == "system" && !crate::config::get().multitenant {
            sqlx::query(query).bind(id).fetch_optional(&mut *tx).await
        } else {
            sqlx::query(query).bind(id).bind(org_id).fetch_optional(&mut *tx).await
        }.map_err(|e| e.to_string())?;

        if res.is_none() {
            return Err("user not found or unauthorized".to_string());
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn revoke_token(&self, jti: String, exp: DateTime<Utc>, org_id: &str) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            INSERT INTO revoked_tokens (jti, expires_at, tenant_id) VALUES (?, ?, ?)
            ON CONFLICT (jti) DO NOTHING
            "#
        )
        .bind(jti)
        .bind(exp.to_rfc3339())
        .bind(org_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let _ = sqlx::query("DELETE FROM revoked_tokens WHERE expires_at < CURRENT_TIMESTAMP")
            .execute(&mut *tx)
            .await;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn is_revoked(&self, jti: &str, _org_id: &str) -> Result<bool, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let row = sqlx::query("SELECT COUNT(*) FROM revoked_tokens WHERE jti = ? AND expires_at >= CURRENT_TIMESTAMP")
            .bind(jti)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        let count: i64 = row.get(0);
        tx.rollback().await.map_err(|e| e.to_string())?;

        Ok(count > 0)
    }
}
