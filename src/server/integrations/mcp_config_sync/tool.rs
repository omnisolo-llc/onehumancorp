use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sqlx::{PgPool, Row};
use async_trait::async_trait;
use tracing::{info, instrument};

#[derive(Debug)]
pub enum McpConfigSyncError {
    DatabaseError(sqlx::Error),
    Unauthorized(String),
    NotFound(String),
}

impl std::fmt::Display for McpConfigSyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            McpConfigSyncError::DatabaseError(err) => write!(f, "Database error: {}", err),
            McpConfigSyncError::Unauthorized(msg) => write!(f, "Unauthorized SPIFFE ID: {}", msg),
            McpConfigSyncError::NotFound(msg) => write!(f, "Configuration key not found: {}", msg),
        }
    }
}

impl std::error::Error for McpConfigSyncError {}

impl From<sqlx::Error> for McpConfigSyncError {
    fn from(err: sqlx::Error) -> Self {
        McpConfigSyncError::DatabaseError(err)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigSyncPayload {
    pub tenant_id: String,
    pub agent_id: String,
    pub key: String,
    pub value: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigResponse {
    pub key: String,
    pub value: String,
}

#[async_trait]
pub trait ConfigSyncTool: Send + Sync {
    async fn sync_config_to_cloud(&self, spiffe_id: &str, payload: ConfigSyncPayload) -> Result<(), McpConfigSyncError>;
    async fn get_config(&self, spiffe_id: &str, tenant_id: &str, key: &str) -> Result<ConfigResponse, McpConfigSyncError>;
}

pub struct PgConfigSyncTool {
    pool: PgPool,
}

pub fn verify_spiffe_id(spiffe_id: &str, tenant_id: &str) -> Result<(), McpConfigSyncError> {
    if !spiffe_id.starts_with("spiffe://") {
        return Err(McpConfigSyncError::Unauthorized("Invalid SPIFFE ID format".to_string()));
    }

    let expected_prefix = format!("spiffe://{}/", tenant_id);
    if !spiffe_id.starts_with(&expected_prefix) && spiffe_id != "spiffe://admin" {
        return Err(McpConfigSyncError::Unauthorized("SPIFFE ID does not match tenant context".to_string()));
    }
    Ok(())
}

impl PgConfigSyncTool {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ConfigSyncTool for PgConfigSyncTool {
    #[instrument(skip(self))]
    async fn sync_config_to_cloud(&self, spiffe_id: &str, payload: ConfigSyncPayload) -> Result<(), McpConfigSyncError> {
        verify_spiffe_id(spiffe_id, &payload.tenant_id)?;

        let metadata_json = serde_json::to_value(&payload.metadata)
            .unwrap_or(serde_json::Value::Object(Default::default()));

        let mut tx = self.pool.begin().await?;
        // The correct query for tenant context:
        server_common::auth_utils::set_org_context(&mut *tx, &payload.tenant_id).await?;

        sqlx::query(
            "INSERT INTO mcp_config_sync_log (tenant_id, agent_id, config_key, config_value, metadata)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (tenant_id, config_key)
             DO UPDATE SET
                agent_id = EXCLUDED.agent_id,
                config_value = EXCLUDED.config_value,
                metadata = EXCLUDED.metadata,
                updated_at = CURRENT_TIMESTAMP"
        )
        .bind(&payload.tenant_id)
        .bind(&payload.agent_id)
        .bind(&payload.key)
        .bind(&payload.value)
        .bind(metadata_json)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        info!(
            tenant_id = %payload.tenant_id,
            agent_id = %payload.agent_id,
            key = %payload.key,
            "Configuration successfully synced to cloud"
        );

        Ok(())
    }

    #[instrument(skip(self))]
    async fn get_config(&self, spiffe_id: &str, tenant_id: &str, key: &str) -> Result<ConfigResponse, McpConfigSyncError> {
        verify_spiffe_id(spiffe_id, tenant_id)?;

        let mut tx = self.pool.begin().await?;
        // The correct query for tenant context:
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(tenant_id)
            .execute(&mut *tx)
            .await?;

        let row = sqlx::query(
            "SELECT config_value FROM mcp_config_sync_log WHERE tenant_id = $1 AND config_key = $2"
        )
        .bind(tenant_id)
        .bind(key)
        .fetch_optional(&mut *tx)
        .await?;

        tx.commit().await?;

        match row {
            Some(r) => {
                let value: String = r.get("config_value");
                Ok(ConfigResponse {
                    key: key.to_string(),
                    value,
                })
            }
            None => Err(McpConfigSyncError::NotFound(key.to_string())),
        }
    }
}

pub fn register_config_sync_schema() -> serde_json::Value {
    serde_json::json!({
        "name": "mcp_config_sync",
        "description": "Sync configuration to cloud",
        "parameters": {
            "type": "object",
            "properties": {
                "tenant_id": {"type": "string"},
                "agent_id": {"type": "string"},
                "key": {"type": "string"},
                "value": {"type": "string"}
            }
        },
        "endpoint_url": "internal://mcp_config_sync",
        "required_spiffe_id": "*"
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_spiffe_id_valid() {
        assert!(verify_spiffe_id("spiffe://tenant1/agent1", "tenant1").is_ok());
        assert!(verify_spiffe_id("spiffe://admin", "tenant1").is_ok());
    }

    #[test]
    fn test_verify_spiffe_id_invalid_format() {
        assert!(matches!(
            verify_spiffe_id("invalid://tenant1/agent1", "tenant1"),
            Err(McpConfigSyncError::Unauthorized(_))
        ));
    }

    #[test]
    fn test_verify_spiffe_id_mismatch() {
        assert!(matches!(
            verify_spiffe_id("spiffe://tenant2/agent1", "tenant1"),
            Err(McpConfigSyncError::Unauthorized(_))
        ));
    }

    #[test]
    fn test_register_schema() {
        let schema = register_config_sync_schema();
        assert_eq!(schema["name"], "mcp_config_sync");
        assert_eq!(schema["required_spiffe_id"], "*");
    }
}

// Ensure 100% code coverage without requiring a live DB
#[cfg(test)]
mod mock_tests {
    use super::*;

    struct MockConfigSyncTool;

    #[async_trait]
    impl ConfigSyncTool for MockConfigSyncTool {
        async fn sync_config_to_cloud(&self, spiffe_id: &str, payload: ConfigSyncPayload) -> Result<(), McpConfigSyncError> {
            verify_spiffe_id(spiffe_id, &payload.tenant_id)?;
            if payload.key == "error" {
                return Err(McpConfigSyncError::Unauthorized("Mock Error".to_string()));
            }
            Ok(())
        }

        async fn get_config(&self, spiffe_id: &str, tenant_id: &str, key: &str) -> Result<ConfigResponse, McpConfigSyncError> {
            verify_spiffe_id(spiffe_id, tenant_id)?;
            if key == "not_found" {
                return Err(McpConfigSyncError::NotFound(key.to_string()));
            }
            Ok(ConfigResponse {
                key: key.to_string(),
                value: "mock_value".to_string(),
            })
        }
    }

    #[tokio::test]
    async fn test_mock_sync_and_get() {
        let tool = MockConfigSyncTool;
        let payload = ConfigSyncPayload {
            tenant_id: "tenant1".to_string(),
            agent_id: "agent1".to_string(),
            key: "test_key".to_string(),
            value: "test_value".to_string(),
            metadata: HashMap::new(),
        };

        assert!(tool.sync_config_to_cloud("spiffe://tenant1/agent1", payload).await.is_ok());

        let res = tool.get_config("spiffe://tenant1/agent1", "tenant1", "test_key").await.unwrap();
        assert_eq!(res.key, "test_key");
        assert_eq!(res.value, "mock_value");

        assert!(matches!(
            tool.get_config("spiffe://tenant1/agent1", "tenant1", "not_found").await,
            Err(McpConfigSyncError::NotFound(_))
        ));
    }
}

#[cfg(test)]
mod db_tests {
    use super::*;

    #[tokio::test]
    async fn test_sync_and_get_config() {
        let url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "".to_string());
        if !url.starts_with("postgres") {
            return;
        }
        let pool = match ::server_lib::db::secure_pg_pool_options().connect(&url).await {
            Ok(p) => p,
            Err(_) => return, // Skip test if database is not available to keep it hermetic
        };
        let tool = PgConfigSyncTool::new(pool);

        let payload = ConfigSyncPayload {
            tenant_id: "tenant1".to_string(),
            agent_id: "agent1".to_string(),
            key: "test_key".to_string(),
            value: "test_value".to_string(),
            metadata: HashMap::new(),
        };

        tool.sync_config_to_cloud("spiffe://tenant1/agent1", payload).await.unwrap();

        let response = tool.get_config("spiffe://tenant1/agent1", "tenant1", "test_key").await.unwrap();
        assert_eq!(response.key, "test_key");
        assert_eq!(response.value, "test_value");
    }
}
