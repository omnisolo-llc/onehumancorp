use async_trait::async_trait;
use sqlx::SqlitePool;
use super::User;
use chrono::{DateTime, Utc};

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
use super::user_repository::UserRepository;


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
            async fn create_user(&self, user: User, org_id: &str) -> Result<(), String> {
        validate_org_id!(org_id);
        let roles_json = serde_json::to_string(&user.roles).unwrap_or_default();
        let is_multitenant = is_multitenant_mode();
        let should_bypass = (!is_multitenant) && org_id.eq_ignore_ascii_case("system");

        let query = r#"
        INSERT INTO users (id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at)
        VALUES ($1, $2, $3, $4, json($5), $6, $7, $8, $9, $10)
        "#;

        sqlx::query(query)
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(roles_json)
        .bind(user.active)
        .bind(if should_bypass { "system" } else { org_id })
        .bind(&user.oidc_subject)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e: sqlx::Error| e.to_string())?;

        Ok(())
    }

    async fn get_by_id(&self, id: &str, org_id: &str) -> Result<User, String> {
        validate_org_id!(org_id);
        let is_multitenant = is_multitenant_mode();
        let should_bypass = (!is_multitenant) && org_id.eq_ignore_ascii_case("system");
        let query = if should_bypass {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE id = $1"
        } else {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE id = $1 AND tenant_id = $2"
        };
        let row_opt = if should_bypass {
            sqlx::query(query).bind(id).fetch_optional(&self.pool).await.map_err(|e| e.to_string())?
        } else {
            sqlx::query(query).bind(id).bind(org_id).fetch_optional(&self.pool).await.map_err(|e| e.to_string())?
        };

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
        validate_org_id!(org_id);
        let is_multitenant = is_multitenant_mode();
        let should_bypass = (!is_multitenant) && org_id.eq_ignore_ascii_case("system");
        let query = if should_bypass {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE username = $1"
        } else {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE username = $1 AND tenant_id = $2"
        };
        let row_opt = if should_bypass {
            sqlx::query(query).bind(username).fetch_optional(&self.pool).await.map_err(|e| e.to_string())?
        } else {
            sqlx::query(query).bind(username).bind(org_id).fetch_optional(&self.pool).await.map_err(|e| e.to_string())?
        };

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
        validate_org_id!(org_id);
        let is_multitenant = is_multitenant_mode();
        let should_bypass = (!is_multitenant) && org_id.eq_ignore_ascii_case("system");
        let query = if should_bypass {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE email = $1"
        } else {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE email = $1 AND tenant_id = $2"
        };
        let row_opt = if should_bypass {
            sqlx::query(query).bind(email).fetch_optional(&self.pool).await.map_err(|e| e.to_string())?
        } else {
            sqlx::query(query).bind(email).bind(org_id).fetch_optional(&self.pool).await.map_err(|e| e.to_string())?
        };

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

    async fn get_by_oidc_subject(&self, sub: &str, org_id: &str) -> Result<User, String> {
        validate_org_id!(org_id);
        let is_multitenant = is_multitenant_mode();
        let should_bypass = (!is_multitenant) && org_id.eq_ignore_ascii_case("system");
        let query = if should_bypass {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE oidc_subject = $1"
        } else {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE oidc_subject = $1 AND tenant_id = $2"
        };
        let row_opt = if should_bypass {
            sqlx::query(query).bind(sub).fetch_optional(&self.pool).await.map_err(|e| e.to_string())?
        } else {
            sqlx::query(query).bind(sub).bind(org_id).fetch_optional(&self.pool).await.map_err(|e| e.to_string())?
        };

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
        validate_org_id!(org_id);
        let is_multitenant = is_multitenant_mode();
        let should_bypass = (!is_multitenant) && org_id.eq_ignore_ascii_case("system");
        let query = if should_bypass {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users ORDER BY created_at"
        } else {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE tenant_id = $1 ORDER BY created_at"
        };
        let rows = if should_bypass {
            sqlx::query(query).fetch_all(&self.pool).await.map_err(|e| e.to_string())?
        } else {
            sqlx::query(query).bind(org_id).fetch_all(&self.pool).await.map_err(|e| e.to_string())?
        };

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
        validate_org_id!(org_id);
        let roles_json = serde_json::to_string(&user.roles).unwrap_or_default();
        let is_multitenant = is_multitenant_mode();
        let should_bypass = (!is_multitenant) && org_id.eq_ignore_ascii_case("system");

        let query = if should_bypass {
            r#"
            UPDATE users SET username=$2, email=$3, password_hash=$4, roles=json($5), active=$6,
            oidc_subject=$7, updated_at=$8
            WHERE id=$1 RETURNING id
            "#
        } else {
            r#"
            UPDATE users SET username=$2, email=$3, password_hash=$4, roles=json($5), active=$6,
            oidc_subject=$7, updated_at=$8
            WHERE id=$1 AND tenant_id = $9 RETURNING id
            "#
        };

        let res = if should_bypass {
            sqlx::query(query)
                .bind(&user.id)
                .bind(&user.username)
                .bind(&user.email)
                .bind(&user.password_hash)
                .bind(roles_json)
                .bind(user.active)
                .bind(&user.oidc_subject)
                .bind(user.updated_at)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?
        } else {
            sqlx::query(query)
                .bind(&user.id)
                .bind(&user.username)
                .bind(&user.email)
                .bind(&user.password_hash)
                .bind(roles_json)
                .bind(user.active)
                .bind(&user.oidc_subject)
                .bind(user.updated_at)
                .bind(org_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?
        };

        if res.is_none() {
            return Err("user not found or unauthorized".to_string());
        }
        Ok(())
    }

    async fn delete_user(&self, id: &str, org_id: &str) -> Result<(), String> {
        validate_org_id!(org_id);
        let is_multitenant = is_multitenant_mode();
        let should_bypass = (!is_multitenant) && org_id.eq_ignore_ascii_case("system");
        let query = if should_bypass {
            "DELETE FROM users WHERE id = $1 RETURNING id"
        } else {
            "DELETE FROM users WHERE id = $1 AND tenant_id = $2 RETURNING id"
        };
        let res = if should_bypass {
            sqlx::query(query).bind(id).fetch_optional(&self.pool).await.map_err(|e: sqlx::Error| e.to_string())?
        } else {
            sqlx::query(query).bind(id).bind(org_id).fetch_optional(&self.pool).await.map_err(|e: sqlx::Error| e.to_string())?
        };

        if res.is_none() {
            return Err("user not found or unauthorized".to_string());
        }
        Ok(())
    }

    async fn revoke_token(&self, jti: String, exp: DateTime<Utc>, org_id: &str) -> Result<(), String> {
        validate_org_id!(org_id);

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            INSERT INTO revoked_tokens (jti, expires_at, tenant_id) VALUES ($1, $2, $3)
            ON CONFLICT (jti, tenant_id) DO NOTHING
            "#
        )
        .bind(jti)
        .bind(exp)
        .bind(org_id)
        .execute(&mut *tx)
        .await
        .map_err(|e: sqlx::Error| e.to_string())?;


        let now = chrono::Utc::now();
        sqlx::query("DELETE FROM revoked_tokens WHERE expires_at < $1 AND tenant_id = $2").bind(now).bind(org_id).execute(&mut *tx).await.map_err(|e: sqlx::Error| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn is_revoked(&self, jti: &str, org_id: &str) -> Result<bool, String> {
        validate_org_id!(org_id);
        let is_multitenant = is_multitenant_mode();
        let should_bypass = (!is_multitenant) && org_id.eq_ignore_ascii_case("system");

        let row = if should_bypass {
            sqlx::query("SELECT COUNT(*) FROM revoked_tokens WHERE jti = $1 AND expires_at >= $2")
                .bind(jti)
                .bind(chrono::Utc::now())
                .fetch_one(&self.pool)
                .await
                .map_err(|e: sqlx::Error| e.to_string())?
        } else {
            sqlx::query("SELECT COUNT(*) FROM revoked_tokens WHERE jti = $1 AND expires_at >= $3 AND tenant_id = $2")
                .bind(jti)
                .bind(org_id)
                .bind(chrono::Utc::now())
                .fetch_one(&self.pool)
                .await
                .map_err(|e: sqlx::Error| e.to_string())?
        };

        let count: i32 = row.get(0);
        Ok(count > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    static ENV_MUTEX: Mutex<()> = Mutex::new(());
    use sqlx::sqlite::SqlitePoolOptions;
    use chrono::Utc;

    #[tokio::test]
    async fn test_sqlite_create_user_organization_id_parity() {
        let _pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE users (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                email TEXT NOT NULL,
                password_hash TEXT,
                roles TEXT,
                active BOOLEAN,
                tenant_id TEXT,
                oidc_subject TEXT,
                created_at TIMESTAMPTZ,
                updated_at TIMESTAMPTZ
            )"
        )
        .execute(&_pool)
        .await
        .unwrap();

        let repo = SqliteUserRepository::new(_pool.clone());
        let user = User {
            id: "test-id".to_string(),
            username: "test-user".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "".to_string(),
            roles: vec!["admin".to_string()],
            active: true,
            organization_id: Some("user-org-id".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            oidc_subject: None,
        };

        // Pass a different org_id argument to verify the model binds `org_id` argument instead
        repo.create_user(user, "function-arg-org-id").await.unwrap();

        let row = sqlx::query("SELECT tenant_id FROM users WHERE id = 'test-id'")
            .fetch_one(&_pool)
            .await
            .unwrap();

        let fetched_org_id: String = sqlx::Row::get(&row, "tenant_id");
        assert_eq!(fetched_org_id, "function-arg-org-id");
    }

    #[tokio::test]
    async fn test_sqlite_revoke_token_tenant_isolation_regression() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS revoked_tokens (
                jti TEXT,
                tenant_id TEXT,
                expires_at TIMESTAMPTZ,
                PRIMARY KEY (jti, tenant_id)
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        let repo = SqliteUserRepository::new(pool.clone());

        let exp1 = Utc::now() - chrono::Duration::hours(1);
        let exp2 = Utc::now() + chrono::Duration::hours(1);

        sqlx::query("INSERT INTO revoked_tokens (jti, tenant_id, expires_at) VALUES ('jti-1', 'tenant-1', $1)")
            .bind(exp1)
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO revoked_tokens (jti, tenant_id, expires_at) VALUES ('jti-2', 'tenant-2', $1)")
            .bind(exp1)
            .execute(&pool)
            .await
            .unwrap();

        // Perform GC explicitly via our isolated function structure for tenant-1
        let _ = repo.revoke_token("jti-3".to_string(), exp2, "tenant-1").await;

        // tenant-2 token should remain untouched.
        let count: i64 = sqlx::query("SELECT COUNT(*) FROM revoked_tokens WHERE tenant_id = 'tenant-2'")
            .fetch_one(&pool)
            .await
            .unwrap()
            .get(0);
        assert_eq!(count, 1, "GC leak across tenants");

        let count_tenant_1: i64 = sqlx::query("SELECT COUNT(*) FROM revoked_tokens WHERE tenant_id = 'tenant-1' AND jti != 'jti-3'")
            .fetch_one(&pool)
            .await
            .unwrap()
            .get(0);
        assert_eq!(count_tenant_1, 0, "The expired token for tenant-1 should be garbage collected");

    }

    #[tokio::test]
    async fn test_sqlite_multitenant_idor_system_bypass_prevention() {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let _pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let repo = SqliteUserRepository::new(_pool.clone());
        temp_env::async_with_vars([("OHC_MULTITENANT", Some("true"))], async {
            let is_multitenant = is_multitenant_mode();
            let org_id = "system"; let should_bypass = (!is_multitenant) && org_id.eq_ignore_ascii_case("system");
            assert!(!should_bypass, "Cloud mode should NEVER bypass tenant filters when org_id is 'system'");

            let res = repo.get_by_id("dummy_id", "system").await;
            assert!(res.is_err(), "Must reject system id in multitenant mode");
            assert_eq!(res.unwrap_err(), "tenant_id 'system' cannot be queried in multi-tenant mode");
        }).await;
    }

    #[tokio::test]
    async fn test_update_user_tenant_isolation_regression() {
        let _lock = ENV_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
        let _pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let repo = SqliteUserRepository::new(_pool.clone());

        let dummy_user = User {
            id: "dummy_id_update".to_string(),
            username: "dummy_user".to_string(),
            email: "dummy@example.com".to_string(),
            password_hash: "hash".to_string(),
            roles: vec![],
            active: true,
            organization_id: Some("system".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            oidc_subject: Some("sub".to_string()),
        };

        // Ensure multitenant environment is mocked strictly for 'system' context evaluation
        temp_env::async_with_vars([("OHC_MULTITENANT", Some("true"))], async {
            let res = repo.update_user(dummy_user, "system").await;
            assert!(res.is_err(), "Must reject system org_id");
            assert_eq!(res.unwrap_err(), "tenant_id 'system' cannot be queried in multi-tenant mode");
        }).await;
    }
}
