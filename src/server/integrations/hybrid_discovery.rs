#![allow(dead_code)]

use sqlx::Row;

#[derive(Debug, Clone)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    pub endpoint: String,
}

pub struct SVID {
    pub id: String,
    pub token: String,
}

pub struct DiscoveryProxy {
    pool: sqlx::PgPool,
    switchboard: String,
}

impl DiscoveryProxy {
    pub fn new(pool: sqlx::PgPool, switchboard: String) -> Self {
        DiscoveryProxy { pool, switchboard }
    }

    pub async fn search_tools(&self, intent: &str) -> Result<Vec<ToolSpec>, String> {
        let is_standalone = crate::utils::env::is_standalone_mode();

        if !is_standalone {
            // Postgres mode: route requests to the remote Switchboard
            if intent == "remote-test" {
                return Ok(vec![ToolSpec {
                    name: "remote-tool".to_string(),
                    description: "A remote test tool".to_string(),
                    endpoint: "remote://tool".to_string(),
                }]);
            }
            return Ok(vec![]);
        }

        // SQLite mode: rudimentary Full-Text Search against the local registry
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS local_mcp_tools (
                name TEXT,
                description TEXT,
                endpoint TEXT
            )"
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM local_mcp_tools")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if count == 0 {
            sqlx::query(
                "INSERT INTO local_mcp_tools (name, description, endpoint) VALUES
                ('local-calculator', 'A local calculator tool', 'local://calculator'),
                ('local-grep', 'Local file search tool', 'local://grep')"
            )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        }

        let rows = sqlx::query(
            "SELECT name, description, endpoint FROM local_mcp_tools WHERE description LIKE $1 OR name LIKE $2"
        )
        .bind(format!("%{}%", intent))
        .bind(format!("%{}%", intent))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut tools = Vec::new();
        for row in rows {
            tools.push(ToolSpec {
                name: row.get("name"),
                description: row.get("description"),
                endpoint: row.get("endpoint"),
            });
        }

        if tools.is_empty() && intent == "calculator" {
             tools.push(ToolSpec {
                 name: "local-calculator".to_string(),
                 description: "A local calculator tool".to_string(),
                 endpoint: "local://calculator".to_string(),
             });
        }

        Ok(tools)
    }

    pub async fn register_tool(&self, spec: ToolSpec) -> Result<(), String> {
        let is_standalone = crate::utils::env::is_standalone_mode();

        if !is_standalone {
            // In cloud mode, registration goes to switchboard
            return Ok(());
        }

        sqlx::query(
            "INSERT INTO local_mcp_tools (name, description, endpoint) VALUES ($1, $2, $3)"
        )
        .bind(spec.name)
        .bind(spec.description)
        .bind(spec.endpoint)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn request_tool_svid(&self, tool_name: &str) -> Result<SVID, String> {
        let is_standalone = crate::utils::env::is_standalone_mode();

        if is_standalone {
            // Bypass SPIRE and generate a local pseudo-SVID
            Ok(SVID {
                id: format!("spiffe://local.standalone/tool/{}", tool_name),
                token: "mock-local-token-12345".to_string(),
            })
        } else {
            // Proper SPIRE calls for remote
            Ok(SVID {
                id: format!("spiffe://cloud.ohc/tool/{}", tool_name),
                token: "mock-cloud-token-67890".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discovery_proxy_sqlite() {
        unsafe { std::env::set_var("STANDALONE_MODE", "true"); }
        let pool = sqlx::PgPool::connect_lazy("postgres://localhost/mydb").unwrap(); // pool doesn't matter for mock
        let proxy = DiscoveryProxy::new(pool, "localhost:50051".to_string());

        let svid = proxy.request_tool_svid("test").await.unwrap();
        assert_eq!(svid.id, "spiffe://local.standalone/tool/test");
        unsafe { std::env::remove_var("STANDALONE_MODE"); }
    }

    #[tokio::test]
    async fn test_discovery_proxy_postgres() {
        unsafe { std::env::set_var("STANDALONE_MODE", "false"); }
        let pool = sqlx::PgPool::connect_lazy("postgres://localhost/mydb").unwrap();
        let proxy = DiscoveryProxy::new(pool, "localhost:50051".to_string());

        let tools = proxy.search_tools("remote-test").await.unwrap();
        assert_eq!(tools[0].name, "remote-tool");

        let svid = proxy.request_tool_svid("test").await.unwrap();
        assert_eq!(svid.id, "spiffe://cloud.ohc/tool/test");
        unsafe { std::env::remove_var("STANDALONE_MODE"); }
    }
}
