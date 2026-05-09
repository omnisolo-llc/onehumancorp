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
    // No need to RESET ROLE since SET LOCAL is transaction scoped.
    query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(org_id)
        .execute(executor)
        .await?;
    Ok(())
}
