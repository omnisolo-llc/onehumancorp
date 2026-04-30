use sqlx::{Executor, Postgres};

pub async fn set_org_context<'a, E>(executor: E, org_id: &str) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    // Use set_config for safe parameter binding of session variables
    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(org_id)
        .execute(executor)
        .await?;
    Ok(())
}
