use sqlx::{Executor, Postgres, query};

pub async fn set_system_context<'a, E>(executor: E) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
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
        // Reject "system" unconditionally to prevent IDOR via set_org_context. System queries MUST use set_system_context.
        return Err(sqlx::Error::Configuration("tenant_id 'system' cannot be queried in multi-tenant mode via set_org_context".into()));
    } else {
        if org_id.trim().is_empty() && ::server_config::get().multitenant {
            return Err(sqlx::Error::Configuration("empty tenant_id is not allowed in multi-tenant mode".into()));
        }
        // No need to RESET ROLE since SET LOCAL is transaction scoped.
        query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(org_id)
            .execute(executor)
            .await?;
    }
    Ok(())
}
