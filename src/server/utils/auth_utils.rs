use sqlx::{Executor, Postgres, query};

pub async fn set_system_context<'a, E>(executor: E) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    // Elevate privileges for system-level queries securely.
    query("SET LOCAL ROLE ohc_bypassrls")
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn set_org_context<'a, E>(executor: E, org_id: &str) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    if org_id == "system" && crate::config::get().multitenant {
        // Explicitly block passing 'system' as an org_id in Cloud (multitenant) mode
        // to prevent Row-Level Security (RLS) bypass vulnerabilities.
        tracing::error!("CRITICAL SECURITY ERROR: Attempted to bypass RLS by passing invalid id in multitenant mode.");
        return Err(sqlx::Error::Configuration("RLS bypass blocked".into()));
    }

    if !crate::config::get().multitenant && org_id == "system" {
        // Elevate privileges for system-level queries in standalone mode for backward compat,
        // though `set_system_context` should be preferred.
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
