use sqlx::{Executor, Postgres, query};

pub async fn set_org_context<'a, E>(executor: E, org_id: &str) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    if org_id == "system" {
        if ::server_config::get().multitenant {
            return Err(sqlx::Error::Protocol("CRITICAL SECURITY ERROR: System bypass is strictly forbidden in Cloud Mode to prevent tenant data leakage.".to_string()));
        }
        // Elevate privileges for system-level queries in Standalone mode ONLY.
        query("SET LOCAL ROLE ohc_bypassrls")
            .execute(executor)
            .await?;
    } else {
        // No need to RESET ROLE since SET LOCAL is transaction scoped.
        query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(org_id)
            .execute(executor)
            .await?;
    }
    Ok(())
}
