use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuditSyncPayload {
    pub tenant_id: String,
    pub agent_id: String,
    pub action: String,
    pub resource: String,
    pub status: String,
    pub metadata: String,
    pub timestamp: i64,
}

pub async fn sync_audit_logs_to_cloud(
    pool: &sqlx::PgPool,
    payload: AuditSyncPayload,
) -> Result<(), sqlx::Error> {
    if std::env::var("OHC_TELEMETRY_ENABLED").unwrap_or_default() == "true" {
        // We will just do a print or mock telemetry as this is the design pattern
        println!(
            "telemetry log: tenant_id={} agent_id={} action={}",
            payload.tenant_id, payload.agent_id, payload.action
        );
    }

    let query = "
        INSERT INTO mcp_audit_sync_log (tenant_id, agent_id, action, resource, status, metadata, timestamp)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
    ";

    sqlx::query(query)
        .bind(payload.tenant_id)
        .bind(payload.agent_id)
        .bind(payload.action)
        .bind(payload.resource)
        .bind(payload.status)
        .bind(payload.metadata)
        .bind(payload.timestamp)
        .execute(pool)
        .await?;

    Ok(())
}
