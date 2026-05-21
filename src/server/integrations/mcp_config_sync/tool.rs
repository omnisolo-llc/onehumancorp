use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sqlx::PgPool;
use sqlx::Row;
use sqlx::Executor;
use opentelemetry::{global, trace::Tracer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSyncPayload {
    pub tenant_id: String,
    pub agent_id: String,
    pub key: String,
    pub value: String,
    pub metadata: HashMap<String, String>,
}

// Define a trait so we can mock the database interaction in tests
#[async_trait::async_trait]
pub trait ConfigSyncDatabase {
    async fn get_config_from_db(&self, tenant_id: &str, key: &str) -> Result<Option<String>, String>;
    async fn sync_config_to_db(&self, tenant_id: &str, agent_id: &str, key: &str, value: &str, metadata: &serde_json::Value) -> Result<(), String>;
}

pub struct PgConfigSyncDatabase {
    pool: PgPool,
}

impl PgConfigSyncDatabase {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ConfigSyncDatabase for PgConfigSyncDatabase {
    async fn get_config_from_db(&self, tenant_id: &str, key: &str) -> Result<Option<String>, String> {
        let tenant_uuid = uuid::Uuid::parse_str(tenant_id).map_err(|e| e.to_string())?;

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let set_tenant_query = format!("SET LOCAL app.current_tenant = '{}'", tenant_uuid);
        tx.execute(set_tenant_query.as_str()).await.map_err(|e| e.to_string())?;

        let record = sqlx::query(
            r#"
            SELECT value FROM mcp_config_sync_log
            WHERE tenant_id = $1::uuid AND key = $2
            ORDER BY created_at DESC LIMIT 1
            "#)
        .bind(tenant_uuid)
        .bind(key)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(record.map(|r| r.get("value")))
    }

    async fn sync_config_to_db(&self, tenant_id: &str, agent_id: &str, key: &str, value: &str, metadata: &serde_json::Value) -> Result<(), String> {
        let tenant_uuid = uuid::Uuid::parse_str(tenant_id).map_err(|e| e.to_string())?;

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let set_tenant_query = format!("SET LOCAL app.current_tenant = '{}'", tenant_uuid);
        tx.execute(set_tenant_query.as_str()).await.map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            INSERT INTO mcp_config_sync_log (tenant_id, agent_id, key, value, metadata)
            VALUES ($1::uuid, $2, $3, $4, $5)
            "#)
        .bind(tenant_uuid)
        .bind(agent_id)
        .bind(key)
        .bind(value)
        .bind(metadata)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }
}

pub struct HybridConfigSyncMcpTool<T: ConfigSyncDatabase> {
    db: T,
}

impl<T: ConfigSyncDatabase> HybridConfigSyncMcpTool<T> {
    pub fn new(db: T) -> Self {
        Self { db }
    }

    pub async fn get_config(&self, payload: &ConfigSyncPayload, spiffe_id: &str) -> Result<String, String> {
        let tracer = global::tracer("mcp_config_sync");
        let _span = tracer.start("get_config");

        if payload.key.is_empty() {
            return Err("key is required".to_string());
        }

        if payload.tenant_id.is_empty() {
             return Err("tenant_id is required".to_string());
        }

        if !spiffe_id.contains(&payload.tenant_id) && spiffe_id != "*" {
            return Err("Unauthorized SPIFFE ID for this tenant".to_string());
        }

        let record = self.db.get_config_from_db(&payload.tenant_id, &payload.key).await?;

        match record {
            Some(value) => Ok(value),
            None => Err("Config not found".to_string()),
        }
    }

    pub async fn sync_config_to_cloud(&self, payload: &ConfigSyncPayload, spiffe_id: &str) -> Result<(), String> {
        let tracer = global::tracer("mcp_config_sync");
        let _span = tracer.start("sync_config_to_cloud");

        if payload.tenant_id.is_empty() {
            return Err("tenant_id is required".to_string());
        }
        if payload.key.is_empty() {
            return Err("key is required".to_string());
        }

        if !spiffe_id.contains(&payload.tenant_id) && spiffe_id != "*" {
            return Err("Unauthorized SPIFFE ID for this tenant".to_string());
        }

        let metadata_json = serde_json::to_value(&payload.metadata).map_err(|e| e.to_string())?;

        self.db.sync_config_to_db(&payload.tenant_id, &payload.agent_id, &payload.key, &payload.value, &metadata_json).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use std::sync::Arc;

    struct MockConfigSyncDatabase {
        pub configs: Arc<Mutex<HashMap<String, String>>>,
    }

    #[async_trait::async_trait]
    impl ConfigSyncDatabase for MockConfigSyncDatabase {
        async fn get_config_from_db(&self, tenant_id: &str, key: &str) -> Result<Option<String>, String> {
            let combined_key = format!("{}:{}", tenant_id, key);
            let map = self.configs.lock().unwrap();
            Ok(map.get(&combined_key).cloned())
        }

        async fn sync_config_to_db(&self, tenant_id: &str, _agent_id: &str, key: &str, value: &str, _metadata: &serde_json::Value) -> Result<(), String> {
            let combined_key = format!("{}:{}", tenant_id, key);
            let mut map = self.configs.lock().unwrap();
            map.insert(combined_key, value.to_string());
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_get_config_missing_key() {
        let db = MockConfigSyncDatabase { configs: Arc::new(Mutex::new(HashMap::new())) };
        let tool = HybridConfigSyncMcpTool::new(db);
        let payload = ConfigSyncPayload {
            tenant_id: "00000000-0000-0000-0000-000000000000".to_string(),
            agent_id: "a1".to_string(),
            key: "".to_string(),
            value: "".to_string(),
            metadata: HashMap::new(),
        };

        let result = tool.get_config(&payload, "spiffe://example.org/agent-1").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "key is required");
    }

    #[tokio::test]
    async fn test_get_config_missing_tenant() {
        let db = MockConfigSyncDatabase { configs: Arc::new(Mutex::new(HashMap::new())) };
        let tool = HybridConfigSyncMcpTool::new(db);
        let payload = ConfigSyncPayload {
            tenant_id: "".to_string(),
            agent_id: "a1".to_string(),
            key: "db_url".to_string(),
            value: "".to_string(),
            metadata: HashMap::new(),
        };

        let result = tool.get_config(&payload, "spiffe://example.org/agent-1").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "tenant_id is required");
    }

    #[tokio::test]
    async fn test_get_config_unauthorized_spiffe() {
        let db = MockConfigSyncDatabase { configs: Arc::new(Mutex::new(HashMap::new())) };
        let tool = HybridConfigSyncMcpTool::new(db);
        let payload = ConfigSyncPayload {
            tenant_id: "00000000-0000-0000-0000-000000000000".to_string(),
            agent_id: "a1".to_string(),
            key: "db_url".to_string(),
            value: "".to_string(),
            metadata: HashMap::new(),
        };

        let result = tool.get_config(&payload, "spiffe://example.org/other-tenant/agent-1").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unauthorized SPIFFE ID for this tenant");
    }

    #[tokio::test]
    async fn test_sync_config_to_cloud_missing_tenant() {
        let db = MockConfigSyncDatabase { configs: Arc::new(Mutex::new(HashMap::new())) };
        let tool = HybridConfigSyncMcpTool::new(db);
        let payload = ConfigSyncPayload {
            tenant_id: "".to_string(),
            agent_id: "a1".to_string(),
            key: "db_url".to_string(),
            value: "postgres://...".to_string(),
            metadata: HashMap::new(),
        };

        let result = tool.sync_config_to_cloud(&payload, "spiffe://example.org/agent-1").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "tenant_id is required");
    }

    #[tokio::test]
    async fn test_sync_config_to_cloud_missing_key() {
        let db = MockConfigSyncDatabase { configs: Arc::new(Mutex::new(HashMap::new())) };
        let tool = HybridConfigSyncMcpTool::new(db);
        let payload = ConfigSyncPayload {
            tenant_id: "00000000-0000-0000-0000-000000000000".to_string(),
            agent_id: "a1".to_string(),
            key: "".to_string(),
            value: "postgres://...".to_string(),
            metadata: HashMap::new(),
        };

        let result = tool.sync_config_to_cloud(&payload, "spiffe://example.org/agent-1").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "key is required");
    }

    #[tokio::test]
    async fn test_sync_config_to_cloud_unauthorized_spiffe() {
        let db = MockConfigSyncDatabase { configs: Arc::new(Mutex::new(HashMap::new())) };
        let tool = HybridConfigSyncMcpTool::new(db);
        let payload = ConfigSyncPayload {
            tenant_id: "00000000-0000-0000-0000-000000000000".to_string(),
            agent_id: "a1".to_string(),
            key: "db_url".to_string(),
            value: "postgres://...".to_string(),
            metadata: HashMap::new(),
        };

        let result = tool.sync_config_to_cloud(&payload, "spiffe://example.org/other-tenant/agent-1").await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unauthorized SPIFFE ID for this tenant");
    }

    #[tokio::test]
    async fn test_happy_path() {
        let db = MockConfigSyncDatabase { configs: Arc::new(Mutex::new(HashMap::new())) };
        let tool = HybridConfigSyncMcpTool::new(db);
        let payload = ConfigSyncPayload {
            tenant_id: "00000000-0000-0000-0000-000000000000".to_string(),
            agent_id: "a1".to_string(),
            key: "db_url".to_string(),
            value: "postgres://...".to_string(),
            metadata: HashMap::new(),
        };

        // Initially not found
        let get_res = tool.get_config(&payload, "*").await;
        assert!(get_res.is_err());
        assert_eq!(get_res.unwrap_err(), "Config not found");

        // Sync
        let sync_res = tool.sync_config_to_cloud(&payload, "*").await;
        assert!(sync_res.is_ok());

        // Now it should be found
        let get_res_2 = tool.get_config(&payload, "*").await;
        assert!(get_res_2.is_ok());
        assert_eq!(get_res_2.unwrap(), "postgres://...");
    }
}
