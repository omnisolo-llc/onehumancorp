use sqlx::{Executor, Postgres, query};

pub async fn set_org_context<'a, E>(executor: E, org_id: &str) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    if org_id == "system" {
        if !::server_config::get().multitenant {
            query("SET LOCAL ROLE ohc_bypassrls").execute(executor).await?;
        } else {
            // Strictly fail if user attempts to use "system" context in multitenant mode.
            // In cloud mode, no one should ever bypass tenant isolation via the "system" keyword.
            return Err(sqlx::Error::Configuration("CRITICAL SECURITY ERROR: System tenant bypass attempted in cloud multitenant mode. Access Denied.".into()));
        }
    } else {
        // No need to RESET ROLE since SET LOCAL is transaction scoped.
        query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(org_id)
            .execute(executor)
            .await?;
    }
    Ok(())
}
