
use serde_json::Value;
use sqlx::PgPool;

pub async fn handle_incident_resolution(tenant_id: String, payload: Value, pool: PgPool) -> Result<(), String> {
    if let Some(incident_id) = payload.get("incident_id").and_then(|v| v.as_str()) {
        sqlx::query("UPDATE incidents SET status = 'RESOLVED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
            .bind(incident_id)
            .bind(&tenant_id)
            .execute(&pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Missing incident_id".to_string())
    }
}
