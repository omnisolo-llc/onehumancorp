use sqlx::PgPool;
use serde_json::Value;

pub async fn handle_incident_resolution(tenant_id: &str, payload: &Value, pool: &PgPool) -> Result<(), sqlx::Error> {
    if let Some(incident_id) = payload.get("incident_id").and_then(|v| v.as_str()) {
        sqlx::query("UPDATE incidents SET status = 'RESOLVED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
            .bind(incident_id)
            .bind(tenant_id)
            .execute(pool)
            .await?;
    }
    Ok(())
}
