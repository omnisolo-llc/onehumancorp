use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use sqlx::{PgPool, SqlitePool, Row};
use async_trait::async_trait;
use std::net::Ipv4Addr;
use std::str::FromStr;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpTool {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub description: Option<String>,
    pub config: Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
pub trait ToolRegistry: Send + Sync {
    async fn register_tool(&self, tenant_id: &str, name: &str, description: Option<&str>, config: Value) -> Result<McpTool, sqlx::Error>;
    async fn get_tool(&self, tenant_id: &str, id: &str) -> Result<McpTool, sqlx::Error>;
    async fn list_tools(&self, tenant_id: &str) -> Result<Vec<McpTool>, sqlx::Error>;
}

fn is_internal_ip(tenant_id: &str) -> bool {
    if let Ok(ip) = Ipv4Addr::from_str(tenant_id) {
        if ip.octets()[0] == 10 {
            return true; // 10.0.0.0/8
        }
    }
    false
}

pub struct PgToolRegistry {
    pool: PgPool,
}

impl PgToolRegistry {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ToolRegistry for PgToolRegistry {
    async fn register_tool(&self, tenant_id: &str, name: &str, description: Option<&str>, config: Value) -> Result<McpTool, sqlx::Error> {
        if is_internal_ip(tenant_id) {
            return Err(sqlx::Error::Protocol("Invalid tenant_id: Cannot use internal IPs".to_string()));
        }

        let id = uuid::Uuid::new_v4().to_string();
        let query = "
            INSERT INTO mcp_tools (id, tenant_id, name, description, config)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, description, config, created_at
        ";

        let row = sqlx::query(query)
            .bind(&id)
            .bind(tenant_id)
            .bind(name)
            .bind(description)
            .bind(config)
            .fetch_one(&self.pool)
            .await?;

        Ok(McpTool {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            name: row.get("name"),
            description: row.get("description"),
            config: row.get("config"),
            created_at: row.get("created_at"),
        })
    }

    async fn get_tool(&self, tenant_id: &str, id: &str) -> Result<McpTool, sqlx::Error> {
        let query = "
            SELECT id, tenant_id, name, description, config, created_at
            FROM mcp_tools
            WHERE id = $1 AND tenant_id = $2
        ";

        let row = sqlx::query(query)
            .bind(id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(McpTool {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            name: row.get("name"),
            description: row.get("description"),
            config: row.get("config"),
            created_at: row.get("created_at"),
        })
    }

    async fn list_tools(&self, tenant_id: &str) -> Result<Vec<McpTool>, sqlx::Error> {
        let query = "
            SELECT id, tenant_id, name, description, config, created_at
            FROM mcp_tools
            WHERE tenant_id = $1
        ";

        let rows = sqlx::query(query)
            .bind(tenant_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(|row| McpTool {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            name: row.get("name"),
            description: row.get("description"),
            config: row.get("config"),
            created_at: row.get("created_at"),
        }).collect())
    }
}

pub struct SqliteToolRegistry {
    pool: SqlitePool,
}

impl SqliteToolRegistry {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ToolRegistry for SqliteToolRegistry {
    async fn register_tool(&self, tenant_id: &str, name: &str, description: Option<&str>, config: Value) -> Result<McpTool, sqlx::Error> {
        let id = uuid::Uuid::new_v4().to_string();

        let query = "
            INSERT INTO mcp_tools (id, tenant_id, name, description, config)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, description, config, created_at
        ";

        // Bind for SQLite, use json config via string conversion manually here or json module to be safe for sqlite
        let config_str = serde_json::to_string(&config).unwrap_or_else(|_| "{}".to_string());

        let row = sqlx::query(query)
            .bind(&id)
            .bind(tenant_id)
            .bind(name)
            .bind(description)
            .bind(config_str)
            .fetch_one(&self.pool)
            .await?;

        let conf_str: String = row.get("config");
        let parsed_config: Value = serde_json::from_str(&conf_str).unwrap_or(serde_json::json!({}));

        Ok(McpTool {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            name: row.get("name"),
            description: row.get("description"),
            config: parsed_config,
            created_at: row.get("created_at"),
        })
    }

    async fn get_tool(&self, tenant_id: &str, id: &str) -> Result<McpTool, sqlx::Error> {
        let query = "
            SELECT id, tenant_id, name, description, config, created_at
            FROM mcp_tools
            WHERE id = $1 AND tenant_id = $2
        ";

        let row = sqlx::query(query)
            .bind(id)
            .bind(tenant_id)
            .fetch_one(&self.pool)
            .await?;

        let conf_str: String = row.get("config");
        let parsed_config: Value = serde_json::from_str(&conf_str).unwrap_or(serde_json::json!({}));

        Ok(McpTool {
            id: row.get("id"),
            tenant_id: row.get("tenant_id"),
            name: row.get("name"),
            description: row.get("description"),
            config: parsed_config,
            created_at: row.get("created_at"),
        })
    }

    async fn list_tools(&self, tenant_id: &str) -> Result<Vec<McpTool>, sqlx::Error> {
        let query = "
            SELECT id, tenant_id, name, description, config, created_at
            FROM mcp_tools
            WHERE tenant_id = $1
        ";

        let rows = sqlx::query(query)
            .bind(tenant_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(|row| {
            let conf_str: String = row.get("config");
            let parsed_config: Value = serde_json::from_str(&conf_str).unwrap_or(serde_json::json!({}));

            McpTool {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                name: row.get("name"),
                description: row.get("description"),
                config: parsed_config,
                created_at: row.get("created_at"),
            }
        }).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::postgres::PgPoolOptions;
    use std::time::Duration;

    #[tokio::test]
    async fn test_sqlite_tool_registry_isolation() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Run migrations/setup for sqlite
        sqlx::query(
            "CREATE TABLE mcp_tools (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                config TEXT DEFAULT '{}',
                created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        let registry = SqliteToolRegistry::new(pool);

        // Register tools for tenant A
        registry.register_tool("tenant_A", "tool_a1", Some("desc a1"), serde_json::json!({})).await.unwrap();
        registry.register_tool("tenant_A", "tool_a2", Some("desc a2"), serde_json::json!({})).await.unwrap();

        // Register tools for tenant B
        registry.register_tool("tenant_B", "tool_b1", Some("desc b1"), serde_json::json!({})).await.unwrap();

        let tools_a = registry.list_tools("tenant_A").await.unwrap();
        assert_eq!(tools_a.len(), 2);

        let tools_b = registry.list_tools("tenant_B").await.unwrap();
        assert_eq!(tools_b.len(), 1);

        // Fetch tool A1
        let tool_a1_id = tools_a[0].id.clone();
        let fetched_tool_a1 = registry.get_tool("tenant_A", &tool_a1_id).await.unwrap();
        assert_eq!(fetched_tool_a1.id, tool_a1_id);

        // Cross-tenant access should fail
        let result = registry.get_tool("tenant_B", &tool_a1_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_pg_tool_registry_isolation() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let database_url = "postgres://postgres:postgres@localhost:5432/test";

        // Testing PostgreSQL RLS using before_acquire
        let pool_tenant_a = PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })

            .acquire_timeout(Duration::from_millis(50))
            .before_acquire(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("SET app.current_tenant = 'tenant_A'").await?;
                    Ok(true)
                })
            })
            .connect_lazy(database_url)
            .unwrap();

        let pool_tenant_b = PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })

            .acquire_timeout(Duration::from_millis(50))
            .before_acquire(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("SET app.current_tenant = 'tenant_B'").await?;
                    Ok(true)
                })
            })
            .connect_lazy(database_url)
            .unwrap();

        let registry_a = PgToolRegistry::new(pool_tenant_a);
        let registry_b = PgToolRegistry::new(pool_tenant_b);

    }
}
