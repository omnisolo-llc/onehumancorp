use sqlx::PgPool;

pub struct IncidentsHandler;

impl IncidentsHandler {
    pub async fn handle_incident_resolution(
        pool: &PgPool,
        tenant_id: &str,
        incident_id: &str,
    ) -> Result<(), String> {
        let _ = sqlx::query("UPDATE incidents SET status = 'RESOLVED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
            .bind(incident_id)
            .bind(tenant_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
