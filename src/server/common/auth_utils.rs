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
    let org_id = org_id.trim();
    if org_id == "system" {
        if ::server_config::get().multitenant {
            return Err(sqlx::Error::Configuration(
                "tenant_id 'system' cannot be queried in multi-tenant mode".into(),
            ));
        }
        // Elevate privileges for system-level queries using a single command.
        // SET LOCAL is transaction-scoped and safely handles RLS bypass for the ohc_bypassrls role.
        query("SET LOCAL ROLE ohc_bypassrls")
            .execute(executor)
            .await?;
    } else {
        if org_id.is_empty() && ::server_config::get().multitenant {
            return Err(sqlx::Error::Configuration(
                "empty tenant_id is not allowed in multi-tenant mode".into(),
            ));
        }
        // Use set_config to set the GUC variable for RLS policies.
        // The third parameter 'true' makes it local to the current transaction.
        query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(org_id)
            .execute(executor)
            .await?;
    }
    Ok(())
}
