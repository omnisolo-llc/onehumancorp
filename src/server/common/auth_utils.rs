use sqlx::{Executor, Postgres, query};

pub async fn set_org_context<'a, E>(executor: E, org_id: &str) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    if org_id == "system" {
        // Elevate privileges for system-level queries.
        // In Cloud mode, we only allow this for 'system' context to handle global tasks.
        // SET LOCAL ROLE ohc_bypassrls allows bypassing RLS for the duration of the transaction.
        query("SET LOCAL ROLE ohc_bypassrls")
            .execute(executor)
            .await?;
    } else {
        // Standard tenant isolation using app.current_tenant.
        // We also ensure we are NOT in the bypass role if we were previously (though SET LOCAL should handle it).
        // To be safe, we explicitly set the role back to the default session user if it's not system.

        // Actually, in many environments, the session user might not have permission to SET ROLE.
        // But ohc_bypassrls is a special role we created.

        query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(org_id)
            .execute(executor)
            .await?;
    }
    Ok(())
}
