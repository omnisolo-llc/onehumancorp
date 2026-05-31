use async_trait::async_trait;
use sqlx::SqlitePool;
use super::{User, UserRepository};
use chrono::{DateTime, Utc};
use sqlx::Row;

#[allow(dead_code)]
pub struct SqliteUserRepository {
    pool: SqlitePool,
}

#[allow(dead_code)]
impl SqliteUserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        SqliteUserRepository { pool }
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn create_user(&self, user: User, org_id: &str) -> Result<(), String> {
        let roles_json = serde_json::to_string(&user.roles).unwrap_or_default();
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(roles_json)
        .bind(user.active)
        .bind(org_id)
        .bind(&user.oidc_subject)
        .bind(user.created_at.timestamp())
        .bind(user.updated_at.timestamp())
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_by_id(&self, id: &str, org_id: &str) -> Result<User, String> {
        let query = "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE id = $1 AND tenant_id = $2";

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let row = sqlx::query(query).bind(id).bind(org_id).fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;

        let roles_json: String = row.get("roles");
        let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

        let created_at_ts: i64 = row.get("created_at");
        let updated_at_ts: i64 = row.get("updated_at");

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            roles,
            active: row.get("active"),
            organization_id: row.get("tenant_id"),
            created_at: DateTime::from_timestamp(created_at_ts, 0).unwrap_or(Utc::now()),
            updated_at: DateTime::from_timestamp(updated_at_ts, 0).unwrap_or(Utc::now()),
            oidc_subject: row.get("oidc_subject"),
        })
    }

    async fn get_by_username(&self, username: &str, org_id: &str) -> Result<User, String> {
        let query = "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE username = $1 AND tenant_id = $2";

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let row = sqlx::query(query).bind(username).bind(org_id).fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;

        let roles_json: String = row.get("roles");
        let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

        let created_at_ts: i64 = row.get("created_at");
        let updated_at_ts: i64 = row.get("updated_at");

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            roles,
            active: row.get("active"),
            organization_id: row.get("tenant_id"),
            created_at: DateTime::from_timestamp(created_at_ts, 0).unwrap_or(Utc::now()),
            updated_at: DateTime::from_timestamp(updated_at_ts, 0).unwrap_or(Utc::now()),
            oidc_subject: row.get("oidc_subject"),
        })
    }

    async fn get_by_email(&self, email: &str, org_id: &str) -> Result<User, String> {
        let query = "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE email = $1 AND tenant_id = $2";

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let row = sqlx::query(query).bind(email).bind(org_id).fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;

        let roles_json: String = row.get("roles");
        let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

        let created_at_ts: i64 = row.get("created_at");
        let updated_at_ts: i64 = row.get("updated_at");

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            roles,
            active: row.get("active"),
            organization_id: row.get("tenant_id"),
            created_at: DateTime::from_timestamp(created_at_ts, 0).unwrap_or(Utc::now()),
            updated_at: DateTime::from_timestamp(updated_at_ts, 0).unwrap_or(Utc::now()),
            oidc_subject: row.get("oidc_subject"),
        })
    }

    async fn get_by_oidc_subject(&self, sub: &str, org_id: &str) -> Result<User, String> {
        let query = "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE oidc_subject = $1 AND tenant_id = $2";

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let row = sqlx::query(query).bind(sub).bind(org_id).fetch_one(&mut *tx).await.map_err(|e| e.to_string())?;

        let roles_json: String = row.get("roles");
        let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

        let created_at_ts: i64 = row.get("created_at");
        let updated_at_ts: i64 = row.get("updated_at");

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            roles,
            active: row.get("active"),
            organization_id: row.get("tenant_id"),
            created_at: DateTime::from_timestamp(created_at_ts, 0).unwrap_or(Utc::now()),
            updated_at: DateTime::from_timestamp(updated_at_ts, 0).unwrap_or(Utc::now()),
            oidc_subject: row.get("oidc_subject"),
        })
    }

    async fn list_users(&self, org_id: &str) -> Result<Vec<User>, String> {
        let query = "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE tenant_id = $1 ORDER BY created_at";

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let rows = sqlx::query(query).bind(org_id).fetch_all(&mut *tx).await.map_err(|e| e.to_string())?;

        let mut users = Vec::new();
        for row in rows {
            let roles_json: String = row.get("roles");
            let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

            let created_at_ts: i64 = row.get("created_at");
            let updated_at_ts: i64 = row.get("updated_at");

            users.push(User {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                roles,
                active: row.get("active"),
                organization_id: row.get("tenant_id"),
                created_at: DateTime::from_timestamp(created_at_ts, 0).unwrap_or(Utc::now()),
                updated_at: DateTime::from_timestamp(updated_at_ts, 0).unwrap_or(Utc::now()),
                oidc_subject: row.get("oidc_subject"),
            });
        }
        Ok(users)
    }

    async fn update_user(&self, user: User, org_id: &str) -> Result<(), String> {
        let roles_json = serde_json::to_string(&user.roles).unwrap_or_default();

        let query = r#"
            UPDATE users SET username=$2, email=$3, password_hash=$4, roles=$5, active=$6,
            tenant_id=$7, oidc_subject=$8, updated_at=$9
            WHERE id=$1 AND tenant_id=$10 RETURNING id
            "#;

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let res = sqlx::query(query)
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(roles_json)
            .bind(user.active)
            .bind(&user.organization_id)
            .bind(&user.oidc_subject)
            .bind(user.updated_at.timestamp())
            .bind(org_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        if res.is_none() {
            return Err("user not found or unauthorized".to_string());
        }

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn delete_user(&self, id: &str, org_id: &str) -> Result<(), String> {
        let query = "DELETE FROM users WHERE id = $1 AND tenant_id = $2 RETURNING id";

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let res = sqlx::query(query).bind(id).bind(org_id).fetch_optional(&mut *tx).await.map_err(|e| e.to_string())?;

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
            INSERT INTO revoked_tokens (jti, expires_at, tenant_id) VALUES ($1, $2, $3)
            ON CONFLICT (jti) DO NOTHING
            "#
        )
        .bind(jti)
        .bind(exp.timestamp())
        .bind(org_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let _ = sqlx::query("DELETE FROM revoked_tokens WHERE expires_at < strftime('%s', 'now')")
            .execute(&mut *tx)
            .await;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn is_revoked(&self, jti: &str, org_id: &str) -> Result<bool, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let row = sqlx::query("SELECT COUNT(*) FROM revoked_tokens WHERE jti = $1 AND tenant_id = $2 AND expires_at >= strftime('%s', 'now')")
            .bind(jti)
            .bind(org_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        let count: i64 = row.get(0);
        tx.rollback().await.map_err(|e| e.to_string())?;

        Ok(count > 0)
    }
}
