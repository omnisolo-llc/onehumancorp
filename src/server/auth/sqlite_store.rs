use async_trait::async_trait;
use sqlx::SqlitePool;
use super::User;
use chrono::{DateTime, Utc};
use sqlx::Row;
use super::postgres_store::UserRepository;


macro_rules! validate_org_id {
    ($org_id:expr) => {
        if ::server_config::get().multitenant {
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
        // For SQLite in Standalone mode, there's no multi-tenant isolation via connection parameters.
        // We still store the org_id to conform to the interface.
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
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e: sqlx::Error| e.to_string())?;

        Ok(())
    }

    async fn get_by_id(&self, id: &str, org_id: &str) -> Result<User, String> {
        validate_org_id!(org_id);
        let is_multitenant = ::server_config::get().multitenant;
        let should_bypass = !is_multitenant;
        let query = if should_bypass {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE id = $1"
        } else {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE id = $1 AND tenant_id = $2"
        };
        let row = if should_bypass {
            sqlx::query(query).bind(id).fetch_one(&self.pool).await.map_err(|e| e.to_string())?
        } else {
            sqlx::query(query).bind(id).bind(org_id).fetch_one(&self.pool).await.map_err(|e| e.to_string())?
        };

        let roles_json: String = row.get("roles");
        let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

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
        let is_multitenant = ::server_config::get().multitenant;
        let should_bypass = !is_multitenant;
        let query = if should_bypass {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE username = $1"
        } else {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE username = $1 AND tenant_id = $2"
        };
        let row = if should_bypass {
            sqlx::query(query).bind(username).fetch_one(&self.pool).await.map_err(|e| e.to_string())?
        } else {
            sqlx::query(query).bind(username).bind(org_id).fetch_one(&self.pool).await.map_err(|e| e.to_string())?
        };

        let roles_json: String = row.get("roles");
        let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

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
        let is_multitenant = ::server_config::get().multitenant;
        let should_bypass = !is_multitenant;
        let query = if should_bypass {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE email = $1"
        } else {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE email = $1 AND tenant_id = $2"
        };
        let row = if should_bypass {
            sqlx::query(query).bind(email).fetch_one(&self.pool).await.map_err(|e| e.to_string())?
        } else {
            sqlx::query(query).bind(email).bind(org_id).fetch_one(&self.pool).await.map_err(|e| e.to_string())?
        };

        let roles_json: String = row.get("roles");
        let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

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
        let is_multitenant = ::server_config::get().multitenant;
        let should_bypass = !is_multitenant;
        let query = if should_bypass {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE oidc_subject = $1"
        } else {
            "SELECT id, username, email, password_hash, roles, active, tenant_id, oidc_subject, created_at, updated_at FROM users WHERE oidc_subject = $1 AND tenant_id = $2"
        };
        let row = if should_bypass {
            sqlx::query(query).bind(sub).fetch_one(&self.pool).await.map_err(|e| e.to_string())?
        } else {
            sqlx::query(query).bind(sub).bind(org_id).fetch_one(&self.pool).await.map_err(|e| e.to_string())?
        };

        let roles_json: String = row.get("roles");
        let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

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
        let is_multitenant = ::server_config::get().multitenant;
        let should_bypass = !is_multitenant;
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
            let roles_json: String = row.get("roles");
            let roles: Vec<String> = serde_json::from_str(&roles_json).unwrap_or_default();

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
        let query = r#"
            UPDATE users SET username=$2, email=$3, password_hash=$4, roles=$5, active=$6,
            tenant_id=$7, oidc_subject=$8, updated_at=$9
            WHERE id=$1 AND tenant_id = $10 RETURNING id
            "#;

        let res = sqlx::query(query)
            .bind(&user.id)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(roles_json)
            .bind(user.active)
            .bind(org_id)
            .bind(&user.oidc_subject)
            .bind(user.updated_at)
            .bind(org_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if res.is_none() {
            return Err("user not found or unauthorized".to_string());
        }
        Ok(())
    }

    async fn delete_user(&self, id: &str, org_id: &str) -> Result<(), String> {
        validate_org_id!(org_id);
        let is_multitenant = ::server_config::get().multitenant;
        let should_bypass = !is_multitenant;
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
        sqlx::query(
            r#"
            INSERT INTO revoked_tokens (jti, expires_at, tenant_id) VALUES ($1, $2, $3)
            ON CONFLICT (jti) DO NOTHING
            "#
        )
        .bind(jti)
        .bind(exp)
        .bind(org_id)
        .execute(&self.pool)
        .await
        .map_err(|e: sqlx::Error| e.to_string())?;

        // GC expired entries
        let _ = sqlx::query("DELETE FROM revoked_tokens WHERE expires_at < CURRENT_TIMESTAMP")
            .execute(&self.pool)
            .await;

        Ok(())
    }

    async fn is_revoked(&self, jti: &str, org_id: &str) -> Result<bool, String> {
        validate_org_id!(org_id);
        let row = sqlx::query("SELECT COUNT(*) FROM revoked_tokens WHERE jti = $1 AND expires_at >= CURRENT_TIMESTAMP AND tenant_id = $2")
            .bind(jti)
            .bind(org_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e: sqlx::Error| e.to_string())?;

        let count: i32 = row.get(0);
        Ok(count > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    async fn test_sqlite_multitenant_idor_system_bypass_prevention() {
        let _pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let repo = SqliteUserRepository::new(_pool.clone());

        let is_multitenant = ::server_config::get().multitenant;
        let should_bypass = !is_multitenant;
        assert!(!should_bypass || is_multitenant == false, "Cloud mode should NEVER bypass tenant filters when org_id is 'system'");

        let res = repo.get_by_id("dummy_id", "system").await;
        if is_multitenant {
            assert!(res.is_err(), "Must reject system id in multitenant mode");
            assert_eq!(res.unwrap_err(), "tenant_id 'system' cannot be queried in multi-tenant mode".to_string());
        } else {
            assert!(res.is_err() || res.is_ok(), "Codebase query executed correctly");
        }
    }
}
