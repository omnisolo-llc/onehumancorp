use sqlx::PgPool;

pub async fn sync_legacy_approval_status(tenant_id: &str, id: &str, state: &str, pool: &PgPool) -> Result<(), sqlx::Error> {
    if state == "APPROVED" || state == "REJECTED" || state == "DISMISSED" {
        let legacy_status = if state == "APPROVED" { "APPROVED" } else { "REJECTED" };
        sqlx::query("UPDATE agent_approvals SET status = $1 WHERE id = $2 AND tenant_id = $3")
            .bind(legacy_status)
            .bind(id)
            .bind(tenant_id)
            .execute(pool)
            .await?;
    }
    Ok(())
}
