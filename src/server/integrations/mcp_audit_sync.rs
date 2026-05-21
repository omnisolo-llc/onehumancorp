use std::collections::HashMap;
use serde_json::Value;
use tokio::sync::RwLock;

pub struct AuditSyncPayload {
    pub tenant_id: String,
    pub agent_id: String,
    pub action: String,
    pub resource: String,
    pub status: String,
    pub metadata: String,
    pub timestamp: i64,
}

pub struct AuditLogger {
    pool: sqlx::PgPool,
}

impl AuditLogger {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub fn name(&self) -> &str {
        "sync_audit_logs_to_cloud"
    }

    pub fn description(&self) -> &str {
        "Synchronizes local agent audit logs to the Enterprise Cloud database."
    }

    pub async fn execute(
        &self,
        params: HashMap<String, Value>,
        spiffe_id: &str,
    ) -> Result<HashMap<String, String>, String> {
        let tenant_id = params.get("tenant_id").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let agent_id = params.get("agent_id").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let action = params.get("action").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let resource = params.get("resource").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let status = params.get("status").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let metadata = params.get("metadata").and_then(|v| v.as_str()).unwrap_or_default().to_string();

        let timestamp = params.get("timestamp")
            .and_then(|v| v.as_f64())
            .map(|t| t as i64)
            .unwrap_or_else(|| chrono::Utc::now().timestamp());

        if tenant_id.is_empty() || agent_id.is_empty() || action.is_empty() {
            return Err("missing required fields in payload".to_string());
        }

        let expected_spiffe_prefix = format!("spiffe://onehumancorp.io/tenant/{}/agent/{}", tenant_id, agent_id);
        if spiffe_id != expected_spiffe_prefix && spiffe_id != "spiffe://onehumancorp.io/admin" {
            return Err(format!(
                "unauthorized: SPIFFE ID {} does not match expected prefix {}",
                spiffe_id, expected_spiffe_prefix
            ));
        }

        let query = "
            INSERT INTO mcp_audit_sync_log (tenant_id, agent_id, action, resource, status, metadata, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
        ";

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        sqlx::query(query)
            .bind(&tenant_id)
            .bind(&agent_id)
            .bind(&action)
            .bind(&resource)
            .bind(&status)
            .bind(&metadata)
            .bind(timestamp)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        if std::env::var("OHC_TELEMETRY_ENABLED").unwrap_or_default() == "true" {
            tracing::info!(
                target: "audit_log",
                tenant_id = %tenant_id,
                agent_id = %agent_id,
                action = %action,
                resource = %resource,
                "Synced audit log to cloud"
            );
        }

        let mut result = HashMap::new();
        result.insert("status".to_string(), "success".to_string());
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sync_audit_logs_unauthorized() {
        // Just test the error boundary
        let pool_opts = sqlx::postgres::PgPoolOptions::new().max_connections(1);
        let pool = pool_opts.connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap();
        let logger = AuditLogger::new(pool);

        let mut params = HashMap::new();
        params.insert("tenant_id".to_string(), Value::String("tenant-1".to_string()));
        params.insert("agent_id".to_string(), Value::String("agent-1".to_string()));
        params.insert("action".to_string(), Value::String("read".to_string()));

        let res = logger.execute(params, "spiffe://onehumancorp.io/tenant/tenant-2/agent/agent-1").await;
        assert!(res.is_err());
    }
}
