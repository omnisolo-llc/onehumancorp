use async_trait::async_trait;
use sqlx::MySqlPool;
use super::User;
use chrono::{DateTime, Utc};
use super::user_repository::UserRepository;

fn is_multitenant_mode() -> bool {
    #[cfg(test)]
    {
        if let Ok(val) = std::env::var("OHC_MULTITENANT") {
            return val == "true";
        }
    }
    ::server_config::get().multitenant
}

use sqlx::Row;

macro_rules! validate_org_id {
    ($org_id:expr) => {
        if is_multitenant_mode() {
            if $org_id.trim().eq_ignore_ascii_case("system") {
                return Err("tenant_id 'system' cannot be queried in multi-tenant mode".to_string());
            }
            if $org_id.trim().is_empty() {
                return Err("empty tenant_id is not allowed in multi-tenant mode".to_string());
            }
        }
    };
}

pub struct MySqlUserRepository {
    pool: MySqlPool,
}

impl MySqlUserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        MySqlUserRepository { pool }
    }
}

#[async_trait]
impl UserRepository for MySqlUserRepository {
    async fn create_user(&self, user: User, org_id: &str) -> Result<(), String> {
        let org_id = org_id.trim();
        validate_org_id!(org_id);
        let roles_json = serde_json::to_string(&user.roles).unwrap_or_default();

        let query = r#"
        INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

        sqlx::query(query)
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(roles_json)
            .bind(user.active)
            .bind(org_id)
            .bind(&user.oidc_subject)
            .bind(user.created_at)
            .bind(user.updated_at)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_by_id(&self, id: &str, org_id: &str) -> Result<User, String> {
        let org_id = org_id.trim();
        validate_org_id!(org_id);
        let query = "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE id = ? AND tenant_id = ?";
        let row_opt = sqlx::query(query).bind(id).bind(org_id).fetch_optional(&self.pool).await.map_err(|e| e.to_string())?;

        let row = match row_opt {
            Some(r) => r,
            None => return Err("user not found".to_string()),
        };

        let roles_json: serde_json::Value = row.try_get("roles").unwrap_or_else(|_| serde_json::Value::Null);
        let roles: Vec<String> = serde_json::from_value(roles_json).unwrap_or_default();

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            roles,
            active: row.get("active"),
            organization_id: row.get("tenant_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            oidc_subject: row.get("oidc_subject"),
        })
    }

    async fn get_by_username(&self, username: &str, org_id: &str) -> Result<User, String> {
        let org_id = org_id.trim();
        validate_org_id!(org_id);
        let query = "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE username = ? AND tenant_id = ?";
        let row_opt = sqlx::query(query).bind(username).bind(org_id).fetch_optional(&self.pool).await.map_err(|e| e.to_string())?;

        let row = match row_opt {
            Some(r) => r,
            None => return Err("user not found".to_string()),
        };

        let roles_json: serde_json::Value = row.try_get("roles").unwrap_or_else(|_| serde_json::Value::Null);
        let roles: Vec<String> = serde_json::from_value(roles_json).unwrap_or_default();

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            roles,
            active: row.get("active"),
            organization_id: row.get("tenant_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            oidc_subject: row.get("oidc_subject"),
        })
    }

    async fn get_by_email(&self, email: &str, org_id: &str) -> Result<User, String> {
        let org_id = org_id.trim();
        validate_org_id!(org_id);
        let query = "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE email = ? AND tenant_id = ?";
        let row_opt = sqlx::query(query).bind(email).bind(org_id).fetch_optional(&self.pool).await.map_err(|e| e.to_string())?;

        let row = match row_opt {
            Some(r) => r,
            None => return Err("user not found".to_string()),
        };

        let roles_json: serde_json::Value = row.try_get("roles").unwrap_or_else(|_| serde_json::Value::Null);
        let roles: Vec<String> = serde_json::from_value(roles_json).unwrap_or_default();

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            roles,
            active: row.get("active"),
            organization_id: row.get("tenant_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            oidc_subject: row.get("oidc_subject"),
        })
    }

    async fn get_by_login_identifier(
        &self,
        identifier: &str,
        org_id: &str,
    ) -> Result<Option<User>, String> {
        let org_id = org_id.trim();
        validate_org_id!(org_id);
        let rows = sqlx::query(
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at
             FROM users
             WHERE (username = ? OR email = ?) AND tenant_id = ? AND active = TRUE
             LIMIT 2",
        )
        .bind(identifier)
        .bind(identifier)
        .bind(org_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if rows.len() != 1 {
            return Ok(None);
        }
        let row = &rows[0];
        let roles_json: serde_json::Value = row.try_get("roles").unwrap_or(serde_json::Value::Null);
        let roles = serde_json::from_value(roles_json).unwrap_or_default();
        Ok(Some(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            roles,
            active: row.get("active"),
            organization_id: row.get("tenant_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            oidc_subject: row.get("oidc_subject"),
        }))
    }

    async fn get_by_oidc_subject(&self, sub: &str, org_id: &str) -> Result<User, String> {
        let org_id = org_id.trim();
        validate_org_id!(org_id);
        let query = "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE oidc_subject = ? AND tenant_id = ?";
        let row_opt = sqlx::query(query).bind(sub).bind(org_id).fetch_optional(&self.pool).await.map_err(|e| e.to_string())?;

        let row = match row_opt {
            Some(r) => r,
            None => return Err("user not found".to_string()),
        };

        let roles_json: serde_json::Value = row.try_get("roles").unwrap_or_else(|_| serde_json::Value::Null);
        let roles: Vec<String> = serde_json::from_value(roles_json).unwrap_or_default();

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            roles,
            active: row.get("active"),
            organization_id: row.get("tenant_id"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            oidc_subject: row.get("oidc_subject"),
        })
    }

    async fn list_users(&self, org_id: &str) -> Result<Vec<User>, String> {
        let org_id = org_id.trim();
        validate_org_id!(org_id);
        let query = "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE tenant_id = ? ORDER BY created_at";
        let rows = sqlx::query(query).bind(org_id).fetch_all(&self.pool).await.map_err(|e| e.to_string())?;

        let mut users = Vec::new();
        for row in rows {
            let roles_json: serde_json::Value = row.try_get("roles").unwrap_or_else(|_| serde_json::Value::Null);
            let roles: Vec<String> = serde_json::from_value(roles_json).unwrap_or_default();

            users.push(User {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                roles,
                active: row.get("active"),
                organization_id: row.get("tenant_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                oidc_subject: row.get("oidc_subject"),
            });
        }
        Ok(users)
    }

    async fn update_user(&self, user: User, org_id: &str) -> Result<(), String> {
        let org_id = org_id.trim();
        validate_org_id!(org_id);
        let roles_json = serde_json::to_string(&user.roles).unwrap_or_default();

        let query = r#"
            UPDATE users SET username=?, email=?, password_hash=?, roles=?, active=?,
            oidc_subject=?, updated_at=?
            WHERE id=? AND tenant_id = ?
            "#;

        let res = sqlx::query(query)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(roles_json)
            .bind(user.active)
            .bind(&user.oidc_subject)
            .bind(user.updated_at)
            .bind(&user.id)
            .bind(org_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if res.rows_affected() == 0 {
            return Err("user not found or unauthorized".to_string());
        }
        Ok(())
    }

    async fn delete_user(&self, id: &str, org_id: &str) -> Result<(), String> {
        let org_id = org_id.trim();
        validate_org_id!(org_id);
        let query = "DELETE FROM users WHERE id = ? AND tenant_id = ?";
        let res = sqlx::query(query).bind(id).bind(org_id).execute(&self.pool).await.map_err(|e: sqlx::Error| e.to_string())?;

        if res.rows_affected() == 0 {
            return Err("user not found or unauthorized".to_string());
        }
        Ok(())
    }

    async fn revoke_token(&self, jti: String, exp: DateTime<Utc>, org_id: &str) -> Result<(), String> {
        let org_id = org_id.trim();
        validate_org_id!(org_id);

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            INSERT INTO revoked_tokens (jti, expires_at, tenant_id) VALUES (?, ?, ?)
            ON DUPLICATE KEY UPDATE expires_at = VALUES(expires_at)
            "#
        )
        .bind(jti)
        .bind(exp)
        .bind(org_id)
        .execute(&mut *tx)
        .await
        .map_err(|e: sqlx::Error| e.to_string())?;

        let now = chrono::Utc::now();
        sqlx::query("DELETE FROM revoked_tokens WHERE expires_at < ? AND tenant_id = ?").bind(now).bind(org_id).execute(&mut *tx).await.map_err(|e: sqlx::Error| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn is_revoked(&self, jti: &str, org_id: &str) -> Result<bool, String> {
        let org_id = org_id.trim();
        validate_org_id!(org_id);

        let row = sqlx::query("SELECT COUNT(*) FROM revoked_tokens WHERE jti = ? AND expires_at >= ? AND tenant_id = ?")
            .bind(jti)
            .bind(chrono::Utc::now())
            .bind(org_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e: sqlx::Error| e.to_string())?;

        let count: i64 = row.get(0);
        Ok(count > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::mysql::MySqlPoolOptions;
    use chrono::Utc;

    async fn get_mysql_db() -> Option<MySqlUserRepository> {
        if let Ok(url) = std::env::var("OHC_DATABASE_URL") {
            if url.starts_with("mysql") {
                let pool = MySqlPoolOptions::new()
                    .max_connections(2)
                    .connect(&url)
                    .await
                    .ok()?;
                return Some(MySqlUserRepository::new(pool));
            }
        }
        None
    }

    #[tokio::test]
    async fn test_mysql_user_lifecycle() {
        let repo = match get_mysql_db().await {
            Some(r) => r,
            None => return, // Skip test if no MySQL DB is configured
        };

        // Create user
        let user_id = format!("user_{}", uuid::Uuid::new_v4());
        let username = format!("user_{}", uuid::Uuid::new_v4());
        let email = format!("{}@example.com", username);
        let user = User {
            id: user_id.clone(),
            username: username.clone(),
            email: email.clone(),
            password_hash: "hashed".to_string(),
            roles: vec!["admin".to_string()],
            active: true,
            organization_id: Some("tenant-mysql".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            oidc_subject: None,
        };

        repo.create_user(user.clone(), "tenant-mysql").await.unwrap();

        // Get by ID
        let fetched = repo.get_by_id(&user_id, "tenant-mysql").await.unwrap();
        assert_eq!(fetched.id, user_id);
        assert_eq!(fetched.username, username);

        // Get by username
        let fetched_by_uname = repo.get_by_username(&username, "tenant-mysql").await.unwrap();
        assert_eq!(fetched_by_uname.id, user_id);

        // Get by email
        let fetched_by_email = repo.get_by_email(&email, "tenant-mysql").await.unwrap();
        assert_eq!(fetched_by_email.id, user_id);

        // Get by login identifier
        let logged_in = repo.get_by_login_identifier(&username, "tenant-mysql").await.unwrap().unwrap();
        assert_eq!(logged_in.id, user_id);

        // Update user
        let mut updated_user = user.clone();
        updated_user.roles = vec!["user".to_string()];
        repo.update_user(updated_user, "tenant-mysql").await.unwrap();

        let fetched_updated = repo.get_by_id(&user_id, "tenant-mysql").await.unwrap();
        assert_eq!(fetched_updated.roles, vec!["user".to_string()]);

        // Revoke token
        let jti = uuid::Uuid::new_v4().to_string();
        repo.revoke_token(jti.clone(), Utc::now() + chrono::Duration::hours(1), "tenant-mysql").await.unwrap();
        assert!(repo.is_revoked(&jti, "tenant-mysql").await.unwrap());

        // Delete user
        repo.delete_user(&user_id, "tenant-mysql").await.unwrap();
        assert!(repo.get_by_id(&user_id, "tenant-mysql").await.is_err());
    }
}
