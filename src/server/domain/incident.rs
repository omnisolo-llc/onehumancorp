use sqlx::PgPool;

pub struct IncidentHandler;

impl IncidentHandler {
    pub async fn handle_resolution(pool: &PgPool, tenant_id: &str, incident_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _ = sqlx::query("UPDATE incidents SET status = 'RESOLVED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
            .bind(incident_id)
            .bind(tenant_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
