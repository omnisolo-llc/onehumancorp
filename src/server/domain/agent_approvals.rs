use sqlx::PgPool;

pub async fn sync_legacy_approval_status(
    tenant_id: &str,
    id: &str,
    state: &str,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    if state == "APPROVED" || state == "REJECTED" || state == "DISMISSED" {
        let legacy_status = if state == "APPROVED" {
            "APPROVED"
        } else {
            "REJECTED"
        };
        let rows_affected =
            sqlx::query("UPDATE agent_approvals SET status = $1 WHERE id = $2 AND tenant_id = $3")
                .bind(legacy_status)
                .bind(id)
                .bind(tenant_id)
                .execute(pool)
                .await?
                .rows_affected();

        if rows_affected == 0 {
            let request_status = if state == "APPROVED" {
                "Approved"
            } else {
                "Rejected"
            };
            sqlx::query(
                "UPDATE agent_action_requests SET status = $1 WHERE id = $2 AND tenant_id = $3",
            )
            .bind(request_status)
            .bind(id)
            .bind(tenant_id)
            .execute(pool)
            .await?;
        }
    }
    Ok(())
}
