use sqlx::{Executor, Postgres, query};
use tracing::warn;

pub async fn set_system_context<'a, E>(executor: E) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    warn!("AUDIT: set_system_context called - elevating privileges to ohc_bypassrls");
    query("SET LOCAL ROLE ohc_bypassrls")
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn set_org_context<'a, E>(executor: E, org_id: &str) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    if org_id.trim() == "system" {
        warn!("AUDIT: rejected attempt to use 'system' as tenant_id in set_org_context");
        return Err(sqlx::Error::Configuration("tenant_id 'system' cannot be used via set_org_context; use set_system_context instead".into()));
    }
    if org_id.trim().is_empty() && ::server_config::get().multitenant {
        return Err(sqlx::Error::Configuration("empty tenant_id is not allowed in multi-tenant mode".into()));
    }
    // No need to RESET ROLE since SET LOCAL is transaction scoped.
    query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(org_id)
        .execute(executor)
        .await?;
    Ok(())
}
