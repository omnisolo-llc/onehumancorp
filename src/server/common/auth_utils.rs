use sqlx::{Postgres, Transaction, query};

pub async fn set_system_context<'a>(tx: &mut sqlx::Transaction<'a, Postgres>) -> Result<(), sqlx::Error>
{
    query("SET LOCAL ROLE ohc_bypassrls")
        .execute(&mut **tx)
        .await?;
    Ok(())
}

pub async fn set_org_context<'a>(tx: &mut sqlx::Transaction<'a, Postgres>, org_id: &str) -> Result<(), sqlx::Error>
{
    if org_id == "system" {
        if ::server_config::get().multitenant {
            return Err(sqlx::Error::Configuration("tenant_id 'system' cannot be queried in multi-tenant mode".into()));
        }
        query("SET LOCAL ROLE ohc_bypassrls")
            .execute(&mut **tx)
            .await?;
    } else {
        query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(org_id)
            .execute(&mut **tx)
            .await?;
    }
    Ok(())
}
