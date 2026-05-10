use sqlx::{Executor, Postgres, query};

pub async fn set_org_context<'a, E>(executor: E, org_id: &str) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    if org_id == "system" {
        if crate::config::get().multitenant {
            return Err(sqlx::Error::Protocol("System bypass rejected in multitenant mode".into()));
        }
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
