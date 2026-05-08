use sqlx::{Executor, Postgres, query};

pub async fn set_org_context<'a, E>(executor: E, org_id: &str) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    let is_multitenant = crate::config::get().multitenant;

    if is_multitenant && org_id == "system" {
        return Err(sqlx::Error::Configuration("CRITICAL SECURITY ERROR: 'system' org_id is not allowed for tenant context in multitenant mode. Use explicit system functions.".into()));
    }

    // Only allow bypass if NOT multitenant and (org_id is system or empty).
    if !is_multitenant && (org_id == "system" || org_id.is_empty()) {
         query("SET LOCAL ROLE ohc_bypassrls")
             .execute(executor)
             .await?;
    } else {
         query("SELECT set_config('app.current_tenant', $1, true)")
             .bind(org_id)
             .execute(executor)
             .await?;
    }
    Ok(())
}

pub async fn set_system_context<'a, E>(executor: E) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    query("SET LOCAL ROLE ohc_bypassrls").execute(executor).await?;
    Ok(())
}
